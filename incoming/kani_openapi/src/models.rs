#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

#[allow(dead_code)]
fn from_validation_error(e: validator::ValidationError) -> validator::ValidationErrors {
  let mut errs = validator::ValidationErrors::new();
  errs.add("na", e);
  errs
}

#[allow(dead_code)]
pub fn check_xss_string(v: &str) -> std::result::Result<(), validator::ValidationError> {
    if ammonia::is_html(v) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_vec_string(v: &[String]) -> std::result::Result<(), validator::ValidationError> {
    if v.iter().any(|i| ammonia::is_html(i)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map_string(
    v: &std::collections::HashMap<String, String>,
) -> std::result::Result<(), validator::ValidationError> {
    if v.keys().any(|k| ammonia::is_html(k)) || v.values().any(|v| ammonia::is_html(v)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map_nested<T>(
    v: &std::collections::HashMap<String, T>,
) -> std::result::Result<(), validator::ValidationError>
where
    T: validator::Validate,
{
    if v.keys().any(|k| ammonia::is_html(k)) || v.values().any(|v| v.validate().is_err()) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map<T>(v: &std::collections::HashMap<String, T>) -> std::result::Result<(), validator::ValidationError> {
    if v.keys().any(|k| ammonia::is_html(k)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct AddImagesToCollectionPathParams {
                #[validate(
                          regex(path = *RE_ADDIMAGESTOCOLLECTIONPATHPARAMS_ID),
            )]
                pub id: String,
    }

    lazy_static::lazy_static! {
        static ref RE_ADDIMAGESTOCOLLECTIONPATHPARAMS_ID: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
    }



    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct DeleteCollectionPathParams {
                #[validate(
                          regex(path = *RE_DELETECOLLECTIONPATHPARAMS_ID),
            )]
                pub id: String,
    }

    lazy_static::lazy_static! {
        static ref RE_DELETECOLLECTIONPATHPARAMS_ID: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
    }



    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct RemoveImagesFromCollectionPathParams {
                #[validate(
                          regex(path = *RE_REMOVEIMAGESFROMCOLLECTIONPATHPARAMS_ID),
            )]
                pub id: String,
    }

    lazy_static::lazy_static! {
        static ref RE_REMOVEIMAGESFROMCOLLECTIONPATHPARAMS_ID: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetImagePathParams {
                #[validate(
                          regex(path = *RE_GETIMAGEPATHPARAMS_ID),
            )]
                pub id: String,
    }

    lazy_static::lazy_static! {
        static ref RE_GETIMAGEPATHPARAMS_ID: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetThumbnailImagePathParams {
                #[validate(
                          regex(path = *RE_GETTHUMBNAILIMAGEPATHPARAMS_ID),
            )]
                pub id: String,
    }

    lazy_static::lazy_static! {
        static ref RE_GETTHUMBNAILIMAGEPATHPARAMS_ID: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct CloseImportSessionPathParams {
                #[validate(
                          regex(path = *RE_CLOSEIMPORTSESSIONPATHPARAMS_ID),
            )]
                pub id: String,
    }

    lazy_static::lazy_static! {
        static ref RE_CLOSEIMPORTSESSIONPATHPARAMS_ID: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
    }



    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct ImportImagePathParams {
                #[validate(
                          regex(path = *RE_IMPORTIMAGEPATHPARAMS_ID),
            )]
                pub id: String,
    }

    lazy_static::lazy_static! {
        static ref RE_IMPORTIMAGEPATHPARAMS_ID: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
    }



    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetImagesQueryParams {
            /// limit search to this collection
                #[serde(rename = "collection")]
                #[validate(
                          regex(path = *RE_GETIMAGESQUERYPARAMS_COLLECTION),
              )]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub collection: Option<String>,
            /// limit search to this import session
                #[serde(rename = "import-session")]
                #[validate(
                          regex(path = *RE_GETIMAGESQUERYPARAMS_IMPORT_SESSION),
              )]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub import_session: Option<String>,
            /// limit search to layout type.
            /// Note: inline enums are not fully supported by openapi-generator
                #[serde(rename = "layout")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub layout: Option<String>,
            /// limit search to images with minimum width
                #[serde(rename = "minw")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub minw: Option<i32>,
            /// limit search to images with maximum width
                #[serde(rename = "maxw")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub maxw: Option<i32>,
            /// limit search to images with minimum height
                #[serde(rename = "minh")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub minh: Option<i32>,
            /// limit search to images with maximum height
                #[serde(rename = "maxh")]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub maxh: Option<i32>,
            /// limit search to images with these tags
                #[serde(rename = "tag")]
                    #[serde(default)]
                    pub tag: Vec<models::TagId>,
            /// limit search to images without these tags
                #[serde(rename = "etag")]
                    #[serde(default)]
                    pub etag: Vec<models::TagId>,
            /// sort order for results. pattern = \"(id|date|resolution)(|:asc|:desc)\" omitting ascending or descending results in default sort order (desc). 
                #[serde(rename = "sort")]
                    #[serde(default)]
                    pub sort: Vec<String>,
    }

    lazy_static::lazy_static! {
        static ref RE_GETIMAGESQUERYPARAMS_COLLECTION: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
    }
    lazy_static::lazy_static! {
        static ref RE_GETIMAGESQUERYPARAMS_IMPORT_SESSION: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
    }

    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct AddImageTagsPathParams {
                #[validate(
                          regex(path = *RE_ADDIMAGETAGSPATHPARAMS_ID),
            )]
                pub id: String,
    }

    lazy_static::lazy_static! {
        static ref RE_ADDIMAGETAGSPATHPARAMS_ID: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetImageTagsPathParams {
                #[validate(
                          regex(path = *RE_GETIMAGETAGSPATHPARAMS_ID),
            )]
                pub id: String,
    }

    lazy_static::lazy_static! {
        static ref RE_GETIMAGETAGSPATHPARAMS_ID: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
    }




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CollectionDto {
    #[serde(rename = "id")]
    #[validate(
            regex(path = *RE_COLLECTIONDTO_ID),
          custom(function = "check_xss_string"),
    )]
    pub id: String,

    /// max length: 60 characters
    #[serde(rename = "name")]
          #[validate(custom(function = "check_xss_string"))]
    pub name: String,

    #[serde(rename = "createdBy")]
    #[validate(
            regex(path = *RE_COLLECTIONDTO_CREATED_BY),
          custom(function = "check_xss_string"),
    )]
    pub created_by: String,

    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime::<chrono::Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime::<chrono::Utc>,

}


