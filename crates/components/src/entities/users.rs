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

/*
 Utility user function
*/

// extract user_id from request path
pub fn extract_user_id(path: &str) -> Result<i32, String> {
    let segments: Vec<&str> = path.split('/').collect();

    if let Some(pos) = segments.iter().position(|&s| s == "users") {
        // there is a segment after "users" for the user_id
        if pos + 1 < segments.len() {
            let user_id_str = segments[pos + 1];

            // parse the user_id into an integer
            match user_id_str.parse::<i32>() {
                Ok(user_id) => Ok(user_id),
                Err(_) => Err("Invalid user ID format".into()),
            }
        } else {
            Err("User ID not found in the path".into())
        }
    } else {
        Err("Path does not contain 'users' segment".into())
    }
}
