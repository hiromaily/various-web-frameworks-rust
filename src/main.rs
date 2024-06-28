use actix_cors::Cors;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use apistos::app::OpenApiWrapper;
use apistos::info::Info;
use apistos::server::Server;
use apistos::spec::Spec;
use apistos::web; // replacement of actix_web::web
use core::time::Duration;
use log::info;
use std::error::Error;

//use env_logger::Builder;
//use log::LevelFilter;

// local
use api_server::args;
use api_server::handlers;
use api_server::registry;
use api_server::routes;
use api_server::toml;

#[actix_web::main]
//async fn main() -> std::io::Result<()> {
async fn main() -> Result<(), impl Error> {
    // initialize log
    env_logger::init();
    // Builder::from_default_env()
    //     .filter_level(LevelFilter::Info)
    //     .init();

    // command line arguments
    let arg = args::get_args();
    dbg!(&arg);

    // load toml
    let file_path = arg.conf;
    let config = toml::load_config(file_path.as_str());
    let config = match config {
        Ok(conf) => conf,
        Err(error) => {
            panic!("fail to load toml file [{}]: {:?}", file_path, error)
        }
    };
    dbg!(&config);

    // registry and get each states
    let reg = registry::Registry::new(config).await.unwrap(); // may panic

    //let global_data = web::Data::new(reg.create_global_state());
    let auth_data = Data::new(reg.create_auth_state());
    let admin_data = Data::new(reg.create_admin_state());
    let app_data = Data::new(reg.create_app_state());

    // In this timing, error would occur if TodoRepository has clone trait as supertrait
    // let client_db: web::Data<Arc<dyn TodoRepository>> =
    //     web::Data::new(Arc::new(TodoRepositoryForMemory::new()));

    // connect to Server

    // [WIP] experimental code
    //let my_app = handlers::basis::MyApp::new(String::from("foobar"));

    // intentionally try various pattern to set routes
    let server = HttpServer::new(move || {
        let api_spec = Spec {
            info: Info {
                title: "todo management API".to_string(),
                version: "1.0.0".to_string(),
                description: Some("todo management API using actix".to_string()),
                ..Default::default()
            },
            servers: vec![Server {
                url: "/api/v3".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .document(api_spec) // requires build() as well
            .wrap(cors)
            .wrap(Logger::default())
            //.app_data(global_data.clone()) // global state
            .app_data(auth_data.clone()) // global state
            .service(
                web::scope("api/v1")
                    //.route("/example", web::get().to(move || my_app.greet()))
                    .route("/health", web::get().to(handlers::basis::health))
                    .service(
                        web::scope("/admin")
                            .app_data(admin_data.clone()) // admin state // maybe divide it into each configuration level
                            .configure(routes::api_admin_login_config_apistos)
                            .configure(routes::api_admin_users_config_apistos)
                            .configure(routes::api_admin_users_id_config_apistos), //.wrap(from_fn(auth_jwt::mw_admin_auth_jwt)),
                    )
                    .service(
                        web::scope("/app")
                            .app_data(app_data.clone()) // app state // maybe divide it into each configuration level
                            .configure(routes::api_app_login_config_apistos)
                            .configure(routes::api_app_users_todo_config_apistos)
                            .configure(routes::api_app_users_todo_id_config_apistos), //.wrap(from_fn(auth_jwt::mw_app_auth_jwt)),
                    ),
            )
            .build("/openapi.json")
    })
    .keep_alive(Duration::from_secs(30));

    // run server
    let host = reg.conf.server.host;
    let port = reg.conf.server.port;
    info!("run server {}:{}", host, port);
    server.bind((host, port))?.run().await
}