lazy_static::lazy_static! {
    static ref RE_COLLECTIONDTO_ID: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
}
lazy_static::lazy_static! {
    static ref RE_COLLECTIONDTO_CREATED_BY: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
}

impl CollectionDto {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(id: String, name: String, created_by: String, created_at: chrono::DateTime::<chrono::Utc>, updated_at: chrono::DateTime::<chrono::Utc>, ) -> CollectionDto {
        CollectionDto {
 id,
 name,
 created_by,
 created_at,
 updated_at,
        }
    }
}

/// Converts the CollectionDto value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for CollectionDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("createdBy".to_string()),
            Some(self.created_by.to_string()),

            // Skipping createdAt in query parameter serialization

            // Skipping updatedAt in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CollectionDto value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CollectionDto {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub name: Vec<String>,
            pub created_by: Vec<String>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CollectionDto".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdBy" => intermediate_rep.created_by.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdAt" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updatedAt" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CollectionDto".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CollectionDto {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in CollectionDto".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in CollectionDto".to_string())?,
            created_by: intermediate_rep.created_by.into_iter().next().ok_or_else(|| "createdBy missing in CollectionDto".to_string())?,
            created_at: intermediate_rep.created_at.into_iter().next().ok_or_else(|| "createdAt missing in CollectionDto".to_string())?,
            updated_at: intermediate_rep.updated_at.into_iter().next().ok_or_else(|| "updatedAt missing in CollectionDto".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CollectionDto> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<CollectionDto>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CollectionDto>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for CollectionDto - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<CollectionDto> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CollectionDto as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into CollectionDto - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, PartialOrd,  serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CollectionId(pub String);

impl validator::Validate for CollectionId {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {

        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for CollectionId {
    fn from(x: String) -> Self {
        CollectionId(x)
    }
}

impl std::fmt::Display for CollectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for CollectionId {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(CollectionId(x.to_string()))
    }
}

impl std::convert::From<CollectionId> for String {
    fn from(x: CollectionId) -> Self {
        x.0
    }
}

impl std::ops::Deref for CollectionId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for CollectionId {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



#[derive(Debug, Clone, PartialEq, PartialOrd,  serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ImageId(pub String);

impl validator::Validate for ImageId {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {

        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for ImageId {
    fn from(x: String) -> Self {
        ImageId(x)
    }
}

impl std::fmt::Display for ImageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ImageId {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(ImageId(x.to_string()))
    }
}

impl std::convert::From<ImageId> for String {
    fn from(x: ImageId) -> Self {
        x.0
    }
}

impl std::ops::Deref for ImageId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for ImageId {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ImageTagDto {
    #[serde(rename = "tag")]
          #[validate(nested)]
    pub tag: models::TagDto,

    #[serde(rename = "createdByUser")]
    #[validate(
            regex(path = *RE_IMAGETAGDTO_CREATED_BY_USER),
          custom(function = "check_xss_string"),
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_by_user: Option<String>,

    #[serde(rename = "createdBySourceSite")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub created_by_source_site: Option<models::ImageTagSourceSiteDto>,

    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime::<chrono::Utc>,

}


lazy_static::lazy_static! {
    static ref RE_IMAGETAGDTO_CREATED_BY_USER: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
}

impl ImageTagDto {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(tag: models::TagDto, created_at: chrono::DateTime::<chrono::Utc>, ) -> ImageTagDto {
        ImageTagDto {
 tag,
 created_by_user: None,
 created_by_source_site: None,
 created_at,
        }
    }
}

/// Converts the ImageTagDto value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ImageTagDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping tag in query parameter serialization


            self.created_by_user.as_ref().map(|created_by_user| {
                [
                    "createdByUser".to_string(),
                    created_by_user.to_string(),
                ].join(",")
            }),

            // Skipping createdBySourceSite in query parameter serialization

            // Skipping createdAt in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ImageTagDto value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ImageTagDto {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub tag: Vec<models::TagDto>,
            pub created_by_user: Vec<String>,
            pub created_by_source_site: Vec<models::ImageTagSourceSiteDto>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ImageTagDto".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "tag" => intermediate_rep.tag.push(<models::TagDto as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdByUser" => intermediate_rep.created_by_user.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdBySourceSite" => intermediate_rep.created_by_source_site.push(<models::ImageTagSourceSiteDto as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdAt" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ImageTagDto".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ImageTagDto {
            tag: intermediate_rep.tag.into_iter().next().ok_or_else(|| "tag missing in ImageTagDto".to_string())?,
            created_by_user: intermediate_rep.created_by_user.into_iter().next(),
            created_by_source_site: intermediate_rep.created_by_source_site.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next().ok_or_else(|| "createdAt missing in ImageTagDto".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ImageTagDto> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ImageTagDto>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ImageTagDto>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ImageTagDto - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ImageTagDto> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ImageTagDto as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ImageTagDto - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum ImageTagSourceSiteDto {
    #[serde(rename = "gelbooru")]
    Gelbooru,
}

impl validator::Validate for ImageTagSourceSiteDto
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for ImageTagSourceSiteDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ImageTagSourceSiteDto::Gelbooru => write!(f, "gelbooru"),
        }
    }
}

impl std::str::FromStr for ImageTagSourceSiteDto {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "gelbooru" => std::result::Result::Ok(ImageTagSourceSiteDto::Gelbooru),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ImportSessionDto {
    #[serde(rename = "id")]
    #[validate(
            regex(path = *RE_IMPORTSESSIONDTO_ID),
          custom(function = "check_xss_string"),
    )]
    pub id: String,

    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime::<chrono::Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime::<chrono::Utc>,

    #[serde(rename = "closedAt")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub closed_at: Option<chrono::DateTime::<chrono::Utc>>,

}


lazy_static::lazy_static! {
    static ref RE_IMPORTSESSIONDTO_ID: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
}

impl ImportSessionDto {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(id: String, created_at: chrono::DateTime::<chrono::Utc>, updated_at: chrono::DateTime::<chrono::Utc>, ) -> ImportSessionDto {
        ImportSessionDto {
 id,
 created_at,
 updated_at,
 closed_at: None,
        }
    }
}

/// Converts the ImportSessionDto value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ImportSessionDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),

            // Skipping createdAt in query parameter serialization

            // Skipping updatedAt in query parameter serialization

            // Skipping closedAt in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ImportSessionDto value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ImportSessionDto {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub updated_at: Vec<chrono::DateTime::<chrono::Utc>>,
            pub closed_at: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ImportSessionDto".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdAt" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "updatedAt" => intermediate_rep.updated_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "closedAt" => intermediate_rep.closed_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ImportSessionDto".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ImportSessionDto {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in ImportSessionDto".to_string())?,
            created_at: intermediate_rep.created_at.into_iter().next().ok_or_else(|| "createdAt missing in ImportSessionDto".to_string())?,
            updated_at: intermediate_rep.updated_at.into_iter().next().ok_or_else(|| "updatedAt missing in ImportSessionDto".to_string())?,
            closed_at: intermediate_rep.closed_at.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ImportSessionDto> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ImportSessionDto>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ImportSessionDto>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ImportSessionDto - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ImportSessionDto> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ImportSessionDto as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ImportSessionDto - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, PartialOrd,  serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ImportSessionId(pub String);

impl validator::Validate for ImportSessionId {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {

        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for ImportSessionId {
    fn from(x: String) -> Self {
        ImportSessionId(x)
    }
}

impl std::fmt::Display for ImportSessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ImportSessionId {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(ImportSessionId(x.to_string()))
    }
}

impl std::convert::From<ImportSessionId> for String {
    fn from(x: ImportSessionId) -> Self {
        x.0
    }
}

impl std::ops::Deref for ImportSessionId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for ImportSessionId {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NewImageTagDto {
    #[serde(rename = "tagType")]
          #[validate(nested)]
    pub tag_type: models::TagTypeDto,

    /// max length: 40 characters
    #[serde(rename = "tagName")]
          #[validate(custom(function = "check_xss_string"))]
    pub tag_name: String,

}



impl NewImageTagDto {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(tag_type: models::TagTypeDto, tag_name: String, ) -> NewImageTagDto {
        NewImageTagDto {
 tag_type,
 tag_name,
        }
    }
}

/// Converts the NewImageTagDto value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NewImageTagDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping tagType in query parameter serialization


            Some("tagName".to_string()),
            Some(self.tag_name.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NewImageTagDto value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NewImageTagDto {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub tag_type: Vec<models::TagTypeDto>,
            pub tag_name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NewImageTagDto".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "tagType" => intermediate_rep.tag_type.push(<models::TagTypeDto as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tagName" => intermediate_rep.tag_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NewImageTagDto".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NewImageTagDto {
            tag_type: intermediate_rep.tag_type.into_iter().next().ok_or_else(|| "tagType missing in NewImageTagDto".to_string())?,
            tag_name: intermediate_rep.tag_name.into_iter().next().ok_or_else(|| "tagName missing in NewImageTagDto".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NewImageTagDto> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NewImageTagDto>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NewImageTagDto>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for NewImageTagDto - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NewImageTagDto> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NewImageTagDto as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into NewImageTagDto - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TagDto {
    #[serde(rename = "id")]
    #[validate(
            regex(path = *RE_TAGDTO_ID),
          custom(function = "check_xss_string"),
    )]
    pub id: String,

    #[serde(rename = "tagType")]
          #[validate(nested)]
    pub tag_type: models::TagTypeDto,

    /// max length: 40 characters
    #[serde(rename = "tagName")]
          #[validate(custom(function = "check_xss_string"))]
    pub tag_name: String,

}


lazy_static::lazy_static! {
    static ref RE_TAGDTO_ID: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
}

impl TagDto {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(id: String, tag_type: models::TagTypeDto, tag_name: String, ) -> TagDto {
        TagDto {
 id,
 tag_type,
 tag_name,
        }
    }
}

/// Converts the TagDto value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TagDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),

            // Skipping tagType in query parameter serialization


            Some("tagName".to_string()),
            Some(self.tag_name.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TagDto value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TagDto {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub tag_type: Vec<models::TagTypeDto>,
            pub tag_name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TagDto".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tagType" => intermediate_rep.tag_type.push(<models::TagTypeDto as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tagName" => intermediate_rep.tag_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TagDto".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TagDto {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in TagDto".to_string())?,
            tag_type: intermediate_rep.tag_type.into_iter().next().ok_or_else(|| "tagType missing in TagDto".to_string())?,
            tag_name: intermediate_rep.tag_name.into_iter().next().ok_or_else(|| "tagName missing in TagDto".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TagDto> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TagDto>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TagDto>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for TagDto - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TagDto> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TagDto as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into TagDto - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, PartialOrd,  serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TagId(pub String);

impl validator::Validate for TagId {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {

        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for TagId {
    fn from(x: String) -> Self {
        TagId(x)
    }
}

impl std::fmt::Display for TagId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for TagId {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(TagId(x.to_string()))
    }
}

impl std::convert::From<TagId> for String {
    fn from(x: TagId) -> Self {
        x.0
    }
}

impl std::ops::Deref for TagId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for TagId {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



/// max length: 40 characters
#[derive(Debug, Clone, PartialEq, PartialOrd,  serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TagName(pub String);

impl validator::Validate for TagName {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {

        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for TagName {
    fn from(x: String) -> Self {
        TagName(x)
    }
}

impl std::fmt::Display for TagName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for TagName {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(TagName(x.to_string()))
    }
}

impl std::convert::From<TagName> for String {
    fn from(x: TagName) -> Self {
        x.0
    }
}

impl std::ops::Deref for TagName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for TagName {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum TagTypeDto {
    #[serde(rename = "rating")]
    Rating,
    #[serde(rename = "artist")]
    Artist,
    #[serde(rename = "source")]
    Source,
    #[serde(rename = "character")]
    Character,
    #[serde(rename = "general")]
    General,
}

impl validator::Validate for TagTypeDto
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for TagTypeDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TagTypeDto::Rating => write!(f, "rating"),
            TagTypeDto::Artist => write!(f, "artist"),
            TagTypeDto::Source => write!(f, "source"),
            TagTypeDto::Character => write!(f, "character"),
            TagTypeDto::General => write!(f, "general"),
        }
    }
}

impl std::str::FromStr for TagTypeDto {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "rating" => std::result::Result::Ok(TagTypeDto::Rating),
            "artist" => std::result::Result::Ok(TagTypeDto::Artist),
            "source" => std::result::Result::Ok(TagTypeDto::Source),
            "character" => std::result::Result::Ok(TagTypeDto::Character),
            "general" => std::result::Result::Ok(TagTypeDto::General),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd,  serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UserId(pub String);

impl validator::Validate for UserId {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {

        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for UserId {
    fn from(x: String) -> Self {
        UserId(x)
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UserId {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(UserId(x.to_string()))
    }
}

impl std::convert::From<UserId> for String {
    fn from(x: UserId) -> Self {
        x.0
    }
}

impl std::ops::Deref for UserId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for UserId {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


