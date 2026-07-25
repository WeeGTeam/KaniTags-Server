use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::tag::image_tag::ImageTag;
use kani_domain_api_model::tag::{NewTag, Tag};
use kani_domain_api_model::user::User;
use thiserror::Error;

#[async_trait::async_trait]
pub trait TagService {
    async fn get_tags(&self) -> Result<Vec<Tag>, GetTagsError>;

    async fn get_image_tags(&self, image_id: ImageId) -> Result<Vec<ImageTag>, GetImageTagsError>;

    async fn add_image_tags(&self, image_id: ImageId, new_tags: Vec<NewTag>, user: User) -> Result<Vec<ImageTag>, AddImageTagsError>;
}

#[derive(Error, Debug)]
pub enum GetTagsError {
    #[error("Get tags internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum GetImageTagsError {
    #[error("Get image tags internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum AddImageTagsError {
    #[error("Add image tags internal server error: '{0}'")]
    Unknown(#[from] anyhow::Error),

    #[error("Image not found: '{0:?}'")]
    ImageNotFound(ImageId),
}
