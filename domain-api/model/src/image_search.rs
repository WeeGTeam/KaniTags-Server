use crate::collection::CollectionId;
use crate::import::ImportSessionId;
use crate::tag::TagId;
use std::str::FromStr;
use strum::EnumString;

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum Layout {
    #[strum(ascii_case_insensitive)]
    Portrait,
    #[strum(ascii_case_insensitive)]
    Landscape,
    #[strum(ascii_case_insensitive)]
    Square,
}

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum SortOrder {
    #[strum(ascii_case_insensitive)]
    Asc,
    #[strum(ascii_case_insensitive)]
    Desc,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortOption {
    Id(SortOrder),
    Date(SortOrder),
    Resolution(SortOrder),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImageSearchFilter {
    pub import_session: Option<ImportSessionId>,
    pub collection: Option<CollectionId>,
    pub layout: Option<Layout>,
    pub min_width: Option<u32>,
    pub max_width: Option<u32>,
    pub min_height: Option<u32>,
    pub max_height: Option<u32>,
    pub tags: Vec<TagId>,
    pub exclude_tags: Vec<TagId>,
    pub sort: Vec<SortOption>,
}

impl FromStr for SortOption {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(':').collect::<Vec<_>>();
        let first = split.first().ok_or_else(|| anyhow::anyhow!("Invalid sort option: {}", s))?;
        let second = split.get(1).map(|direction| direction.parse::<SortOrder>()).transpose()?;
        match *first {
            "id" => Ok(SortOption::Id(second.unwrap_or(SortOrder::Desc))),
            "date" => Ok(SortOption::Date(second.unwrap_or(SortOrder::Desc))),
            "resolution" => Ok(SortOption::Resolution(second.unwrap_or(SortOrder::Desc))),
            _ => Err(anyhow::anyhow!("Invalid sort option: {}", s)),
        }
    }
}
