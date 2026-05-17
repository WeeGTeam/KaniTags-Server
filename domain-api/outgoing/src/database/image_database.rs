use kani_domain_api_model::image::{CreatePantsuImage, PantsuImage};
use kani_domain_api_model::image_hash::IdHash;

pub trait ImageDatabase {
    fn get_image_by_id_hash(&self, id_hash: &IdHash) -> Result<Option<PantsuImage>, anyhow::Error>;

    fn store_image(&self, image: &CreatePantsuImage) -> Result<PantsuImage, anyhow::Error>;
}
