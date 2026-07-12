use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::tag::image_tag::ImageTag;
use kani_domain_api_model::tag::{NewTag, Tag};
use kani_domain_api_model::user::User;

pub trait TagDatabase {
    fn get_all_tags(&self) -> Result<Vec<Tag>, anyhow::Error>;

    fn get_tags_create_if_missing(&self, new_tags: Vec<NewTag>) -> Result<Vec<Tag>, anyhow::Error>;

    fn get_image_tags_of_image(&self, image_id: &ImageId) -> Result<Vec<ImageTag>, anyhow::Error>;

    fn add_image_tags_to_image_by_user(&self, tags: Vec<Tag>, image_id: ImageId, user: User) -> Result<usize, anyhow::Error>;
}


#[cfg(feature = "test-util")]
pub mod mock {
    use super::*;

    pub struct MockTagDatabase {
        pub get_all_tags_fn: Box<dyn Fn() -> Result<Vec<Tag>, anyhow::Error> + Send + Sync>,
        pub get_tags_create_if_missing_fn: Box<dyn Fn(Vec<NewTag>) -> Result<Vec<Tag>, anyhow::Error> + Send + Sync>,
        pub get_image_tags_of_image_fn: Box<dyn Fn(&ImageId) -> Result<Vec<ImageTag>, anyhow::Error> + Send + Sync>,
        pub add_image_tags_to_image_by_user_fn: Box<dyn Fn(Vec<Tag>, ImageId, User) -> Result<usize, anyhow::Error> + Send + Sync>,
    }

    impl Default for MockTagDatabase {
        fn default() -> Self {
            Self {
                get_all_tags_fn: Box::new(|| unimplemented!("get_all_tags was not configured")),
                get_tags_create_if_missing_fn: Box::new(|_| unimplemented!("get_tags_create_if_missing was not configured")),
                get_image_tags_of_image_fn: Box::new(|_| unimplemented!("get_image_tags_of_image was not configured")),
                add_image_tags_to_image_by_user_fn: Box::new(|_, _, _| unimplemented!("add_image_tags_to_image_by_user was not configured")),
            }
        }
    }

    impl MockTagDatabase {
        pub fn with_get_all_tags(
            mut self,
            f: impl Fn() -> Result<Vec<Tag>, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.get_all_tags_fn = Box::new(f);
            self
        }

        pub fn with_get_tags_create_if_missing(
            mut self,
            f: impl Fn(Vec<NewTag>) -> Result<Vec<Tag>, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.get_tags_create_if_missing_fn = Box::new(f);
            self
        }

        pub fn with_get_image_tags_of_image(
            mut self,
            f: impl Fn(&ImageId) -> Result<Vec<ImageTag>, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.get_image_tags_of_image_fn = Box::new(f);
            self
        }

        pub fn with_add_image_tags_to_image_by_user(
            mut self,
            f: impl Fn(Vec<Tag>, ImageId, User) -> Result<usize, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.add_image_tags_to_image_by_user_fn = Box::new(f);
            self
        }
    }

    impl TagDatabase for MockTagDatabase {
        fn get_all_tags(&self) -> Result<Vec<Tag>, anyhow::Error> {
            (self.get_all_tags_fn)()
        }

        fn get_tags_create_if_missing(&self, new_tags: Vec<NewTag>) -> Result<Vec<Tag>, anyhow::Error> {
            (self.get_tags_create_if_missing_fn)(new_tags)
        }

        fn get_image_tags_of_image(&self, image_id: &ImageId) -> Result<Vec<ImageTag>, anyhow::Error> {
            (self.get_image_tags_of_image_fn)(image_id)
        }

        fn add_image_tags_to_image_by_user(&self, tags: Vec<Tag>, image_id: ImageId, user: User) -> Result<usize, anyhow::Error> {
            (self.add_image_tags_to_image_by_user_fn)(tags, image_id, user)
        }
    }
}
