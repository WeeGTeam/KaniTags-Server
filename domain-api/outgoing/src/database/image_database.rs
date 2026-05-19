use kani_domain_api_model::image::{CreatePantsuImage, PantsuImage};
use kani_domain_api_model::image_hash::IdHash;
use kani_domain_api_model::import::ImportSession;
use kani_domain_api_model::user::User;

pub trait ImageDatabase {
    fn get_image_by_id_hash(&self, id_hash: &IdHash) -> Result<Option<PantsuImage>, anyhow::Error>;

    fn store_image(&self, user: &User, import_session_id: i64, image: &CreatePantsuImage) -> Result<PantsuImage, anyhow::Error>;

    fn start_import_session(&self, user: &User) -> Result<ImportSession, anyhow::Error>;
}
