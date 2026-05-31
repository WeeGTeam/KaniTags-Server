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
    pub struct GetImagePathParams {
                #[validate(
                          regex(path = *RE_GETIMAGEPATHPARAMS_ID),
            )]
                pub id: String,
    }

    lazy_static::lazy_static! {
        static ref RE_GETIMAGEPATHPARAMS_ID: regex::Regex = regex::Regex::new("^[0-9a-f]{16}$").unwrap();
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
        static ref RE_GETTHUMBNAILIMAGEPATHPARAMS_ID: regex::Regex = regex::Regex::new("^[0-9a-f]{16}$").unwrap();
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
        static ref RE_ADDIMAGETAGSPATHPARAMS_ID: regex::Regex = regex::Regex::new("^[0-9a-f]{16}$").unwrap();
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
        static ref RE_GETIMAGETAGSPATHPARAMS_ID: regex::Regex = regex::Regex::new("^[0-9a-f]{16}$").unwrap();
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
pub struct ImageTag {
    #[serde(rename = "tagId")]
    #[validate(
            regex(path = *RE_IMAGETAG_TAG_ID),
          custom(function = "check_xss_string"),
    )]
    pub tag_id: String,

    #[serde(rename = "createdBy")]
    #[validate(
            regex(path = *RE_IMAGETAG_CREATED_BY),
          custom(function = "check_xss_string"),
    )]
    pub created_by: String,

    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime::<chrono::Utc>,

}


lazy_static::lazy_static! {
    static ref RE_IMAGETAG_TAG_ID: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
}
lazy_static::lazy_static! {
    static ref RE_IMAGETAG_CREATED_BY: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
}

impl ImageTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(tag_id: String, created_by: String, created_at: chrono::DateTime::<chrono::Utc>, ) -> ImageTag {
        ImageTag {
 tag_id,
 created_by,
 created_at,
        }
    }
}

