use axum::{
    extract::{Path, Request, State}, //Query
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use components::state;
use log::debug;

pub async fn mw_admin_auth_jwt(
    State(auth_state): State<state::AuthState>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    debug!("mw_admin_auth_jwt is called");

    if !auth_state.auth_usecase.is_jwt_disable() {
        // retrieve token from request
        let headers = req.headers();

        let token = match headers.get("authorization") {
            Some(value) => value.to_str().unwrap().strip_prefix("Bearer ").unwrap(),
            None => return Err(StatusCode::UNAUTHORIZED),
        };
        debug!("token: {}", token);

        // is_admin must be true
        match auth_state.auth_usecase.validate_token(token) {
            Ok(payload) => {
                // admin only
                if !payload.is_admin {
                    return Err(StatusCode::UNAUTHORIZED);
                    // return 401
                }
            }
            Err(e) => {
                debug!("token in invalid: {}", e);
                return Err(StatusCode::UNAUTHORIZED); // return 401
            }
        };
    }

    Ok(next.run(req).await)
}

pub async fn mw_app_auth_jwt(
    State(auth_state): State<state::AuthState>,
    Path(user_id): Path<i32>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    debug!("mw_app_auth_jwt is called");

    if !auth_state.auth_usecase.is_jwt_disable() {
        // retrieve token from request
        let headers = req.headers();
        let token = match headers.get("authorization") {
            Some(value) => value.to_str().unwrap().strip_prefix("Bearer ").unwrap(),
            None => return Err(StatusCode::UNAUTHORIZED),
        };
        debug!("token: {}", token);

        //let user_id = users::extract_user_id(req.path()).unwrap_or(0);
        debug!("user_id: {}", user_id);

        match auth_state.auth_usecase.validate_token(token) {
            Ok(payload) => {
                if !payload.is_admin && payload.user_id as i32 != user_id {
                    return Err(StatusCode::UNAUTHORIZED);
                    // return 401
                }
            }
            Err(e) => {
                debug!("token in invalid: {}", e);
                return Err(StatusCode::UNAUTHORIZED); // return 401
            }
        };
    }

    Ok(next.run(req).await)
}
