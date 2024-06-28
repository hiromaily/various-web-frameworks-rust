use thiserror::Error;

/*
e.g
 ```
 return Err(CustomError::UnauthorizedAccess);
 return Err(CustomError::Other(e.into()));
 ```
*/
#[derive(Debug, Error)]
pub enum CustomError {
    #[error("Unauthorized access")]
    UnauthorizedAccess,
    #[error("invalid data")]
    InvalidData,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
