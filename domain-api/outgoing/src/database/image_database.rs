use kani_domain_api_model::image::{CreatePantsuImage, PantsuImage};
use kani_domain_api_model::image_id::{ImageId, ImageIdHash};
use kani_domain_api_model::image_search::ImageSearchFilter;
use kani_domain_api_model::import::ImportSessionId;
use kani_domain_api_model::user::User;

pub trait ImageDatabase {
    fn get_image_by_image_id(&self, image_id: ImageId) -> Result<Option<PantsuImage>, anyhow::Error>;

    fn get_image_by_image_id_hash(&self, image_id_hash: &ImageIdHash) -> Result<Option<PantsuImage>, anyhow::Error>;

    fn get_images_by_image_ids(&self, user: &User, id_hash: &[ImageId]) -> Result<Vec<PantsuImage>, anyhow::Error>;

    fn store_image(&self, user: &User, import_session_id: ImportSessionId, image: &CreatePantsuImage) -> Result<PantsuImage, anyhow::Error>;

    fn start_import_session(&self, user: &User) -> Result<ImportSessionId, anyhow::Error>;

    fn close_import_session(&self, import_session_id: ImportSessionId) -> Result<(), anyhow::Error>;

    fn search_images(&self, user: &User, filter: &ImageSearchFilter) -> Result<Vec<ImageId>, anyhow::Error>;
}


#[cfg(feature = "test-util")]
pub mod mock {
    use super::*;

    pub struct MockImageDatabase {
        pub get_image_by_image_id_fn: Box<dyn Fn(ImageId) -> Result<Option<PantsuImage>, anyhow::Error> + Send + Sync>,
        pub get_image_by_image_id_hash_fn: Box<dyn Fn(&ImageIdHash) -> Result<Option<PantsuImage>, anyhow::Error> + Send + Sync>,
        pub get_images_by_image_ids_fn: Box<dyn Fn(&User, &[ImageId]) -> Result<Vec<PantsuImage>, anyhow::Error> + Send + Sync>,
        pub store_image_fn: Box<dyn Fn(&User, ImportSessionId, &CreatePantsuImage) -> Result<PantsuImage, anyhow::Error> + Send + Sync>,
        pub start_import_session_fn: Box<dyn Fn(&User) -> Result<ImportSessionId, anyhow::Error> + Send + Sync>,
        pub close_import_session_fn: Box<dyn Fn(ImportSessionId) -> Result<(), anyhow::Error> + Send + Sync>,
        pub search_images_fn: Box<dyn Fn(&User, &ImageSearchFilter) -> Result<Vec<ImageId>, anyhow::Error> + Send + Sync>,
    }

    impl Default for MockImageDatabase {
        fn default() -> Self {
            Self {
                get_image_by_image_id_fn: Box::new(|_| unimplemented!("get_image_by_image_id_fn was not configured")),
                get_image_by_image_id_hash_fn: Box::new(|_| unimplemented!("get_image_by_image_id_hash_fn")),
                get_images_by_image_ids_fn: Box::new(|_, _| unimplemented!("get_images_by_image_ids_fn")),
                store_image_fn: Box::new(|_, _, _| unimplemented!("store_image_fn")),
                start_import_session_fn: Box::new(|_| unimplemented!("start_import_session_fn")),
                close_import_session_fn: Box::new(|_| unimplemented!("close_import_session_fn")),
                search_images_fn: Box::new(|_, _| unimplemented!("search_images_fn")),
            }
        }
    }

    impl MockImageDatabase {
        pub fn with_get_image_by_image_id(
            mut self,
            f: impl Fn(ImageId) -> Result<Option<PantsuImage>, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.get_image_by_image_id_fn = Box::new(f);
            self
        }

        pub fn with_get_image_by_image_id_hash(
            mut self,
            f: impl Fn(&ImageIdHash) -> Result<Option<PantsuImage>, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.get_image_by_image_id_hash_fn = Box::new(f);
            self
        }

        pub fn with_get_images_by_image_ids(
            mut self,
            f: impl Fn(&User, &[ImageId]) -> Result<Vec<PantsuImage>, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.get_images_by_image_ids_fn = Box::new(f);
            self
        }

        pub fn with_store_image(
            mut self,
            f: impl Fn(&User, ImportSessionId, &CreatePantsuImage) -> Result<PantsuImage, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.store_image_fn = Box::new(f);
            self
        }

        pub fn with_start_import_session(
            mut self,
            f: impl Fn(&User) -> Result<ImportSessionId, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.start_import_session_fn = Box::new(f);
            self
        }

        pub fn with_close_import_session(
            mut self,
            f: impl Fn(ImportSessionId) -> Result<(), anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.close_import_session_fn = Box::new(f);
            self
        }

        pub fn with_search_images(
            mut self,
            f: impl Fn(&User, &ImageSearchFilter) -> Result<Vec<ImageId>, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.search_images_fn = Box::new(f);
            self
        }
    }

    impl ImageDatabase for MockImageDatabase {
        fn get_image_by_image_id(&self, image_id: ImageId) -> Result<Option<PantsuImage>, anyhow::Error> {
            (self.get_image_by_image_id_fn)(image_id)
        }
        fn get_image_by_image_id_hash(&self, image_id_hash: &ImageIdHash) -> Result<Option<PantsuImage>, anyhow::Error> {
            (self.get_image_by_image_id_hash_fn)(image_id_hash)
        }
        fn get_images_by_image_ids(&self, user: &User, id_hash: &[ImageId]) -> Result<Vec<PantsuImage>, anyhow::Error> {
            (self.get_images_by_image_ids_fn)(user, id_hash)
        }
        fn store_image(&self, user: &User, import_session_id: ImportSessionId, image: &CreatePantsuImage) -> Result<PantsuImage, anyhow::Error> {
            (self.store_image_fn)(user, import_session_id, image)
        }
        fn start_import_session(&self, user: &User) -> Result<ImportSessionId, anyhow::Error> {
            (self.start_import_session_fn)(user)
        }

        fn close_import_session(&self, import_session_id: ImportSessionId) -> Result<(), anyhow::Error> {
            (self.close_import_session_fn)(import_session_id)
        }

        fn search_images(&self, user: &User, filter: &ImageSearchFilter) -> Result<Vec<ImageId>, anyhow::Error> {
            (self.search_images_fn)(user, filter)
        }
    }
}
