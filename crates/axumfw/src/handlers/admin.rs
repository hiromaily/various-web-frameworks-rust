use crate::handlers::error::AppError;
use axum::{extract::Path, extract::State, http::StatusCode, Json};
use components::entities::login::LoginResult;
use components::entities::users;
use components::schemas::users as db_users;
use components::state;
use validator::Validate;

/*
 Admin
*/

// [post] /login
pub(crate) async fn admin_login(
    State(auth_state): State<state::AuthState>,
    body: Json<users::LoginBody>,
) -> Result<Json<LoginResult>, AppError> {
    // validation
    if let Err(e) = body.validate() {
        return Err(AppError::BadRequest(format!(
            "request body is invalid: {:?}",
            e
        )));
    }

    // Extract the email and password
    let email = &body.email;
    let password = &body.password;

    // authentication usecase
    match auth_state.auth_usecase.login_admin(email, password).await {
        Ok(Some(user)) => {
            // return access key
            match auth_state.auth_usecase.generate_token(
                user.id,
                user.email.as_str(),
                user.is_admin,
            ) {
                Ok(token) => Ok(Json(LoginResult {
                    message: "Login successful".into(),
                    token: Some(token),
                })),
                Err(e) => Err(AppError::InternalServerError(format!(
                    "Fatal error: {:?}",
                    e
                ))),
            }
        }
        // Ok(None) => HttpResponse::Unauthorized().json(LoginResult {
        //     message: "user is not found".into(),
        //     token: None,
        // }),
        Ok(None) => Err(AppError::Unauthorized("user is not found".into())),
        Err(e) => Err(AppError::InternalServerError(format!(
            "Fatal error: {:?}",
            e
        ))),
    }
}

// [get] /users
pub(crate) async fn get_user_list(
    State(admin_state): State<state::AdminState>,
) -> Result<Json<Vec<db_users::Model>>, AppError> {
    // usecase
    match admin_state.admin_usecase.get_user_list().await {
        Ok(user_list) => Ok(Json(user_list)),
        Err(e) => Err(AppError::InternalServerError(format!(
            "Fatal error: {:?}",
            e
        ))),
    }
}

// [post] /users
pub async fn add_user(
    State(admin_state): State<state::AdminState>,
    Json(body): Json<users::UserBody>,
) -> Result<Json<db_users::Model>, AppError> {
    // validation
    if let Err(e) = body.validate() {
        return Err(AppError::BadRequest(format!(
            "request body is invalid: {:?}",
            e
        )));
    }

    // usecase
    match admin_state.admin_usecase.add_user(body).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(AppError::InternalServerError(format!(
            "Fatal error: {:?}",
            e
        ))),
    }
}

// [get] "/users/{user_id}"
pub async fn get_user(
    State(admin_state): State<state::AdminState>,
    Path(user_id): Path<i32>,
) -> Result<Json<db_users::Model>, AppError> {
    // usecase
    let res = admin_state.admin_usecase.get_user(user_id).await;
    match res {
        Ok(Some(user)) => Ok(Json(user)),
        // Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
        //     error: format!("User with ID {} not found", user_id),
        // }),
        Ok(None) => Err(AppError::NotFound(format!(
            "User with ID {} not found",
            user_id
        ))),
        Err(e) => Err(AppError::InternalServerError(format!(
            "Fatal error: {:?}",
            e
        ))),
    }
}

// [put] "/users/{user_id}"
pub async fn update_user(
    State(admin_state): State<state::AdminState>,
    Path(user_id): Path<i32>,
    Json(body): Json<users::UserUpdateBody>,
) -> Result<Json<db_users::Model>, AppError> {
    // validate
    if let Err(e) = body.validate() {
        return Err(AppError::BadRequest(format!(
            "request body is invalid: {:?}",
            e
        )));
    }

    // usecase
    match admin_state.admin_usecase.update_user(user_id, body).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(AppError::NotFound(format!(
            "User with ID {} not found",
            user_id
        ))),
        Err(e) => Err(AppError::BadRequest(format!(
            "request body is invalid: {:?}",
            e
        ))),
    }
}

// [delete] "/users/{user_id}"
pub async fn delete_user(
    State(admin_state): State<state::AdminState>,
    Path(user_id): Path<i32>,
) -> Result<StatusCode, AppError> {
    match admin_state.admin_usecase.delete_user(user_id).await {
        Ok(0) => Err(AppError::NotFound(format!(
            "User with ID {} not found",
            user_id
        ))),
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError::InternalServerError(format!(
            "Fatal error: {:?}",
            e
        ))),
    }
}
