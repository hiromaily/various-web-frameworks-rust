use components::args;
use components::registry;
use components::toml;
//use env_logger::Builder;
//use log::LevelFilter;

// local
use actix::servers;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    //async fn main() -> std::io::Result<()> {
    //async fn main() -> Result<(), impl Error> {

    // initialize log
    env_logger::init();
    // Builder::from_default_env()
    //     .filter_level(LevelFilter::Info)
    //     .init();

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

    // let auth_data = Data::new(reg.create_auth_state());
    // let admin_data = Data::new(reg.create_admin_state());
    // let app_data = Data::new(reg.create_app_state());

    // run server
    let server = servers::run_server(
        reg.create_auth_state(),
        reg.create_admin_state(),
        reg.create_app_state(),
        reg.conf.server.host,
        reg.conf.server.port,
    )
    .await?;

    server.await
}
