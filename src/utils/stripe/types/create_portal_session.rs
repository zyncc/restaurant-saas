use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePortalSessionResponse {
    pub id: String,
    pub object: String,
    pub configuration: String,
    pub created: i64,
    pub customer: String,
    pub flow: Value,
    pub livemode: bool,
    pub locale: Value,
    pub on_behalf_of: Value,
    pub return_url: String,
    pub url: String,
}
