use crate::handlers;
use actix_cors::Cors;
use actix_web::{dev::Server, middleware::Logger, web::Data, App, HttpServer};
use components::state;
cfg_if! {
    if #[cfg(feature = "openapi")] {
        use apistos::info::Info;
        use apistos::server::Server as ApistosServer;
        use apistos::spec::Spec;
        use apistos::app::OpenApiWrapper;
        use apistos::web; // replacement of actix_web::web
        use crate::routes::apistos as route;
    } else {
        use actix_web::web;
        use crate::routes::actix as route;
    }
}
use cfg_if::cfg_if;
use core::time::Duration;

fn create_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .supports_credentials()
}

cfg_if! {
    if #[cfg(feature = "openapi")] {
        fn create_api_spec() -> Spec {
            Spec {
                info: Info {
                    title: "todo management API".to_string(),
                    version: "1.0.0".to_string(),
                    description: Some("todo management API using actix".to_string()),
                    ..Default::default()
                },
                servers: vec![ApistosServer {
                    url: "/api/v3".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            }
        }

        pub async fn run_server(
            auth_state: state::AuthState,
            admin_state: state::AdminState,
            app_state: state::AppState,
            host: String,
            port: u16,
        ) -> std::io::Result<Server> {
            let server = HttpServer::new(move || {
                App::new()
                    .document(create_api_spec()) // requires build() as well
                    .wrap(create_cors())
                    .wrap(Logger::default())
                    .app_data(Data::new(auth_state.clone())) // global state
                    .service(
                        web::scope("api/v1")
                            .route("/health", web::get().to(handlers::basis::health))
                            .service(
                                web::scope("/admin")
                                    .app_data(Data::new(admin_state.clone())) // admin state // maybe divide it into each configuration level
                                    .configure(route::api_admin_login_config)
                                    .configure(route::api_admin_users_config)
                                    .configure(route::api_admin_users_id_config), //.wrap(from_fn(auth_jwt::mw_admin_auth_jwt)),
                            )
                            .service(
                                web::scope("/app")
                                    .app_data(Data::new(app_state.clone())) // app state // maybe divide it into each configuration level
                                    .configure(route::api_app_login_config)
                                    .configure(route::api_app_users_todo_config)
                                    .configure(route::api_app_users_todo_id_config), //.wrap(from_fn(auth_jwt::mw_app_auth_jwt)),
                            ),
                    )
                    .build("/openapi.json")
            })
            .keep_alive(Duration::from_secs(30));

            Ok(server.bind((host, port))?.run())
        }
    } else {
        pub async fn run_server(
            auth_state: state::AuthState,
            admin_state: state::AdminState,
            app_state: state::AppState,
            host: String,
            port: u16,
        ) -> std::io::Result<Server> {
            let server = HttpServer::new(move || {
                App::new()
                    .wrap(create_cors())
                    .wrap(Logger::default())
                    .app_data(Data::new(auth_state.clone())) // global state
                    .service(
                        web::scope("api/v1")
                            .route("/health", web::get().to(handlers::basis::health))
                            .service(
                                web::scope("/admin")
                                    .app_data(Data::new(admin_state.clone())) // admin state // maybe divide it into each configuration level
                                    .configure(route::api_admin_login_config)
                                    .configure(route::api_admin_users_config)
                                    .configure(route::api_admin_users_id_config), //.wrap(from_fn(auth_jwt::mw_admin_auth_jwt)),
                            )
                            .service(
                                web::scope("/app")
                                    .app_data(Data::new(app_state.clone())) // app state // maybe divide it into each configuration level
                                    .configure(route::api_app_login_config)
                                    .configure(route::api_app_users_todo_config)
                                    .configure(route::api_app_users_todo_id_config), //.wrap(from_fn(auth_jwt::mw_app_auth_jwt)),
                            ),
                    )
            })
            .keep_alive(Duration::from_secs(30));

            Ok(server.bind((host, port))?.run())
        }
    }
}
