// use axum::{
//     extract::{Path, State},
//     http::StatusCode,
//     routing::{get, post},
//     Json, Router,
// };
// use components::state;

// pub fn api_admin_login_config(state: state::AuthState) {
//     Router::new()
//         .route("/users/:id", get(get_user_dyn))
//         .route("/users", post(create_user_dyn))
//         .with_state(AppStateDyn {
//             user_repo: Arc::new(user_repo.clone()),
//         })

//     // cfg.service(
//     //     web::resource("/login")
//     //         .route(web::get().to(HttpResponse::MethodNotAllowed))
//     //         .route(web::post().to(handlers::admin::admin_login)),
//     // );
// }

// // Note: In this case, middleware is configured per config
// pub fn api_admin_users_config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/users")
//             .route(web::get().to(handlers::admin::get_user_list))
//             .route(web::post().to(handlers::admin::add_user))
//             .wrap(from_fn(auth_jwt::mw_admin_auth_jwt)),
//     );
// }

// // Note: In this case, middleware is configured per config
// pub fn api_admin_users_id_config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/users/{user_id}")
//             .route(web::get().to(handlers::admin::get_user))
//             .route(web::put().to(handlers::admin::update_user))
//             .route(web::delete().to(handlers::admin::delete_user))
//             .wrap(from_fn(auth_jwt::mw_admin_auth_jwt)),
//     );
// }
