use crate::entities::login::LoginResult;
use crate::entities::{todos, users};
use crate::handlers::error::ErrorResponse;
use crate::state;
use actix_http::StatusCode;
use actix_web::{web, HttpResponse};
use apistos::api_operation;
use serde_json::json;
use validator::Validate;

/*
 App
*/

// [post] /login
#[api_operation(summary = "login for app")]
pub async fn app_login(
    auth_data: web::Data<state::AuthState>,
    body: web::Json<users::LoginBody>,
) -> HttpResponse {
    // validation
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("request body is invalid: {:?}", e),
        });
    }

    // Extract the email and password
    let email = &body.email;
    let password = &body.password;

    // authentication usecase
    match auth_data.auth_usecase.login(email, password).await {
        Ok(Some(user)) => {
            // return access key
            match auth_data
                .auth_usecase
                .generate_token(user.id, user.email.as_str(), user.is_admin)
            {
                Ok(token) => HttpResponse::Ok().json(LoginResult {
                    message: "Login successful".into(),
                    token: Some(token),
                }),
                Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
                    error: format!("Fatal error: {:?}", e),
                }),
            }
            //HttpResponse::Ok().json(json!({ "status": "success", "message": "Login successful" }))
        }
        Ok(None) => HttpResponse::Unauthorized().json(LoginResult {
            message: "user is not found".into(),
            token: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Fatal error: {:?}", e),
        }),
    }
}

// [get] /users/{user_id}/todos
#[api_operation(summary = "get user todo list")]
pub async fn get_user_todo_list(
    app_data: web::Data<state::AppState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let user_id = path.into_inner();

    // usecase
    match app_data.app_usecase.get_user_todo_list(user_id).await {
        Ok(todo_list) => HttpResponse::Ok().json(todo_list),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Fatal error: {:?}", e),
        }),
    }
}

// [post] /users/{user_id}/todos
#[api_operation(summary = "add user todo")]
pub async fn add_user_todo(
    app_data: web::Data<state::AppState>,
    path: web::Path<i32>,
    body: web::Json<todos::TodoBody>,
) -> HttpResponse {
    let user_id = path.into_inner();

    // validation
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("request body is invalid: {:?}", e),
        });
    }
    let todo_body: todos::TodoBody = body.into_inner();

    // usecase
    match app_data.app_usecase.add_user_todo(user_id, todo_body).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Fatal error: {:?}", e),
        }),
    }
}

// [get] "/users/{user_id}/todos/{todo_id}"
#[api_operation(summary = "get user todo")]
pub async fn get_user_todo(
    app_data: web::Data<state::AppState>,
    path: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (user_id, todo_id) = path.into_inner();

    // usecase
    let res = app_data.app_usecase.get_user_todo(user_id, todo_id).await;
    // response
    match res {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: format!("User with ID {} not found", user_id),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Fatal error: {:?}", e),
        }),
    }
}

// [put] "/users/{user_id}/todos/{todo_id}"
#[api_operation(summary = "update user todo")]
pub async fn update_user_todo(
    app_data: web::Data<state::AppState>,
    path: web::Path<(i32, i32)>,
    body: web::Json<todos::TodoUpdateBody>,
) -> HttpResponse {
    let (user_id, todo_id) = path.into_inner();

    // validate
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("request body is invalid: {:?}", e),
        });
    }
    let todo_body: todos::TodoUpdateBody = body.into_inner();

    // usecase
    match app_data
        .app_usecase
        .update_user_todo(user_id, todo_id, todo_body)
        .await
    {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: format!("User with ID {} not found", user_id),
        }),
        Err(e) => {
            HttpResponse::BadRequest().json(json!({ "status": "error", "message": e.to_string() }))
        }
    }
}

// [delete] // [post] "/users/{user_id}/todos/{todo_id}"
#[api_operation(summary = "elete user todo")]
pub async fn delete_user_todo(
    app_data: web::Data<state::AppState>,
    path: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (user_id, todo_id) = path.into_inner();
    match app_data
        .app_usecase
        .delete_user_todo(user_id, todo_id)
        .await
    {
        Ok(0) => HttpResponse::NotFound().json(ErrorResponse {
            error: format!("User with ID {} not found", user_id),
        }),
        Ok(_) => {
            //HttpResponse::Ok().json(json!({ "status": "success", "message": "Delete successful" }))
            HttpResponse::new(StatusCode::NO_CONTENT)
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Fatal error: {:?}", e),
        }),
    }
}
