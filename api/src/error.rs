use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientError {
    pub reason: String,
    pub message: String,
    pub r#type: String,
    pub detail: String,
}
