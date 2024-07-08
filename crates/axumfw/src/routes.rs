use crate::handlers;
use crate::middlewares::auth_jwt;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use components::state;
//use tower::ServiceBuilder;
//use tower_http::{add_extension::AddExtensionLayer, trace::TraceLayer};

//.layer(TraceLayer::new_for_http())
//.layer(AddExtensionLayer::new(state.clone()))
//.layer(axum::middleware::from_fn(auth_jwt::print_request_body));

// [Path] /api/v1/admin
// - admin login: [POST] `/admin/login`
// - Show User List: [GET] `/admin/users`
// - Show User: [GET] `/admin/users/{user_id}`
// - Add User: [POST] `/admin/users`
// - Update User: [PUT] `/admin/users/{user_id}`
// - Remove User: [DELETE] `/admin/users/{user_id}`

fn api_admin_login_router(state: state::AuthState) -> Router {
    Router::new()
        .route("/login", post(handlers::admin::admin_login))
        .with_state(state)
    // cfg.service(
    //     web::resource("/login")
    //         .route(web::get().to(HttpResponse::MethodNotAllowed))
    //         .route(web::post().to(handlers::admin::admin_login)),
    // );
}

// Note: In this case, middleware is configured per config
fn api_admin_users_router(auth_state: state::AuthState, admin_state: state::AdminState) -> Router {
    Router::new()
        .route("/users", get(handlers::admin::get_user_list))
        .route("/users", post(handlers::admin::add_user))
        .layer(axum::middleware::from_fn_with_state(
            auth_state.clone(),
            auth_jwt::mw_admin_auth_jwt,
        ))
        .with_state(admin_state)
    // cfg.service(
    //     web::resource("/users")
    //         .route(web::get().to(handlers::admin::get_user_list))
    //         .route(web::post().to(handlers::admin::add_user))
    //         .wrap(from_fn(auth_jwt::mw_admin_auth_jwt)),
    // );
}

// Note: In this case, middleware is configured per config
fn api_admin_users_id_router(
    auth_state: state::AuthState,
    admin_state: state::AdminState,
) -> Router {
    Router::new()
        .route("/users/:user_id", get(handlers::admin::get_user))
        .route("/users/:user_id", put(handlers::admin::update_user))
        .route("/users/:user_id", delete(handlers::admin::delete_user))
        .layer(axum::middleware::from_fn_with_state(
            auth_state.clone(),
            auth_jwt::mw_admin_auth_jwt,
        ))
        .with_state(admin_state)
    // cfg.service(
    //     web::resource("/users/{user_id}")
    //         .route(web::get().to(handlers::admin::get_user))
    //         .route(web::put().to(handlers::admin::update_user))
    //         .route(web::delete().to(handlers::admin::delete_user))
    //         .wrap(from_fn(auth_jwt::mw_admin_auth_jwt)),
    // );
}

fn api_admin_router(auth_state: state::AuthState, admin_state: state::AdminState) -> Router {
    let internal = Router::new()
        .merge(api_admin_login_router(auth_state.clone()))
        .merge(api_admin_users_router(
            auth_state.clone(),
            admin_state.clone(),
        ))
        .merge(api_admin_users_id_router(
            auth_state.clone(),
            admin_state.clone(),
        ));

    Router::new().nest("/admin", internal)
}

// [Path] /api/v1/app
// - client login: [POST] `/app/login`
// - Show Todos for Specific User: [GET] `/app/users/{user_id}/todos`
// - Add Todo: [POST] `/app/users/{user_id}/todos`
// - Update Todo for Specific User: [PUT] `/app/users/{user_id}/todos/{id}`
// - Remove Todo for Specific User: [DELETE] `/app/users/{user_id}/todos/{id}`

fn api_app_login_router(state: state::AuthState) -> Router {
    Router::new()
        .route("/login", post(handlers::app::app_login))
        .with_state(state)
    // cfg.service(
    //     web::resource("/login")
    //         .route(web::get().to(HttpResponse::MethodNotAllowed))
    //         .route(web::post().to(handlers::app::app_login)),
    // );
}

fn api_app_users_todo_router(state: state::AppState) -> Router {
    Router::new()
        .route(
            "/users/:user_id/todos",
            get(handlers::app::get_user_todo_list),
        )
        .route("/users/:user_id/todos", post(handlers::app::add_user_todo))
        .with_state(state)
    // cfg.service(
    //     web::resource("/users/{user_id}/todos")
    //         .route(web::get().to(handlers::app::get_user_todo_list))
    //         .route(web::post().to(handlers::app::add_user_todo))
    //         .wrap(from_fn(auth_jwt::mw_app_auth_jwt)),
    // );
}

fn api_app_users_todo_id_router(state: state::AppState) -> Router {
    Router::new()
        .route(
            "/users/:user_id/todos/:todo_id",
            get(handlers::app::get_user_todo),
        )
        .route(
            "/users/:user_id/todos/:todo_id",
            put(handlers::app::update_user_todo),
        )
        .route(
            "/users/:user_id/todos/:todo_id",
            delete(handlers::app::delete_user_todo),
        )
        .with_state(state)
    // cfg.service(
    //     web::resource("/users/{user_id}/todos/{todo_id}")
    //         .route(web::get().to(handlers::app::get_user_todo))
    //         .route(web::put().to(handlers::app::update_user_todo))
    //         .route(web::delete().to(handlers::app::delete_user_todo))
    //         .wrap(from_fn(auth_jwt::mw_app_auth_jwt)),
    // );
}

fn api_app_router(auth_state: state::AuthState, app_state: state::AppState) -> Router {
    let internal = Router::new()
        .merge(api_app_login_router(auth_state))
        .merge(api_app_users_todo_router(app_state.clone()))
        .merge(api_app_users_todo_id_router(app_state.clone()));

    Router::new().nest("/app", internal)
}

fn health_router() -> Router {
    Router::new().route("/health", get(handlers::basis::health))
}

// for dummy before implementation
fn openapi_router() -> Router {
    Router::new().route("/openapi.json", get(handlers::basis::dummy))
}

pub fn get_api_router(
    auth_state: state::AuthState,
    admin_state: state::AdminState,
    app_state: state::AppState,
) -> Router {
    let api_internal = Router::new()
        .merge(health_router())
        //.merge(openapi_router())
        .merge(api_admin_router(auth_state.clone(), admin_state))
        .merge(api_app_router(auth_state.clone(), app_state.clone()));

    //Router::new().nest("/api/v1", api_internal)

    // temporary
    let api_router = Router::new().nest("/api/v1", api_internal);
    Router::new().merge(openapi_router()).merge(api_router)
}
