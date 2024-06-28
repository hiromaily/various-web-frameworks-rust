use crate::entities::todos::{TodoBody, TodoUpdateBody};
use crate::schemas::{prelude::Todos, sea_orm_active_enums::TodoStatus, todos as db_todos};
//use anyhow::Context;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{self, ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter}; // DbErr
use std::{
    clone::Clone,
    collections::HashMap,
    fmt::Debug,
    marker::{Send, Sync},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use thiserror::Error;

#[allow(dead_code, unused_variables)]
#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

#[async_trait]
//pub trait TodoRepository: Debug + Clone + Send + Sync + 'static {
pub trait TodoRepository: Debug + Send + Sync + 'static {
    async fn create(&self, user_id: i32, payload: TodoBody) -> anyhow::Result<db_todos::Model>;
    async fn find_by_id(&self, todo_id: i32) -> anyhow::Result<Option<db_todos::Model>>;
    async fn find_all(&self, user_id: i32) -> anyhow::Result<Vec<db_todos::Model>>;
    async fn update(
        &self,
        todo_id: i32,
        payload: TodoUpdateBody,
    ) -> anyhow::Result<Option<db_todos::Model>>;
    async fn delete(&self, todo_id: i32) -> anyhow::Result<u64>;
}

/*******************************************************************************
 PostgreSQL
*******************************************************************************/
#[derive(Debug, Clone)]
pub struct TodoRepositoryForDB {
    conn: sea_orm::DatabaseConnection,
}

impl TodoRepositoryForDB {
    pub fn new(conn: sea_orm::DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryForDB {
    async fn create(&self, user_id: i32, payload: TodoBody) -> anyhow::Result<db_todos::Model> {
        // actually: Result<db_todos::Model, DbErr>

        // already validated
        let status = payload.status.parse::<TodoStatus>().unwrap();

        let todo = db_todos::ActiveModel {
            user_id: Set(user_id),
            title: Set(payload.title),
            description: Set(payload.description),
            status: Set(status),
            created_at: Set(Some(Utc::now().naive_utc())), // for type `Option<DateTime>`
            ..Default::default()
        };
        todo.insert(&self.conn).await.map_err(Into::into)
        //.map_err(|e| anyhow::Error::from(e))
        //.with_context(|| format!("Failed to create todo: {:?}", payload))
    }

    async fn find_by_id(&self, todo_id: i32) -> anyhow::Result<Option<db_todos::Model>> {
        // Result<Option<db_todos::Model>, DbErr>
        Todos::find_by_id(todo_id)
            .one(&self.conn)
            .await
            .map_err(Into::into)
    }

    async fn find_all(&self, user_id: i32) -> anyhow::Result<Vec<db_todos::Model>> {
        // Result<Vec<db_todos::Model>, DbErr>
        let query = Todos::find().filter(db_todos::Column::UserId.eq(user_id));
        query.all(&self.conn).await.map_err(Into::into)
    }

    async fn update(
        &self,
        todo_id: i32,
        payload: TodoUpdateBody,
    ) -> anyhow::Result<Option<db_todos::Model>> {
        let todo_option = Todos::find_by_id(todo_id).one(&self.conn).await?;
        let mut todo: db_todos::ActiveModel = match todo_option {
            Some(todo) => todo.into(),
            None => return Ok(None),
        };

        if let Some(val) = payload.title {
            todo.title = Set(val);
        }
        if let Some(val) = payload.description {
            todo.description = Set(Some(val));
        }
        // payload.status is enum (Option<String>)
        // need to convert Option<String> to TodoStatus
        if let Some(val) = payload.status {
            todo.status = Set(val.parse::<TodoStatus>().unwrap());
        }
        todo.update(&self.conn).await.map(Some).map_err(Into::into)
    }

    async fn delete(&self, todo_id: i32) -> anyhow::Result<u64> {
        // actually: Result<u64, DbErr>
        let todo = db_todos::ActiveModel {
            id: Set(todo_id),
            ..Default::default()
        };
        Todos::delete(todo)
            .exec(&self.conn)
            .await
            .map(|res| res.rows_affected)
            .map_err(Into::into)
    }
}

/*******************************************************************************
 On memory
*******************************************************************************/
type TodoDatas = HashMap<i32, db_todos::Model>;

#[derive(Debug, Default, Clone)]
#[allow(dead_code, unused_variables)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDatas>>,
}

// impl Default for TodoRepositoryForMemory {
//     fn default() -> Self {
//         TodoRepositoryForMemory {
//             store: Arc::default(),
//         }
//     }
// }

#[allow(dead_code, unused_variables)]
impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        Self::default()
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<TodoDatas> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<TodoDatas> {
        self.store.read().unwrap()
    }
}

#[async_trait]
#[allow(dead_code, unused_variables)]
impl TodoRepository for TodoRepositoryForMemory {
    async fn create(&self, user_id: i32, payload: TodoBody) -> anyhow::Result<db_todos::Model> {
        todo!()
    }

    async fn find_by_id(&self, todo_id: i32) -> anyhow::Result<Option<db_todos::Model>> {
        todo!()
    }

    async fn find_all(&self, user_id: i32) -> anyhow::Result<Vec<db_todos::Model>> {
        todo!()
    }

    async fn update(
        &self,
        todo_id: i32,
        payload: TodoUpdateBody,
    ) -> anyhow::Result<Option<db_todos::Model>> {
        todo!()
    }

    async fn delete(&self, todo_id: i32) -> anyhow::Result<u64> {
        todo!()
    }
}
