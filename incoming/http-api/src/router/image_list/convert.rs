use anyhow::Context;
use kani_domain_api_model::collection::CollectionId;
use kani_domain_api_model::image_search::ImageSearchFilter;
use kani_domain_api_model::tag::TagId;
use kani_openapi::models::{GetImagesQueryParams, ImageId};

pub fn convert_filter(params: &GetImagesQueryParams) -> Result<ImageSearchFilter, anyhow::Error> {
    Ok(ImageSearchFilter {
        collection: params.collection.as_ref().map(|c| c.parse()).transpose().context("invalid collection id")?.map(|i| CollectionId(i)),
        layout: params.layout.as_ref().map(|l| l.parse()).transpose().context("invalid layout")?,
        min_width: params.minw.map(|value| value as u32),
        max_width: params.maxw.map(|value| value as u32),
        min_height: params.minh.map(|value| value as u32),
        max_height: params.maxh.map(|value| value as u32),
        tags: params.tag.iter().map(|tag| tag.parse().and_then(|tag| Ok(TagId(tag)))).collect::<Result<Vec<_>, _>>()?,
        exclude_tags: params.etag.iter().map(|tag| tag.parse().and_then(|tag| Ok(TagId(tag)))).collect::<Result<Vec<_>, _>>()?,
        sort: params.sort.iter().map(|sort| sort.parse()).collect::<Result<Vec<_>, _>>()?,
    })
}

pub fn convert_images(images: Vec<kani_domain_api_model::image_id::ImageId>) -> Vec<ImageId> {
    images.into_iter().map(|id| ImageId(id.format_id_hash())).collect()
}
