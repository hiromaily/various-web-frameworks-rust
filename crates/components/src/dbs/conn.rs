use anyhow::{Context, Result as AnyhowResult};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::ConnectionError;
use sea_orm::{ConnectOptions, Database, DbErr};
use std::time::Duration;

//-----------------------------------------------------------------------------
// sea_orm
//-----------------------------------------------------------------------------

// refer to: https://www.sea-ql.org/sea-orm-tutorial/ch01-01-project-setup.html
pub async fn get_sea_orm_conn(
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

//-----------------------------------------------------------------------------
// diesel
//-----------------------------------------------------------------------------

pub fn get_diesel_conn(
    user: &str,
    password: &str,
    host: &str,
    db_name: &str,
    //) -> ConnectionResult<PgConnection> {
) -> Result<PgConnection, ConnectionError> {
    let db_url = format!("postgresql://{user}:{password}@{host}/{db_name}");

    PgConnection::establish(&db_url)
    //.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_diesel_pool(
    user: &str,
    password: &str,
    host: &str,
    db_name: &str,
    //) -> ConnectionResult<PgConnection> {
) -> AnyhowResult<Pool<ConnectionManager<PgConnection>>> {
    let db_url = format!("postgresql://{user}:{password}@{host}/{db_name}");

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .context("Failed to create database connection pool")
}
