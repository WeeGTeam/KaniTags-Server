use crate::models::image::ImageRow;
use crate::schema::collection_image::dsl as collection_image_dsl;
use crate::schema::image::dsl as image_dsl;
use crate::schema::image::dsl::image;
use crate::schema::image_tag::dsl as image_tag_dsl;
use crate::schema::user_image::dsl as user_image_dsl;
use crate::schema::user_image::dsl::user_image;
use diesel::expression::expression_types::NotSelectable;
use diesel::pg::Pg;
use diesel::{AggregateExpressionMethods, BoxableExpression, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};
use kani_domain_api_model::collection::CollectionId;
use kani_domain_api_model::image_search::{ImageSearchFilter, Layout, SortOption, SortOrder};
use kani_domain_api_model::tag::TagId;

type ImageQuery<'a> = crate::schema::image::BoxedQuery<'a, Pg>;

pub struct ImageSearchQueryBuilder<'a> {
    query: ImageQuery<'a>,
}

impl<'a> ImageSearchQueryBuilder<'a> {

    pub fn for_user(user_id: i64) -> Self {
        let subquery = user_image.select(user_image_dsl::image_id)
            .filter(user_image_dsl::user_id.eq(user_id));
        Self {
            query: image
                .filter(image_dsl::id.eq_any(subquery))
                .into_boxed(),
        }
    }

    pub fn with_dimensions(mut self, filter: &ImageSearchFilter) -> Self {
        if let Some(v) = filter.min_width  { self.query = self.query.filter(image_dsl::res_width.ge(v as i32)); }
        if let Some(v) = filter.max_width  { self.query = self.query.filter(image_dsl::res_width.le(v as i32)); }
        if let Some(v) = filter.min_height { self.query = self.query.filter(image_dsl::res_height.ge(v as i32)); }
        if let Some(v) = filter.max_height { self.query = self.query.filter(image_dsl::res_height.le(v as i32)); }
        self
    }

    pub fn with_layout(mut self, layout: Option<&Layout>) -> Self {
        if let Some(layout) = layout {
            self.query = match layout {
                Layout::Portrait => self.query.filter(image_dsl::res_width.lt(image_dsl::res_height)),
                Layout::Landscape => self.query.filter(image_dsl::res_width.gt(image_dsl::res_height)),
                Layout::Square => self.query.filter(diesel::dsl::sql::<diesel::sql_types::Bool>(
                    "res_width * 100 BETWEEN res_height * 95 AND res_height * 105"
                )),
            };
        }
        self
    }

    pub fn with_tags(mut self, tags: &[TagId]) -> Self {
        if tags.is_empty() { return self; }
        let tag_ids = tags.iter().map(|t| **t).collect::<Vec<i64>>();
        let n = tag_ids.len() as i64;
        let sub = image_tag_dsl::image_tag
            .filter(image_tag_dsl::tag_id.eq_any(tag_ids))
            .group_by(image_tag_dsl::image_id)
            .having(diesel::dsl::count(image_tag_dsl::tag_id).aggregate_distinct().eq(n))
            .select(image_tag_dsl::image_id);
        self.query = self.query.filter(image_dsl::id.eq_any(sub));
        self
    }

    pub fn excluding_tags(mut self, tags: &[TagId]) -> Self {
        if tags.is_empty() { return self; }
        let ids = tags.iter().map(|t| **t).collect::<Vec<i64>>();
        let sub = image_tag_dsl::image_tag
            .filter(image_tag_dsl::tag_id.eq_any(ids))
            .select(image_tag_dsl::image_id);
        self.query = self.query.filter(diesel::dsl::not(image_dsl::id.eq_any(sub)));
        self
    }

    pub fn in_collection(mut self, collection: Option<&CollectionId>) -> Self {
        if let Some(cid) = collection {
            let sub = collection_image_dsl::collection_image
                .filter(collection_image_dsl::collection_id.eq(cid.0))
                .select(collection_image_dsl::image_id);
            self.query = self.query.filter(image_dsl::id.eq_any(sub));
        }
        self
    }

    pub fn sorted_by(mut self, sort: &[SortOption]) -> Self {
        let mut ordered = false;
        for opt in sort {
            let col: Box<dyn BoxableExpression<_, Pg, SqlType = NotSelectable>> = match opt {
                SortOption::Id(SortOrder::Asc)          => Box::new(image_dsl::id.asc()),
                SortOption::Id(SortOrder::Desc)         => Box::new(image_dsl::id.desc()),
                SortOption::Date(SortOrder::Asc)        => Box::new(image_dsl::created_at.asc()),
                SortOption::Date(SortOrder::Desc)       => Box::new(image_dsl::created_at.desc()),
                SortOption::Resolution(SortOrder::Asc)  => Box::new(image_dsl::res_width.asc()),
                SortOption::Resolution(SortOrder::Desc) => Box::new(image_dsl::res_width.desc()),
            };
            if !ordered { self.query = self.query.order_by(col); ordered = true; }
            else        { self.query = self.query.then_order_by(col); }
        }
        self
    }

    pub fn load(self, conn: &mut PgConnection) -> QueryResult<Vec<ImageRow>> {
        self.query
            .select(ImageRow::as_select())
            .load(conn)
    }
}
