use image::DynamicImage;
use kani_domain_api_model::image_hash::{IdHash, PerceptualHash};

mod id_hash;
mod perceptual_hash;

// TODO: think about implementing hashes because of changes/inconsistencies between versions/platforms

pub fn get_id_hash(bytes: &[u8]) -> IdHash {
    id_hash::calculate_id_hash(bytes)
}

pub fn get_perceptual_hash(image: &DynamicImage) -> PerceptualHash {
    perceptual_hash::calculate_perceptual_hash(image)
}

