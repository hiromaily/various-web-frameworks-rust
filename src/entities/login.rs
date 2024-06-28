use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct LoginResult {
    pub message: String,
    pub token: Option<String>,
}
