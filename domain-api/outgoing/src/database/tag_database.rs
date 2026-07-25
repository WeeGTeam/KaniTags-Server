use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::tag::image_tag::ImageTag;
use kani_domain_api_model::tag::{NewTag, Tag};
use kani_domain_api_model::user::User;

#[cfg_attr(feature = "test-util", mockall::automock)]
pub trait TagDatabase {
    fn get_all_tags(&self) -> Result<Vec<Tag>, anyhow::Error>;

    fn get_tags_create_if_missing(&self, new_tags: Vec<NewTag>) -> Result<Vec<Tag>, anyhow::Error>;

    fn get_image_tags_of_image(&self, image_id: &ImageId) -> Result<Vec<ImageTag>, anyhow::Error>;

    fn add_image_tags_to_image_by_user(&self, tags: Vec<Tag>, image_id: ImageId, user: User) -> Result<usize, anyhow::Error>;
}
