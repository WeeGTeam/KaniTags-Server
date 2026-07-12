use kani_domain_api_model::image_id::ImageIdHash;
use kani_domain_api_model::similarity::{SimilarImage, SimilarImagePair};

pub trait SimilarityDatabase {
    fn get_similar_images(&self, image_id_hash: &ImageIdHash) -> Result<Vec<SimilarImage>, anyhow::Error>;
    fn get_all_similar_images(&self) -> Result<Vec<SimilarImagePair>, anyhow::Error>;
}


#[cfg(feature = "test-util")]
pub mod mock {
    use super::*;

    pub struct MockSimilarityDatabase {
        pub get_similar_images_fn: Box<dyn Fn(&ImageIdHash) -> Result<Vec<SimilarImage>, anyhow::Error> + Send + Sync>,
        pub get_all_similar_images_fn: Box<dyn Fn() -> Result<Vec<SimilarImagePair>, anyhow::Error> + Send + Sync>,
    }

    impl Default for MockSimilarityDatabase {
        fn default() -> Self {
            Self {
                get_similar_images_fn: Box::new(|_| unimplemented!("get_similar_images was not configured")),
                get_all_similar_images_fn: Box::new(|| unimplemented!("get_all_similar_images was not configured")),
            }
        }
    }

    impl MockSimilarityDatabase {
        pub fn with_get_similar_images(
            mut self,
            f: impl Fn(&ImageIdHash) -> Result<Vec<SimilarImage>, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.get_similar_images_fn = Box::new(f);
            self
        }

        pub fn with_get_all_similar_images(
            mut self,
            f: impl Fn() -> Result<Vec<SimilarImagePair>, anyhow::Error> + Send + Sync + 'static,
        ) -> Self {
            self.get_all_similar_images_fn = Box::new(f);
            self
        }
    }

    impl SimilarityDatabase for MockSimilarityDatabase {
        fn get_similar_images(&self, image_id_hash: &ImageIdHash) -> Result<Vec<SimilarImage>, anyhow::Error> {
            (self.get_similar_images_fn)(image_id_hash)
        }

        fn get_all_similar_images(&self) -> Result<Vec<SimilarImagePair>, anyhow::Error> {
            (self.get_all_similar_images_fn)()
        }
    }
}