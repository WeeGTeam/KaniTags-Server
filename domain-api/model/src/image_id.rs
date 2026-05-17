use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use crate::image_hash::{hash_to_hex, IdHash, PerceptualHash};
use anyhow::{anyhow, Context};
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageId {
    id_hash: IdHash,
    perceptual_hash: PerceptualHash,
}

impl ImageId {
    pub fn new(id_hash: IdHash, perceptual_hash: PerceptualHash) -> Self {
        ImageId {
            id_hash,
            perceptual_hash,
        }
    }

    pub fn get_id_hash(&self) -> &IdHash {
        &self.id_hash
    }

    pub fn get_perceptual_hash(&self) -> &PerceptualHash {
        &self.perceptual_hash
    }

    pub fn filename_format(&self) -> String {
        format!("{}-{}", self.format_id_hash(), self.format_perceptual_hash())
    }

    pub fn format_id_hash(&self) -> String {
        hash_to_hex(&self.id_hash)
    }

    pub fn format_perceptual_hash(&self) -> String {
        hash_to_hex(&self.perceptual_hash)
    }
}

impl FromStr for ImageId {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"^(?<id>[[:xdigit:]]{16})-(?<perceptual>[[:xdigit:]]{36})$").unwrap();
        let captures = regex.captures(value.trim())
            .ok_or_else(|| anyhow!("Cannot parse string to ImageId, unexpected format: {}", value.to_owned()))?;
        let id_hash: IdHash = hex_to_hash::<8>(&captures["id"])?;
        let perceptual_hash: PerceptualHash = hex_to_hash::<18>(&captures["perceptual"])?;
        Ok(ImageId {
            id_hash,
            perceptual_hash,
        })
    }
}

/// should be infallible, because we already check for valid size and content of `str` with regex
fn hex_to_hash<const SIZE: usize>(str: &str) -> Result<[u8; SIZE], anyhow::Error> {
    (0..2*SIZE)
        .step_by(2)
        .map(|i| u8::from_str_radix(&str[i..i+2], 16))
        .collect::<Result<Vec<u8>, ParseIntError>>()
        .with_context(|| format!("Unable to parse hex to hash: {}", str.to_owned()))?
        .try_into()
        .map_err(|_| anyhow!("Unable to parse hex to hash: {}", str))
}

impl Display for ImageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.filename_format())
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::ImageId;

    #[test]
    fn creates_image_id_from_correct_string() {
        let name = "a8c65b2726296dcc-07807e4fe23cb3c1dca0ce71f382bf81f00f";
        ImageId::from_str(name).unwrap();
    }

    #[test]
    fn empty_string_is_invalid() {
        let name = "";
        let image_id = ImageId::from_str(name);
        assert!(matches!(image_id, Err(anyhow::Error {..})));
    }

    #[test]
    fn no_dash_is_invalid() {
        let name = "a8c65b2726296dcc07807e4fe23cb3c1dca0ce71f382bf81f00f";
        let image_id = ImageId::from_str(name);
        assert!(matches!(image_id, Err(anyhow::Error {..})));
    }

    #[test]
    fn non_hex_string_is_invalid() {
        let name = "a8c65j2726296dcc-07807e4fe23cb3c1dca0ce71f382bf81f00f";
        let image_id = ImageId::from_str(name);
        assert!(matches!(image_id, Err(anyhow::Error {..})));
    }

    #[test]
    fn too_short_id_hash_is_invalid() {
        let name = "a8c652726296dcc-07807e4fe23cb3c1dca0ce71f382bf81f00f";
        let image_id = ImageId::from_str(name);
        assert!(matches!(image_id, Err(anyhow::Error {..})));
    }

    #[test]
    fn too_short_perceptual_hash_is_invalid() {
        let name = "a8c65b2726296dcc-07807e4fe23cb3c1dca0ce71f382bf8100f";
        let image_id = ImageId::from_str(name);
        assert!(matches!(image_id, Err(anyhow::Error {..})));
    }

    #[test]
    fn excess_string_is_invalid() {
        let name = "a8c65b2726296dcc-07807e4fe23cb3c1dca0ce71f382bf81f00f HelloThere";
        let image_id = ImageId::from_str(name);
        assert!(matches!(image_id, Err(anyhow::Error {..})));
    }
}
