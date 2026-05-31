pub mod convert;

use crate::error::HttpApiUnhandledError;
use crate::router::image_list::convert::{convert_filter, convert_images};
use crate::router::AppState;
use async_trait::async_trait;
use axum::http::Method;
use axum_extra::extract::CookieJar;
use headers::Host;
use kani_openapi::apis::image_list::{GetImagesResponse, ImageList};
use kani_openapi::models::GetImagesQueryParams;

#[async_trait]
impl ImageList<HttpApiUnhandledError> for AppState {
    async fn get_images(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
        query_params: &GetImagesQueryParams,
    ) -> Result<GetImagesResponse, HttpApiUnhandledError> {
        let filter = convert_filter(query_params)?;
        let images = self.image_search_service.search_images(&filter).map_err(|e| HttpApiUnhandledError::Unknown(e.into()))?;
        let images = convert_images(images);
        Ok(GetImagesResponse::Status200_Ok(images))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::image_list::convert::convert_filter;
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

        let filter = convert_filter(&params);

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
