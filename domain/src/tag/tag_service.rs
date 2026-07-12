use kani_domain_api_incoming::tag_service::{AddImageTagsError, GetImageTagsError, GetTagsError, TagService};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::tag::image_tag::ImageTag;
use kani_domain_api_model::tag::{NewTag, Tag};
use kani_domain_api_model::user::User;
use kani_domain_api_outgoing::database::Database;
use std::sync::Arc;
use tracing::info;

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
        info!("Getting all tags");
        let tags = self.database.tag().get_all_tags()?;
        info!("Retrieved {} tags", tags.len());
        Ok(tags)
    }

    fn get_image_tags(&self, image_id: ImageId) -> Result<Vec<ImageTag>, GetImageTagsError> {
        info!("Getting image tags of image {}", *image_id);
        let image_tags = self.database.tag().get_image_tags_of_image(&image_id)?;
        info!("Retrieved {} image tags of image {}", image_tags.len(), *image_id);
        Ok(image_tags)
    }

    fn add_image_tags(&self, image_id: ImageId, new_tags: Vec<NewTag>, user: User) -> Result<Vec<ImageTag>, AddImageTagsError> {
        info!("Adding image tags to image {}", *image_id);
        if let None = self.database.image().get_image_by_image_id(image_id.clone())? {
            info!("Image does not exist: {}", *image_id);
            return Err(AddImageTagsError::ImageNotFound(image_id));
        }

        let tags = self.database.tag().get_tags_create_if_missing(new_tags)?;
        let added_image_tags_number = self.database.tag().add_image_tags_to_image_by_user(tags, image_id.clone(), user)?;
        info!("Added {} image tags to image {}", added_image_tags_number, *image_id);

        Ok(self.database.tag().get_image_tags_of_image(&image_id)?)
    }
}

#[cfg(test)]
mod test {
    use crate::tag::tag_service::TagServiceImpl;
    use kani_domain_api_incoming::tag_service::TagService;
    use kani_domain_api_model::image_id::ImageId;
    use kani_domain_api_model::tag::NewTag;
    use kani_domain_api_model::user::User;
    use kani_domain_api_outgoing::database::image_database::mock::MockImageDatabase;
    use kani_domain_api_outgoing::database::mock::MockDatabase;
    use std::sync::Arc;

    mod test_add_image_tags {
        use super::*;
        use assertables::assert_err;

        #[test]
        fn should_fail_on_non_existing_image() {
            let database = MockDatabase::default()
                .with_image_database(MockImageDatabase::default()
                    .with_get_image_by_image_id(|_| Ok(None)));
            let tag_service = TagServiceImpl::new(Arc::new(database));

            let result = tag_service.add_image_tags(ImageId(1), vec![NewTag::stub()], User::stub());

            assert_err!(result);
        }
    }
}
