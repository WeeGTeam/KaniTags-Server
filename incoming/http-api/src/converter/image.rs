use crate::converter::FromDomain;
use kani_openapi::models::ImageId;

impl FromDomain<kani_domain_api_model::image_id::ImageId> for ImageId {
    fn from_domain(value: kani_domain_api_model::image_id::ImageId) -> Self {
        ImageId(value.format_id_hash())
    }
}
