use kani_domain_api_incoming::tag_service::{AddImageTagsError, GetImageTagsError, GetTagsError, TagService};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::tag::image_tag::ImageTag;
use kani_domain_api_model::tag::{NewTag, Tag};
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::Database;
use std::sync::Arc;

pub struct TagServiceImpl {
    database: Arc<dyn Database + Sync + Send>,
}

impl TagServiceImpl {
    pub fn new(
        database: Arc<dyn Database + Send + Sync + 'static>,
    ) -> Self {
        Self {
            database,
        }
    }
}

impl TagService for TagServiceImpl {
    fn get_tags(&self) -> Result<Vec<Tag>, GetTagsError> {
        todo!()
    }

    fn get_image_tags(&self, _image_id: ImageId) -> Result<Vec<ImageTag>, GetImageTagsError> {
        todo!()
    }

    fn add_image_tags(&self, _image_id: ImageId, _new_tag: Vec<NewTag>, _user: User) -> Result<Vec<ImageTag>, AddImageTagsError> {
        todo!()
    }
}
