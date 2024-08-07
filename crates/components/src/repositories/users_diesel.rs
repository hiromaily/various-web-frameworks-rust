#![allow(unused)]
#![allow(clippy::all)]

use crate::entities::users::{UserBody, UserUpdateBody};
use crate::schemas::diesel::schema;
use crate::schemas::diesel::schema::users::first_name;
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

//#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    fn create(&mut self, payload: UserBody) -> anyhow::Result<diesel_users::User>;
    fn find(&self, email: &str, password: &str) -> anyhow::Result<Option<diesel_users::User>>;
    fn find_with_is_admin(
        &self,
        email: &str,
        password: &str,
        is_admin: bool,
    ) -> anyhow::Result<Option<diesel_users::User>>;
    fn find_by_id(&self, id: i32) -> anyhow::Result<Option<diesel_users::User>>;
    fn find_all(&self) -> anyhow::Result<Vec<diesel_users::User>>;
    fn update(&self, id: i32, payload: UserUpdateBody) -> anyhow::Result<diesel_users::User>;
    fn delete(&self, id: i32) -> anyhow::Result<u64>;
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

//#[async_trait]
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

    fn find(&self, email: &str, password: &str) -> anyhow::Result<Option<diesel_users::User>> {
        //unimplemented!("TODO");
        let mut conn = self.get_conn()?;
        schema::users::table
            .filter(schema::users::email.eq(email))
            .filter(schema::users::password.eq(password))
            .first::<diesel_users::User>(&mut conn)
            .optional()
            .map_err(Into::into)
    }

    fn find_with_is_admin(
        &self,
        email: &str,
        password: &str,
        is_admin: bool,
    ) -> anyhow::Result<Option<diesel_users::User>> {
        // unimplemented!("TODO");
        let mut conn = self.get_conn()?;
        schema::users::table
            .filter(schema::users::email.eq(email))
            .filter(schema::users::password.eq(password))
            .filter(schema::users::is_admin.eq(is_admin))
            .first::<diesel_users::User>(&mut conn)
            .optional()
            .map_err(Into::into)
    }

    fn find_by_id(&self, id: i32) -> anyhow::Result<Option<diesel_users::User>> {
        //unimplemented!("TODO");
        let mut conn = self.get_conn()?;
        schema::users::table
            .find(id)
            .first::<diesel_users::User>(&mut conn)
            .optional()
            .map_err(Into::into)

        // match result {
        //     Ok(Some(user)) => Ok(Some(user)),
        //     Ok(None) => Ok(None),
        //     Err(e) => Err(anyhow::anyhow!(e)),
        // }
    }

    fn find_all(&self) -> anyhow::Result<Vec<diesel_users::User>> {
        //unimplemented!("TODO");
        let mut conn = self.get_conn()?;
        schema::users::table
            .load::<diesel_users::User>(&mut conn)
            .map_err(Into::into)
    }

    fn update(&self, id: i32, payload: UserUpdateBody) -> anyhow::Result<diesel_users::User> {
        //unimplemented!("TODO");
        let mut conn = self.get_conn()?;
        // schema::users::table
        //     .find(id)
        //     .for_update()
        //     .first::<diesel_users::User>(&mut conn)
        //     .optional()
        //     .map_err(Into::into)

        // diesel::update(schema::users::table.find(id))
        //     .set(first_name.eq("hogehoge"))
        //     .returning(diesel_users::User::as_returning())
        //     .get_result(&mut conn)
        //     .map_err(Into::into)

        // convert UserUpdateBody to diesel_users::UpdateUser
        let convertd_payload: diesel_users::UpdateUser = payload.into();

        diesel::update(schema::users::table.find(id))
            .set::<diesel_users::UpdateUser>(convertd_payload)
            .returning(diesel_users::User::as_returning())
            .get_result(&mut conn)
            .map_err(Into::into)
    }

    fn delete(&self, id: i32) -> anyhow::Result<u64> {
        //unimplemented!("TODO");
        let mut conn = self.get_conn()?;
        let num_deleted = diesel::delete(schema::users::table.find(id)).execute(&mut conn)?;

        Ok(num_deleted as u64)
    }
}
