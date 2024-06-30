use crate::handlers::error::AppError;
use axum::{extract::Path, extract::State, http::StatusCode, Json};
use components::entities::login::LoginResult;
use components::entities::{todos, users};
use components::schemas::todos as db_todos;
use components::state;
use validator::Validate;

// /*
//  App
// */
// [post] /login
pub async fn app_login(
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
    match auth_state.auth_usecase.login(email, password).await {
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
            //HttpResponse::Ok().json(json!({ "status": "success", "message": "Login successful" }))
        }
        Ok(None) => Err(AppError::Unauthorized("user is not found".into())),
        Err(e) => Err(AppError::InternalServerError(format!(
            "Fatal error: {:?}",
            e
        ))),
    }
}

// [get] /users/{user_id}/todos
pub async fn get_user_todo_list(
    State(app_state): State<state::AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<db_todos::Model>>, AppError> {
    // usecase
    match app_state.app_usecase.get_user_todo_list(user_id).await {
        Ok(todo_list) => Ok(Json(todo_list)),
        Err(e) => Err(AppError::InternalServerError(format!(
            "Fatal error: {:?}",
            e
        ))),
    }
}

// [post] /users/{user_id}/todos
pub async fn add_user_todo(
    State(app_state): State<state::AppState>,
    Path(user_id): Path<i32>,
    Json(body): Json<todos::TodoBody>,
) -> Result<Json<db_todos::Model>, AppError> {
    // validation
    if let Err(e) = body.validate() {
        return Err(AppError::BadRequest(format!(
            "request body is invalid: {:?}",
            e
        )));
    }

    // usecase
    match app_state.app_usecase.add_user_todo(user_id, body).await {
        Ok(todo) => Ok(Json(todo)),
        Err(e) => Err(AppError::InternalServerError(format!(
            "Fatal error: {:?}",
            e
        ))),
    }
}

// [get] "/users/{user_id}/todos/{todo_id}"
pub async fn get_user_todo(
    State(app_state): State<state::AppState>,
    Path((user_id, todo_id)): Path<(i32, i32)>,
) -> Result<Json<db_todos::Model>, AppError> {
    // usecase
    let res = app_state.app_usecase.get_user_todo(user_id, todo_id).await;
    // response
    match res {
        Ok(Some(todo)) => Ok(Json(todo)),
        Ok(None) => Err(AppError::NotFound(format!(
            "Todo with ID {} not found",
            todo_id
        ))),
        Err(e) => Err(AppError::InternalServerError(format!(
            "Fatal error: {:?}",
            e
        ))),
    }
}

// [put] "/users/{user_id}/todos/{todo_id}"
pub async fn update_user_todo(
    State(app_state): State<state::AppState>,
    Path((user_id, todo_id)): Path<(i32, i32)>,
    Json(body): Json<todos::TodoUpdateBody>,
) -> Result<Json<db_todos::Model>, AppError> {
    // validate
    if let Err(e) = body.validate() {
        return Err(AppError::BadRequest(format!(
            "request body is invalid: {:?}",
            e
        )));
    }
    // usecase
    match app_state
        .app_usecase
        .update_user_todo(user_id, todo_id, body)
        .await
    {
        Ok(Some(todo)) => Ok(Json(todo)),
        Ok(None) => Err(AppError::NotFound(format!(
            "Todo with ID {} not found",
            todo_id
        ))),
        Err(e) => Err(AppError::BadRequest(format!(
            "request body is invalid: {:?}",
            e
        ))),
    }
}

// [delete] // [post] "/users/{user_id}/todos/{todo_id}"
pub async fn delete_user_todo(
    State(app_state): State<state::AppState>,
    Path((user_id, todo_id)): Path<(i32, i32)>,
) -> Result<StatusCode, AppError> {
    match app_state
        .app_usecase
        .delete_user_todo(user_id, todo_id)
        .await
    {
        Ok(0) => Err(AppError::NotFound(format!(
            "Todo with ID {} not found",
            todo_id
        ))),
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(AppError::InternalServerError(format!(
            "Fatal error: {:?}",
            e
        ))),
    }
}
