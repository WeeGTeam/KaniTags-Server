use crate::converter::TryToDomain;
use kani_domain_api_model::collection::CollectionId;
use kani_domain_api_model::image_search::ImageSearchFilter;
use kani_domain_api_model::tag::TagId;
use kani_openapi::models::GetImagesQueryParams;

#[derive(Debug, thiserror::Error)]
pub enum ImageSearchFilterConvertError {
    #[error("invalid collection id: {0}")]
    InvalidCollectionId(#[source] std::num::ParseIntError),
    #[error("invalid layout: {0}")]
    InvalidLayout(#[source] strum::ParseError),
    #[error("invalid tag id: {0}")]
    InvalidTagId(#[source] std::num::ParseIntError),
    #[error("invalid sort option: {0}")]
    InvalidSortOption(#[source] anyhow::Error),
}

impl TryToDomain<ImageSearchFilter> for &GetImagesQueryParams {
    type Error = ImageSearchFilterConvertError;

    fn try_to_domain(self) -> Result<ImageSearchFilter, Self::Error> {
        Ok(ImageSearchFilter {
            collection: self.collection.as_ref().map(|c| c.parse()).transpose()
                .map_err(|e| ImageSearchFilterConvertError::InvalidCollectionId(e))?
                .map(|i| CollectionId(i)),
            layout: self.layout.as_ref().map(|l| l.parse()).transpose()
                .map_err(|e| ImageSearchFilterConvertError::InvalidLayout(e))?,
            min_width: self.minw.map(|value| value as u32),
            max_width: self.maxw.map(|value| value as u32),
            min_height: self.minh.map(|value| value as u32),
            max_height: self.maxh.map(|value| value as u32),
            tags: self.tag.iter().map(|tag| tag.parse().and_then(|tag| Ok(TagId(tag)))).collect::<Result<Vec<_>, _>>()
                .map_err(|e| ImageSearchFilterConvertError::InvalidTagId(e))?,
            exclude_tags: self.etag.iter().map(|tag| tag.parse().and_then(|tag| Ok(TagId(tag)))).collect::<Result<Vec<_>, _>>()
                .map_err(|e| ImageSearchFilterConvertError::InvalidTagId(e))?,
            sort: self.sort.iter().map(|sort| sort.parse()).collect::<Result<Vec<_>, _>>()
                .map_err(|e| ImageSearchFilterConvertError::InvalidSortOption(e))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kani_domain_api_model::collection::CollectionId;
    use kani_domain_api_model::image_search::SortOrder::Desc;
    use kani_domain_api_model::image_search::{ImageSearchFilter, Layout, SortOption, SortOrder};
    use kani_domain_api_model::tag::TagId;

    #[test]
    fn test_convert_filter() {
        let params = GetImagesQueryParams {
            collection: Some("12345".to_owned()),
            layout: Some("portrait".to_owned()),
            minw: Some(100),
            maxw: Some(200),
            minh: Some(300),
            maxh: Some(400),
            tag: vec![kani_openapi::models::TagId("12345".to_owned()), kani_openapi::models::TagId("67890".to_owned())],
            etag: vec![kani_openapi::models::TagId("67890".to_owned())],
            sort: vec!["id:desc".to_owned(), "date".to_owned(), "resolution:asc".to_owned()],
        };

        let filter = params.try_to_domain();

        assert!(filter.is_ok());
        let filter = filter.unwrap();
        assert_eq!(filter, ImageSearchFilter {
            collection: Some(CollectionId(12345)),
            layout: Some(Layout::Portrait),
            min_width: Some(100),
            max_width: Some(200),
            min_height: Some(300),
            max_height: Some(400),
            tags: vec![TagId(12345), TagId(67890)],
            exclude_tags: vec![TagId(67890)],
            sort: vec![SortOption::Id(Desc), SortOption::Date(Desc), SortOption::Resolution(SortOrder::Asc)],
        });
    }
}
