#![allow(unused)]
#![allow(clippy::all)]

use crate::entities::users::{UserBody, UserUpdateBody};
use crate::schemas::diesel::schema;
use crate::schemas::diesel::users as diesel_users;
use async_trait::async_trait;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::{
    clone::Clone,
    collections::HashMap,
    fmt::Debug,
    marker::{Send, Sync},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use thiserror::Error;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    fn create(&mut self, payload: UserBody) -> anyhow::Result<diesel_users::User>;
    async fn find(&self, email: &str, password: &str)
        -> anyhow::Result<Option<diesel_users::User>>;
    async fn find_with_is_admin(
        &self,
        email: &str,
        password: &str,
        is_admin: bool,
    ) -> anyhow::Result<Option<diesel_users::User>>;
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<diesel_users::User>>;
    fn find_all(&self) -> anyhow::Result<Vec<diesel_users::User>>;
    async fn update(
        &self,
        id: i32,
        payload: UserUpdateBody,
    ) -> anyhow::Result<Option<diesel_users::User>>;
    async fn delete(&self, id: i32) -> anyhow::Result<u64>;
}

/*******************************************************************************
 PostgreSQL by diesel
*******************************************************************************/

type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[allow(dead_code)]
pub struct UserRepositoryForDB {
    //conn: PgConnection,
    pool: Arc<PgPool>,
}

impl UserRepositoryForDB {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }
    // pub fn new(conn: PgConnection) -> Self {
    //     Self { conn }
    // }

    fn get_conn(&self) -> anyhow::Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>> {
        self.pool.get().map_err(|e| anyhow::anyhow!(e))
    }
}

#[async_trait]
impl UserRepository for UserRepositoryForDB {
    fn create(&mut self, payload: UserBody) -> anyhow::Result<diesel_users::User> {
        //unimplemented!("TODO");

        let mut conn = self.get_conn()?;
        let new_user = diesel_users::CreateUser {
            first_name: payload.first_name.as_str(),
            last_name: payload.last_name.as_str(),
            email: payload.email.as_str(),
            password: payload.password.as_str(),
            is_admin: payload.is_admin,
        };

        diesel::insert_into(schema::users::table)
            .values(&new_user)
            // .returning((
            //     schema::users::id,
            //     schema::users::first_name,
            //     schema::users::last_name,
            //     schema::users::email,
            //     schema::users::password,
            //     schema::users::is_admin,
            // ))
            .returning(diesel_users::User::as_returning())
            //.get_result(&conn)
            .get_result::<diesel_users::User>(&mut conn)
            .map_err(Into::into)
    }

    async fn find(
        &self,
        email: &str,
        password: &str,
    ) -> anyhow::Result<Option<diesel_users::User>> {
        unimplemented!("TODO");
    }

    async fn find_with_is_admin(
        &self,
        email: &str,
        password: &str,
        is_admin: bool,
    ) -> anyhow::Result<Option<diesel_users::User>> {
        unimplemented!("TODO");
    }

    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<diesel_users::User>> {
        unimplemented!("TODO");
    }

    fn find_all(&self) -> anyhow::Result<Vec<diesel_users::User>> {
        //unimplemented!("TODO");
        let mut conn = self.get_conn()?;
        schema::users::table
            .load::<diesel_users::User>(&mut conn)
            .map_err(Into::into)
    }

    async fn update(
        &self,
        id: i32,
        payload: UserUpdateBody,
    ) -> anyhow::Result<Option<diesel_users::User>> {
        unimplemented!("TODO");
    }

    async fn delete(&self, id: i32) -> anyhow::Result<u64> {
        unimplemented!("TODO");
    }
}
