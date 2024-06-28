use sea_orm::{ConnectOptions, Database, DbErr};
use std::time::Duration;

// refer to: https://www.sea-ql.org/sea-orm-tutorial/ch01-01-project-setup.html
pub async fn get_conn(
    user: &str,
    password: &str,
    host: &str,
    db_name: &str,
) -> Result<sea_orm::DatabaseConnection, DbErr> {
    let db_url = format!("postgresql://{user}:{password}@{host}/{db_name}");
    let mut opt = ConnectOptions::new(db_url);
    // TODO: define at config toml file
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(5))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);
    //.set_schema_search_path("my_schema"); // Setting default PostgreSQL schema
    let conn = Database::connect(opt).await?;

    //let conn: sea_orm::DatabaseConnection = Database::connect(db_url).await?;
    // let db = &match conn.get_database_backend() {
    //     DbBackend::Postgres => {
    //         db.execute(Statement::from_string(
    //             db.get_database_backend(),
    //             format!("DROP DATABASE IF EXISTS \"{}\";", *db_name),
    //         ))
    //         .await?;
    //         db.execute(Statement::from_string(
    //             db.get_database_backend(),
    //             format!("CREATE DATABASE \"{}\";", *db_name),
    //         ))
    //         .await?;

    //         Database::connect(format!("{}/{}", db_url, *db_name)).await?
    //     }
    // };

    Ok(conn)
}
