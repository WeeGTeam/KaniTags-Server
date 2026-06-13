use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::tag::image_tag::ImageTag;
use kani_domain_api_model::tag::{NewTag, Tag};
use kani_domain_api_model::user::User;
use thiserror::Error;

pub trait TagDatabase {
    fn get_all_tags(&self) -> Result<Vec<Tag>, anyhow::Error>;

    fn get_image_tags_of_image(&self, image_id: &ImageId) -> Result<Vec<ImageTag>, anyhow::Error>;

    fn add_image_tags_to_image_by_user(&self, new_tags: Vec<NewTag>, image_id: ImageId, user: User) -> Result<Vec<ImageTag>, AddImageTagsByUserError>;
}

#[derive(Error, Debug)]
pub enum AddImageTagsByUserError {
    #[error("Add image tags by user internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),

    #[error("Image not found: '{0}'")]
    ImageNotFound(ImageId),
}
