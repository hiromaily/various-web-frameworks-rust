use components::args;
use components::registry;
use components::toml;
use log::info;

// local
use axumfw::middlewares::common::apply_middleware;
use axumfw::routes;

// refer to
// https://github.com/tokio-rs/axum/blob/main/examples/dependency-injection/src/main.rs

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // command line arguments
    let arg = args::get_args();
    //dbg!(&arg);

    // load toml
    let file_path = arg.conf;
    let config = toml::load_config(file_path.as_str());
    let config = match config {
        Ok(conf) => conf,
        Err(error) => {
            panic!("fail to load toml file [{}]: {:?}", file_path, error)
        }
    };
    //dbg!(&config);

    // registry and get each states
    let reg = registry::Registry::new(config).await.unwrap(); // may panic

    let auth_state = reg.create_auth_state();
    let admin_state = reg.create_admin_state();
    let app_state = reg.create_app_state();

    // get router
    let router = routes::get_api_router(auth_state, admin_state, app_state);
    // apply common middleware
    let router = apply_middleware(router);

    let host = reg.conf.server.host;
    let port = reg.conf.server.port;
    info!("run server {}:{}", host, port);

    // run server with hyper
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
