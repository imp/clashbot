use std::str::FromStr;

use thiserror::Error;

use super::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Error, Serialize, Deserialize)]
#[error("ClientError: {message}: ({reason})")]
#[serde(default)]
pub struct ClientError {
    pub reason: String,
    pub message: String,
    pub r#type: String,
    pub detail: String,
}

impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        Self {
            reason: "reqwest::Error".to_string(),
            message: error.to_string(),
            r#type: "".to_string(),
            detail: "".to_string(),
        }
    }
}

impl ClientError {
    pub(crate) fn parse_error(error: <reqwest::Url as FromStr>::Err) -> Self {
        Self {
            reason: "ParseError".to_string(),
            message: error.to_string(),
            r#type: "".to_string(),
            detail: "".to_string(),
        }
    }
}
