use kani_domain_api_model::collection::{Collection, CollectionId, CollectionName};
use kani_domain_api_model::image_id::ImageId;
use kani_domain_api_model::user::User;

pub trait CollectionDatabase {
    fn load_collection_by_user_and_id(&self, user: &User, collection_id: CollectionId) -> Result<Option<Collection>, anyhow::Error>;
    fn load_collection_by_user_and_name(&self, user: &User, collection_name: &CollectionName) -> Result<Option<Collection>, anyhow::Error>;
    fn load_collections_by_user(&self, user: &User) -> Result<Vec<Collection>, anyhow::Error>;
    fn create_collection(&self, user: &User, name: &CollectionName) -> Result<Collection, anyhow::Error>;
    fn delete_collection(&self, user: &User, collection_id: CollectionId) -> Result<(), anyhow::Error>;
    fn add_images_to_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<usize, anyhow::Error>;
    fn remove_images_from_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<usize, anyhow::Error>;
}


#[cfg(feature = "test-util")]
pub mod mock {
    use super::*;

    pub struct MockCollectionDatabase {
        pub load_collection_by_user_and_id_fn: Box<dyn Fn(&User, CollectionId) -> Result<Option<Collection>, anyhow::Error> + Send + Sync>,
        pub load_collection_by_user_and_name_fn: Box<dyn Fn(&User, &CollectionName) -> Result<Option<Collection>, anyhow::Error> + Send + Sync>,
        pub load_collections_by_user_fn: Box<dyn Fn(&User) -> Result<Vec<Collection>, anyhow::Error> + Send + Sync>,
        pub create_collection_fn: Box<dyn Fn(&User, &CollectionName) -> Result<Collection, anyhow::Error> + Send + Sync>,
        pub delete_collection_fn: Box<dyn Fn(&User, CollectionId) -> Result<(), anyhow::Error> + Send + Sync>,
        pub add_images_to_collection_fn: Box<dyn Fn(&User, CollectionId, &[ImageId]) -> Result<usize, anyhow::Error> + Send + Sync>,
        pub remove_images_from_collection_fn: Box<dyn Fn(&User, CollectionId, &[ImageId]) -> Result<usize, anyhow::Error> + Send + Sync>,
    }

    impl Default for MockCollectionDatabase {
        fn default() -> Self {
            Self {
                load_collection_by_user_and_id_fn: Box::new(|_, _| unimplemented!("load_collection_by_user_and_id was not configured")),
                load_collection_by_user_and_name_fn: Box::new(|_, _| unimplemented!("load_collection_by_user_and_name was not configured")),
                load_collections_by_user_fn: Box::new(|_| unimplemented!("load_collections_by_user was not configured")),
                create_collection_fn: Box::new(|_, _| unimplemented!("create_collection was not configured")),
                delete_collection_fn: Box::new(|_, _| unimplemented!("delete_collection was not configured")),
                add_images_to_collection_fn: Box::new(|_, _, _| unimplemented!("add_images_to_collection was not configured")),
                remove_images_from_collection_fn: Box::new(|_, _, _| unimplemented!("remove_images_from_collection was not configured")),
            }
        }
    }

    impl MockCollectionDatabase {
        pub fn with_load_collection_by_user_and_id(
            mut self,
            f: impl Fn(&User, CollectionId) -> Result<Option<Collection>, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.load_collection_by_user_and_id_fn = Box::new(f);
            self
        }

        pub fn with_load_collection_by_user_and_name(
            mut self,
            f: impl Fn(&User, &CollectionName) -> Result<Option<Collection>, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.load_collection_by_user_and_name_fn = Box::new(f);
            self
        }

        pub fn with_load_collections_by_user(
            mut self,
            f: impl Fn(&User) -> Result<Vec<Collection>, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.load_collections_by_user_fn = Box::new(f);
            self
        }

        pub fn with_create_collection(
            mut self,
            f: impl Fn(&User, &CollectionName) -> Result<Collection, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.create_collection_fn = Box::new(f);
            self
        }

        pub fn with_delete_collection(
            mut self,
            f: impl Fn(&User, CollectionId) -> Result<(), anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.delete_collection_fn = Box::new(f);
            self
        }

        pub fn with_add_images_to_collection(
            mut self,
            f: impl Fn(&User, CollectionId, &[ImageId]) -> Result<usize, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.add_images_to_collection_fn = Box::new(f);
            self
        }

        pub fn with_remove_images_from_collection(
            mut self,
            f: impl Fn(&User, CollectionId, &[ImageId]) -> Result<usize, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.remove_images_from_collection_fn = Box::new(f);
            self
        }
    }

    impl CollectionDatabase for MockCollectionDatabase {
        fn load_collection_by_user_and_id(&self, user: &User, collection_id: CollectionId) -> Result<Option<Collection>, anyhow::Error> {
            (self.load_collection_by_user_and_id_fn)(user, collection_id)
        }

        fn load_collection_by_user_and_name(&self, user: &User, collection_name: &CollectionName) -> Result<Option<Collection>, anyhow::Error> {
            (self.load_collection_by_user_and_name_fn)(user, collection_name)
        }

        fn load_collections_by_user(&self, user: &User) -> Result<Vec<Collection>, anyhow::Error> {
            (self.load_collections_by_user_fn)(user)
        }

        fn create_collection(&self, user: &User, name: &CollectionName) -> Result<Collection, anyhow::Error> {
            (self.create_collection_fn)(user, name)
        }

        fn delete_collection(&self, user: &User, collection_id: CollectionId) -> Result<(), anyhow::Error> {
            (self.delete_collection_fn)(user, collection_id)
        }

        fn add_images_to_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<usize, anyhow::Error> {
            (self.add_images_to_collection_fn)(user, collection_id, image_ids)
        }

        fn remove_images_from_collection(&self, user: &User, collection_id: CollectionId, image_ids: &[ImageId]) -> Result<usize, anyhow::Error> {
            (self.remove_images_from_collection_fn)(user, collection_id, image_ids)
        }
    }
}