/// Converts the ImageTag value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ImageTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("tagId".to_string()),
            Some(self.tag_id.to_string()),


            Some("createdBy".to_string()),
            Some(self.created_by.to_string()),

            // Skipping createdAt in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ImageTag value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ImageTag {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub tag_id: Vec<String>,
            pub created_by: Vec<String>,
            pub created_at: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ImageTag".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "tagId" => intermediate_rep.tag_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdBy" => intermediate_rep.created_by.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "createdAt" => intermediate_rep.created_at.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ImageTag".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ImageTag {
            tag_id: intermediate_rep.tag_id.into_iter().next().ok_or_else(|| "tagId missing in ImageTag".to_string())?,
            created_by: intermediate_rep.created_by.into_iter().next().ok_or_else(|| "createdBy missing in ImageTag".to_string())?,
            created_at: intermediate_rep.created_at.into_iter().next().ok_or_else(|| "createdAt missing in ImageTag".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ImageTag> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ImageTag>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ImageTag>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ImageTag - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ImageTag> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ImageTag as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ImageTag - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ImportSession {
    #[serde(rename = "id")]
    #[validate(
            regex(path = *RE_IMPORTSESSION_ID),
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
    static ref RE_IMPORTSESSION_ID: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
}

impl ImportSession {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(id: String, created_at: chrono::DateTime::<chrono::Utc>, updated_at: chrono::DateTime::<chrono::Utc>, ) -> ImportSession {
        ImportSession {
 id,
 created_at,
 updated_at,
 closed_at: None,
        }
    }
}

/// Converts the ImportSession value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ImportSession {
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

/// Converts Query Parameters representation (style=form, explode=false) to a ImportSession value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ImportSession {
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
                None => return std::result::Result::Err("Missing value while parsing ImportSession".to_string())
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
                    _ => return std::result::Result::Err("Unexpected key while parsing ImportSession".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ImportSession {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in ImportSession".to_string())?,
            created_at: intermediate_rep.created_at.into_iter().next().ok_or_else(|| "createdAt missing in ImportSession".to_string())?,
            updated_at: intermediate_rep.updated_at.into_iter().next().ok_or_else(|| "updatedAt missing in ImportSession".to_string())?,
            closed_at: intermediate_rep.closed_at.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ImportSession> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ImportSession>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ImportSession>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ImportSession - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ImportSession> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ImportSession as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ImportSession - {err}"#))
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
pub struct NewImageTag {
    #[serde(rename = "tagType")]
          #[validate(nested)]
    pub tag_type: models::TagType,

    /// max length: 40 characters
    #[serde(rename = "tagName")]
          #[validate(custom(function = "check_xss_string"))]
    pub tag_name: String,

}



impl NewImageTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(tag_type: models::TagType, tag_name: String, ) -> NewImageTag {
        NewImageTag {
 tag_type,
 tag_name,
        }
    }
}

/// Converts the NewImageTag value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NewImageTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping tagType in query parameter serialization


            Some("tagName".to_string()),
            Some(self.tag_name.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NewImageTag value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NewImageTag {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub tag_type: Vec<models::TagType>,
            pub tag_name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NewImageTag".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "tagType" => intermediate_rep.tag_type.push(<models::TagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tagName" => intermediate_rep.tag_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NewImageTag".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NewImageTag {
            tag_type: intermediate_rep.tag_type.into_iter().next().ok_or_else(|| "tagType missing in NewImageTag".to_string())?,
            tag_name: intermediate_rep.tag_name.into_iter().next().ok_or_else(|| "tagName missing in NewImageTag".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NewImageTag> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NewImageTag>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NewImageTag>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for NewImageTag - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NewImageTag> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NewImageTag as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into NewImageTag - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Tag {
    #[serde(rename = "id")]
    #[validate(
            regex(path = *RE_TAG_ID),
          custom(function = "check_xss_string"),
    )]
    pub id: String,

    #[serde(rename = "tagType")]
          #[validate(nested)]
    pub tag_type: models::TagType,

    /// max length: 40 characters
    #[serde(rename = "tagName")]
          #[validate(custom(function = "check_xss_string"))]
    pub tag_name: String,

}


lazy_static::lazy_static! {
    static ref RE_TAG_ID: regex::Regex = regex::Regex::new("^[0-9]+$").unwrap();
}

impl Tag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(id: String, tag_type: models::TagType, tag_name: String, ) -> Tag {
        Tag {
 id,
 tag_type,
 tag_name,
        }
    }
}

/// Converts the Tag value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Tag {
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

/// Converts Query Parameters representation (style=form, explode=false) to a Tag value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Tag {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub tag_type: Vec<models::TagType>,
            pub tag_name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Tag".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tagType" => intermediate_rep.tag_type.push(<models::TagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tagName" => intermediate_rep.tag_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Tag".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Tag {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in Tag".to_string())?,
            tag_type: intermediate_rep.tag_type.into_iter().next().ok_or_else(|| "tagType missing in Tag".to_string())?,
            tag_name: intermediate_rep.tag_name.into_iter().next().ok_or_else(|| "tagName missing in Tag".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Tag> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Tag>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Tag>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Tag - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Tag> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Tag as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Tag - {err}"#))
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
pub enum TagType {
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

impl validator::Validate for TagType
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for TagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TagType::Rating => write!(f, "rating"),
            TagType::Artist => write!(f, "artist"),
            TagType::Source => write!(f, "source"),
            TagType::Character => write!(f, "character"),
            TagType::General => write!(f, "general"),
        }
    }
}

impl std::str::FromStr for TagType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "rating" => std::result::Result::Ok(TagType::Rating),
            "artist" => std::result::Result::Ok(TagType::Artist),
            "source" => std::result::Result::Ok(TagType::Source),
            "character" => std::result::Result::Ok(TagType::Character),
            "general" => std::result::Result::Ok(TagType::General),
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


