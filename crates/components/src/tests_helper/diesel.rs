use crate::dbs;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use std::sync::Mutex;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

lazy_static::lazy_static! {
    static ref TEST_DB_POOL: Mutex<Option<DbPool>> = Mutex::new(None);
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_test_db_pool() -> DbPool {
    let mut pool = TEST_DB_POOL.lock().unwrap();
    if pool.is_none() {
        let manager = ConnectionManager::<PgConnection>::new(
            env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        );
        *pool = Some(
            r2d2::Pool::builder()
                .build(manager)
                .expect("Failed to create pool."),
        );
    }
    pool.clone().unwrap()
}

pub fn setup_test_database() {
    let host = "127.0.0.1:5432";
    let dbname = "example";
    let user = "admin";
    let password = "admin";
    let conn = dbs::get_diesel_conn(user, password, host, dbname).unwrap();
    // let conn = establish_connection();
    // Run migrations or set up schema here
    // e.g., diesel_migrations::run_pending_migrations(&conn).unwrap();
}
