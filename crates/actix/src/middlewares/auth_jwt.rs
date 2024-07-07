use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    web, Error as ActixErr,
};
use actix_web_lab::middleware::Next;
use components::entities::users;
use components::errors::CustomError;
use components::state;
use log::{debug, info};
//use std::collections::HashMap;

// refer to
// - https://crates.io/crates/actix-web-lab
// - https://github.com/actix/examples/tree/master/middleware
// - https://github.com/openobserve/openobserve/blob/27eab898aa5b4dd74592299916c1df483282ea4a/src/common/meta/middleware_data.rs#L79

pub async fn mw_admin_auth_jwt(
    auth_data: web::Data<state::AuthState>,
    //_query: web::Query<HashMap<String, String>>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, ActixErr> {
    info!("middleware run");

    // [temporary] skip `/login`, or is_disable==true
    // let is_login_page = req.path().contains("/login");
    // if !is_login_page || (!is_login_page && !auth_data.auth_usecase.is_jwt_disable()) {
    if !auth_data.auth_usecase.is_jwt_disable() {
        // retrieve token from request
        let headers = req.headers();

        let token = match headers.get("authorization") {
            Some(value) => value.to_str().unwrap().strip_prefix("Bearer ").unwrap(),
            None => return Err(ErrorUnauthorized(CustomError::UnauthorizedAccess)),
        };
        debug!("token: {}", token);

        // is_admin must be true
        match auth_data.auth_usecase.validate_token(token) {
            Ok(payload) => {
                // admin only
                if !payload.is_admin {
                    return Err(ErrorUnauthorized(CustomError::UnauthorizedAccess));
                    // return 401
                }
            }
            Err(e) => {
                debug!("token in invalid: {}", e);
                return Err(ErrorUnauthorized(e)); // return 401
            }
        };
    }

    // pre-processing
    next.call(req).await
    // post-processing
}

pub async fn mw_app_auth_jwt(
    auth_data: web::Data<state::AuthState>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, ActixErr> {
    info!("middleware run");

    // [temporary] skip `/login` or is_disable==true
    // let is_login_page = req.path().contains("/login");
    // if !is_login_page || (!is_login_page && !auth_data.auth_usecase.is_jwt_disable()) {
    if !auth_data.auth_usecase.is_jwt_disable() {
        // retrieve token from request
        let headers = req.headers();
        let token = match headers.get("authorization") {
            Some(value) => value.to_str().unwrap().strip_prefix("Bearer ").unwrap(),
            None => return Err(ErrorUnauthorized(CustomError::UnauthorizedAccess)),
        };
        debug!("token: {}", token);

        // let user_id = match extract_user_id(req.path()) {
        //     Ok(user_id) => user_id,
        //     Err(_) => 0,
        // };
        let user_id = users::extract_user_id(req.path()).unwrap_or(0);
        debug!("user_id: {}", user_id);

        match auth_data.auth_usecase.validate_token(token) {
            Ok(payload) => {
                if !payload.is_admin && payload.user_id as i32 != user_id {
                    return Err(ErrorUnauthorized(CustomError::UnauthorizedAccess));
                    // return 401
                }
            }
            Err(e) => {
                debug!("token in invalid: {}", e);
                return Err(ErrorUnauthorized(e)); // return 401
            }
        };
    }

    // pre-processing
    next.call(req).await
    // post-processing
}
