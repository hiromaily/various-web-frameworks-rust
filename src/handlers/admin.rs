use crate::entities::login::LoginResult;
use crate::entities::users;
use crate::handlers::error::ErrorResponse;
use crate::state;
use actix_http::StatusCode;
use actix_web::{web, HttpResponse};
use apistos::api_operation;
use log::info;
use serde_json::json;
use validator::Validate;

/*
 Admin
*/

// change response from impl Responder to HttpResponse

// [post] /login
#[api_operation(summary = "login for admin")]
pub(crate) async fn admin_login(
    auth_data: web::Data<state::AuthState>,
    body: web::Json<users::LoginBody>,
) -> HttpResponse {
    info!("admin_login received");

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
    match auth_data.auth_usecase.login_admin(email, password).await {
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

// [get] /users
#[api_operation(summary = "get user list for admin")]
pub(crate) async fn get_user_list(admin_data: web::Data<state::AdminState>) -> HttpResponse {
    // usecase
    match admin_data.admin_usecase.get_user_list().await {
        Ok(user_list) => HttpResponse::Ok().json(user_list),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Fatal error: {:?}", e),
        }),
    }
}

// [post] /users
#[api_operation(summary = "add user for admin")]
pub async fn add_user(
    admin_data: web::Data<state::AdminState>,
    body: web::Json<users::UserBody>,
) -> HttpResponse {
    // validation
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("request body is invalid: {:?}", e),
        });
    }
    let user_body: users::UserBody = body.into_inner();

    // usecase
    match admin_data.admin_usecase.add_user(user_body).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Fatal error: {:?}", e),
        }),
    }
}

// [get] "/users/{user_id}"
#[api_operation(summary = "get user for admin")]
pub async fn get_user(
    admin_data: web::Data<state::AdminState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let user_id = path.into_inner();

    // usecase
    let res = admin_data.admin_usecase.get_user(user_id).await;
    // response
    // if let Some(user) = res {
    //     HttpResponse::Ok().json(user)
    // }
    match res {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: format!("User with ID {} not found", user_id),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Fatal error: {:?}", e),
        }),
    }
}

// [put] "/users/{user_id}"
#[api_operation(summary = "update user for admin")]
pub async fn update_user(
    admin_data: web::Data<state::AdminState>,
    path: web::Path<i32>,
    body: web::Json<users::UserUpdateBody>,
) -> HttpResponse {
    let user_id = path.into_inner();

    // validate
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("request body is invalid: {:?}", e),
        });
    }
    let user_body: users::UserUpdateBody = body.into_inner();

    // usecase
    match admin_data
        .admin_usecase
        .update_user(user_id, user_body)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: format!("User with ID {} not found", user_id),
        }),
        Err(e) => {
            HttpResponse::BadRequest().json(json!({ "status": "error", "message": e.to_string() }))
        }
    }
}

// [delete] "/users/{user_id}"
#[api_operation(summary = "delete user for admin")]
pub async fn delete_user(
    admin_data: web::Data<state::AdminState>,
    path: web::Path<i32>,
) -> HttpResponse {
    let user_id = path.into_inner();
    // let app_name = &data.app_name;
    // HttpResponse::Ok().body(format!("[delete_user] Hello {app_name}:{user_id}!"))
    match admin_data.admin_usecase.delete_user(user_id).await {
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
