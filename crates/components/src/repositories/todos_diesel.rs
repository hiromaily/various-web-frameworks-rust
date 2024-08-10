#![allow(unused)]
#![allow(clippy::all)]

use crate::entities::todos::{TodoBody, TodoUpdateBody};
use crate::schemas::diesel::schema;
use crate::schemas::diesel::schema::todos::title;
use crate::schemas::diesel::todo_status::TodoStatus;
use crate::schemas::diesel::todos as diesel_todos;

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
pub trait TodoRepository: Send + Sync + 'static {
    fn create(&mut self, user_id: i32, payload: TodoBody) -> anyhow::Result<diesel_todos::Todo>;
    fn find_by_id(&self, todo_id: i32) -> anyhow::Result<Option<diesel_todos::Todo>>;
    fn find_all(&self) -> anyhow::Result<Vec<diesel_todos::Todo>>;
    fn update(&self, todo_id: i32, payload: TodoUpdateBody) -> anyhow::Result<diesel_todos::Todo>;
    fn delete(&self, todo_id: i32) -> anyhow::Result<u64>;
}

/*******************************************************************************
 PostgreSQL by diesel
*******************************************************************************/

type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[allow(dead_code)]
pub struct TodoRepositoryForDB {
    //conn: PgConnection,
    pool: Arc<PgPool>,
}

impl TodoRepositoryForDB {
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
impl TodoRepository for TodoRepositoryForDB {
    fn create(&mut self, user_id: i32, payload: TodoBody) -> anyhow::Result<diesel_todos::Todo> {
        let mut conn = self.get_conn()?;

        // already validated
        let status = payload
            .status
            .parse::<TodoStatus>()
            .map_err(|_| anyhow::anyhow!("Failed to parse TodoStatus from payload status"))?;

        let new_todos = diesel_todos::CreateTodo {
            user_id: user_id,
            title: payload.title.as_str(),
            description: payload.description.as_deref(), // Option<String> to Option<&'a str>
            status: status,
        };

        diesel::insert_into(schema::todos::table)
            .values(&new_todos)
            .returning(diesel_todos::Todo::as_returning())
            .get_result::<diesel_todos::Todo>(&mut conn)
            .map_err(Into::into)
    }

    fn find_by_id(&self, todo_id: i32) -> anyhow::Result<Option<diesel_todos::Todo>> {
        let mut conn = self.get_conn()?;
        schema::todos::table
            .find(todo_id)
            .first::<diesel_todos::Todo>(&mut conn)
            .optional()
            .map_err(Into::into)
    }

    fn find_all(&self) -> anyhow::Result<Vec<diesel_todos::Todo>> {
        let mut conn = self.get_conn()?;
        schema::todos::table
            .load::<diesel_todos::Todo>(&mut conn)
            .map_err(Into::into)
    }

    fn update(&self, todo_id: i32, payload: TodoUpdateBody) -> anyhow::Result<diesel_todos::Todo> {
        let mut conn = self.get_conn()?;

        // convert TodoUpdateBody to diesel_todos::UpdateTodo
        let converted_payload: diesel_todos::UpdateTodo = payload.into();

        diesel::update(schema::todos::table.find(todo_id))
            .set::<diesel_todos::UpdateTodo>(converted_payload)
            .returning(diesel_todos::Todo::as_returning())
            .get_result(&mut conn)
            .map_err(Into::into)
    }

    fn delete(&self, todo_id: i32) -> anyhow::Result<u64> {
        let mut conn = self.get_conn()?;
        let num_deleted = diesel::delete(schema::todos::table.find(todo_id)).execute(&mut conn)?;

        Ok(num_deleted as u64)
    }
}
