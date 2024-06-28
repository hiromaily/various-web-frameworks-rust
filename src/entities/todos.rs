use crate::schemas::sea_orm_active_enums::TodoStatus;
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

/*
 HTTP request body and response
*/

fn validate_status(status: &str) -> Result<(), ValidationError> {
    match status {
        "canceled" | "doing" | "done" | "pending" => Ok(()),
        _ => Err(ValidationError::new("invalid status")),
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, ApiComponent, JsonSchema)]
pub struct TodoBody {
    #[validate(length(min = 1, max = 50))]
    pub title: String,
    #[validate(length(min = 1, max = 200))]
    pub description: Option<String>,
    #[validate(length(min = 1), custom(function = "validate_status"))]
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ApiComponent, JsonSchema)]
pub struct TodoUpdateBody {
    #[validate(length(min = 1, max = 50))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 200))]
    pub description: Option<String>,
    #[validate(length(min = 1), custom(function = "validate_status"))]
    pub status: Option<String>,
}

/// extension for TodoStatus
///
/// # Examples
///
/// ```
/// use api_server::schemas::sea_orm_active_enums::TodoStatus;
///
/// let status_str = TodoStatus::Doing.to_string();
/// println!("Status as string: {}", status_str);
///
/// let status_from_str = "done".parse::<TodoStatus>().unwrap();
/// println!("Status from string: {:?}", status_from_str);
/// ```
impl std::str::FromStr for TodoStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<TodoStatus, Self::Err> {
        match input {
            "canceled" => Ok(TodoStatus::Canceled),
            "doing" => Ok(TodoStatus::Doing),
            "done" => Ok(TodoStatus::Done),
            "pending" => Ok(TodoStatus::Pending),
            _ => Err(()),
        }
    }
}

impl TodoStatus {
    pub fn to_string(&self) -> &str {
        match self {
            TodoStatus::Canceled => "canceled",
            TodoStatus::Doing => "doing",
            TodoStatus::Done => "done",
            TodoStatus::Pending => "pending",
        }
    }
}
