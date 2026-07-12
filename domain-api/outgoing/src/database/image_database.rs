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
