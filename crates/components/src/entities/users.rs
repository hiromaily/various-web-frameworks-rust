use crate::schemas::users::Model;
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

/*
 HTTP request body and response
*/

#[derive(Debug, Serialize, Deserialize, Validate, ApiComponent, JsonSchema)]
pub struct LoginBody {
    #[validate(length(min = 8, max = 50))]
    pub email: String,
    #[validate(length(min = 10, max = 20))]
    pub password: String,
}

#[derive(
    Debug, Serialize, Deserialize, Validate, Clone, PartialEq, Eq, ApiComponent, JsonSchema,
)]
pub struct UserBody {
    #[validate(length(min = 1, max = 50))]
    pub first_name: String,
    #[validate(length(min = 1, max = 50))]
    pub last_name: String,
    #[validate(length(min = 8, max = 50))]
    pub email: String,
    #[validate(length(min = 10, max = 20))]
    pub password: String,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate, ApiComponent, JsonSchema)]
pub struct UserUpdateBody {
    #[validate(length(min = 1, max = 50))]
    pub first_name: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub last_name: Option<String>,
    #[validate(length(min = 8, max = 50))]
    pub email: Option<String>,
    #[validate(length(min = 10, max = 20))]
    pub password: Option<String>,
    pub is_admin: Option<bool>,
}

#[derive(ApiComponent, JsonSchema)]
pub struct UserModel {
    pub model: Model,
}
