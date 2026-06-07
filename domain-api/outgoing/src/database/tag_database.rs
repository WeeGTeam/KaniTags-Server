use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::tag::image_tag::ImageTag;
use kani_domain_api_model::tag::Tag;

pub trait TagDatabase {
    fn get_all_tags(&self) -> Result<Vec<Tag>, anyhow::Error>;

    fn get_image_tags_of_image(&self, image_id: &ImageId) -> Result<Vec<ImageTag>, anyhow::Error>;
}
