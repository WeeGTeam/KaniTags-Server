use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::ops::Deref;
use std::str::FromStr;

use crate::image_hash::{hash_to_hex, IdHash};
use anyhow::{anyhow, Context};
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageId(pub i64);

impl Deref for ImageId {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageIdHash(pub IdHash);

impl ImageIdHash {
    pub fn format_id_hash(&self) -> String {
        hash_to_hex(&self.0)
    }
}

impl FromStr for ImageIdHash {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"^(?<id>[[:xdigit:]]{16})$")?;
        let captures = regex.captures(value.trim())
            .ok_or_else(|| anyhow!("Cannot parse string to ImageId, unexpected format: {}", value.to_owned()))?;
        let id_hash: IdHash = hex_to_hash::<8>(&captures[0])?;
        Ok(ImageIdHash(id_hash))
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

impl Display for ImageIdHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_id_hash())
    }
}

impl Deref for ImageIdHash {
    type Target = IdHash;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::ImageIdHash;

    #[test]
    fn creates_image_id_from_correct_string() {
        let name = "a8c65b2726296dcc";
        ImageIdHash::from_str(name).unwrap();
    }

    #[test]
    fn empty_string_is_invalid() {
        let name = "";
        let image_id_hash = ImageIdHash::from_str(name);
        assert!(matches!(image_id_hash, Err(anyhow::Error {..})));
    }

    #[test]
    fn non_hex_string_is_invalid() {
        let name = "a8c65j2726296dcc";
        let image_id_hash = ImageIdHash::from_str(name);
        assert!(matches!(image_id_hash, Err(anyhow::Error {..})));
    }

    #[test]
    fn too_short_id_hash_is_invalid() {
        let name = "a8c652726296dcc-07807e4fe23cb3c1dca0ce71f382bf81f00f";
        let image_id_hash = ImageIdHash::from_str(name);
        assert!(matches!(image_id_hash, Err(anyhow::Error {..})));
    }

    #[test]
    fn excess_string_is_invalid() {
        let name = "a8c65b2726296dcc HelloThere";
        let image_id_hash = ImageIdHash::from_str(name);
        assert!(matches!(image_id_hash, Err(anyhow::Error {..})));
    }
}
