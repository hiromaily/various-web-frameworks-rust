use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct ErrorResponse {
    pub error: String,
}
