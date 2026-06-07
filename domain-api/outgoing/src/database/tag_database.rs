use kani_domain_api_model::tag::Tag;

pub trait TagDatabase {
    fn get_all_tags(&self) -> Result<Vec<Tag>, anyhow::Error>;
}
