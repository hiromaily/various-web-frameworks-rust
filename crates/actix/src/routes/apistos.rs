use crate::handlers;
use crate::middlewares::auth_jwt;
use actix_web_lab::middleware::from_fn;
use apistos::web;

/*
 apistos version
*/

// [Path] /api/v1/admin
// - admin login: [POST] `/admin/login`
// - Show User List: [GET] `/admin/users`
// - Show User: [GET] `/admin/users/{user_id}`
// - Add User: [POST] `/admin/users`
// - Update User: [PUT] `/admin/users/{user_id}`
// - Remove User: [DELETE] `/admin/users/{user_id}`

pub fn api_admin_login_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            //.route(web::get().to(HttpResponse::MethodNotAllowed))
            .route(web::post().to(handlers::admin::admin_login)),
    );
}

// Note: In this case, middleware is configured per config
pub fn api_admin_users_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(handlers::admin::get_user_list))
            .route(web::post().to(handlers::admin::add_user))
            .wrap(from_fn(auth_jwt::mw_admin_auth_jwt)),
    );
}

// Note: In this case, middleware is configured per config
pub fn api_admin_users_id_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users/{user_id}")
            .route(web::get().to(handlers::admin::get_user))
            .route(web::put().to(handlers::admin::update_user))
            .route(web::delete().to(handlers::admin::delete_user))
            .wrap(from_fn(auth_jwt::mw_admin_auth_jwt)),
    );
}

// [Path] /api/v1/app
// - client login: [POST] `/app/login`
// - Show Todos for Specific User: [GET] `/app/users/{user_id}/todos`
// - Add Todo: [POST] `/app/users/{user_id}/todos`
// - Update Todo for Specific User: [PUT] `/app/users/{user_id}/todos/{id}`
// - Remove Todo for Specific User: [DELETE] `/app/users/{user_id}/todos/{id}`

pub fn api_app_login_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            //.route(web::get().to(HttpResponse::MethodNotAllowed))
            .route(web::post().to(handlers::app::app_login)),
    );
}

pub fn api_app_users_todo_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users/{user_id}/todos")
            .route(web::get().to(handlers::app::get_user_todo_list))
            .route(web::post().to(handlers::app::add_user_todo))
            .wrap(from_fn(auth_jwt::mw_app_auth_jwt)),
    );
}

pub fn api_app_users_todo_id_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users/{user_id}/todos/{todo_id}")
            .route(web::get().to(handlers::app::get_user_todo))
            .route(web::put().to(handlers::app::update_user_todo))
            .route(web::delete().to(handlers::app::delete_user_todo))
            .wrap(from_fn(auth_jwt::mw_app_auth_jwt)),
    );
}
