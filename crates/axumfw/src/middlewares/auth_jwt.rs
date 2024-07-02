use axum::{extract::Request, middleware::Next, response::IntoResponse, response::Response};

// WIP
// async fn mw_admin_auth_jwt<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
//     next.run(req).await
//     // let token = match req.headers().get("Authorization") {
//     //     Some(header_value) => header_value.to_str().ok(),
//     //     None => None,
//     // };

//     // if let Some(token) = token {
//     //     match validate_token(token) {
//     //         Ok(claims) => {
//     //             // Attach claims to request extensions if needed
//     //             // req.extensions_mut().insert(claims);
//     //             next.run(req).await
//     //         }
//     //         Err(_) => (StatusCode::UNAUTHORIZED, "Invalid token").into_response(),
//     //     }
//     // } else {
//     //     (StatusCode::UNAUTHORIZED, "Authorization token missing").into_response()
//     // }
// }

pub async fn print_request_body(
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    Ok(next.run(request).await)
}
