use crate::entities::users::{UserBody, UserUpdateBody};
use crate::schemas::{prelude::Users, users as db_users};
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
//pub trait UserRepository: Debug + Clone + Send + Sync + 'static {
pub trait UserRepository: Debug + Send + Sync + 'static {
    async fn create(&self, payload: UserBody) -> anyhow::Result<db_users::Model>;
    async fn find(&self, email: &str, password: &str) -> anyhow::Result<Option<db_users::Model>>;
    async fn find_with_is_admin(
        &self,
        email: &str,
        password: &str,
        is_admin: bool,
    ) -> anyhow::Result<Option<db_users::Model>>;
    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<db_users::Model>>;
    async fn find_all(&self) -> anyhow::Result<Vec<db_users::Model>>;
    async fn update(
        &self,
        id: i32,
        payload: UserUpdateBody,
    ) -> anyhow::Result<Option<db_users::Model>>;
    async fn delete(&self, id: i32) -> anyhow::Result<u64>;
}

/*******************************************************************************
 PostgreSQL
*******************************************************************************/
#[derive(Debug, Clone)]
pub struct UserRepositoryForDB {
    conn: sea_orm::DatabaseConnection,
}

impl UserRepositoryForDB {
    pub fn new(conn: sea_orm::DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryForDB {
    async fn create(&self, payload: UserBody) -> anyhow::Result<db_users::Model> {
        // actually: Result<db_users::Model, DbErr>
        let user = db_users::ActiveModel {
            first_name: Set(payload.first_name),
            last_name: Set(payload.last_name),
            email: Set(payload.email),
            password: Set(payload.password),
            is_admin: Set(payload.is_admin),
            created_at: Set(Some(Utc::now().naive_utc())), // for type `Option<DateTime>`
            ..Default::default()
        };
        user.insert(&self.conn).await.map_err(Into::into)
        //.map_err(|e| anyhow::Error::from(e))
        //.with_context(|| format!("Failed to create user: {:?}", payload))
    }

    async fn find(&self, email: &str, password: &str) -> anyhow::Result<Option<db_users::Model>> {
        // Result<Option<db_users::Model>, DbErr>
        let query = Users::find()
            .filter(db_users::Column::Email.eq(email))
            .filter(db_users::Column::Password.eq(password));

        query.one(&self.conn).await.map_err(Into::into)
    }

    async fn find_with_is_admin(
        &self,
        email: &str,
        password: &str,
        is_admin: bool,
    ) -> anyhow::Result<Option<db_users::Model>> {
        // Result<Option<db_users::Model>, DbErr>
        let query = Users::find()
            .filter(db_users::Column::Email.eq(email))
            .filter(db_users::Column::Password.eq(password))
            .filter(db_users::Column::IsAdmin.eq(is_admin));

        query.one(&self.conn).await.map_err(Into::into)
    }

    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<db_users::Model>> {
        // Result<Option<db_users::Model>, DbErr>
        Users::find_by_id(id)
            .one(&self.conn)
            .await
            .map_err(Into::into)
    }

    async fn find_all(&self) -> anyhow::Result<Vec<db_users::Model>> {
        // Result<Vec<db_users::Model>, DbErr>
        Users::find().all(&self.conn).await.map_err(Into::into)
    }

    async fn update(
        &self,
        id: i32,
        payload: UserUpdateBody,
    ) -> anyhow::Result<Option<db_users::Model>> {
        // Result<Option<db_users::Model>, DbErr>

        // let mut user: db_users::ActiveModel =
        //     Users::find_by_id(id).one(&self.conn).await?.unwrap().into();
        let user_option = Users::find_by_id(id).one(&self.conn).await?;
        let mut user: db_users::ActiveModel = match user_option {
            Some(user) => user.into(),
            None => return Ok(None),
        };

        if let Some(val) = payload.first_name {
            user.first_name = Set(val);
        }
        if let Some(val) = payload.last_name {
            user.last_name = Set(val);
        }
        if let Some(val) = payload.email {
            user.email = Set(val);
        }
        if let Some(val) = payload.password {
            user.password = Set(val);
        }
        if let Some(val) = payload.is_admin {
            user.is_admin = Set(val);
        }

        user.update(&self.conn).await.map(Some).map_err(Into::into)
    }

    async fn delete(&self, id: i32) -> anyhow::Result<u64> {
        // actually: Result<u64, DbErr>
        let user = db_users::ActiveModel {
            id: Set(id),
            ..Default::default()
        };
        Users::delete(user)
            .exec(&self.conn)
            .await
            .map(|res| res.rows_affected)
            .map_err(Into::into)
    }
}

/*******************************************************************************
 On memory
*******************************************************************************/
type UserDatas = HashMap<i32, db_users::Model>;

#[derive(Debug, Default, Clone)]
#[allow(dead_code, unused_variables)]
pub struct UserRepositoryForMemory {
    store: Arc<RwLock<UserDatas>>,
}

// impl Default for UserRepositoryForMemory {
//     fn default() -> Self {
//         UserRepositoryForMemory {
//             store: Arc::default(),
//         }
//     }
// }

#[allow(dead_code, unused_variables)]
impl UserRepositoryForMemory {
    pub fn new() -> Self {
        Self::default()
    }

    fn write_store_ref(&self) -> RwLockWriteGuard<UserDatas> {
        self.store.write().unwrap()
    }

    fn read_store_ref(&self) -> RwLockReadGuard<UserDatas> {
        self.store.read().unwrap()
    }
}

#[async_trait]
#[allow(dead_code, unused_variables)]
impl UserRepository for UserRepositoryForMemory {
    async fn create(&self, payload: UserBody) -> anyhow::Result<db_users::Model> {
        todo!()
        // let mut store = self.write_store_ref();
        // let id = (store.len() + 1) as i32;
        // let user = db_users::Model::new(id, payload.text.clone());
        // store.insert(id, user.clone());
        // user
    }

    async fn find(&self, email: &str, password: &str) -> anyhow::Result<Option<db_users::Model>> {
        todo!()
    }

    async fn find_with_is_admin(
        &self,
        email: &str,
        password: &str,
        is_admin: bool,
    ) -> anyhow::Result<Option<db_users::Model>> {
        todo!()
    }

    async fn find_by_id(&self, id: i32) -> anyhow::Result<Option<db_users::Model>> {
        todo!()
        // let store = self.read_store_ref();
        // store.get(&id).cloned()
    }

    async fn find_all(&self) -> anyhow::Result<Vec<db_users::Model>> {
        todo!()
        // let store = self.read_store_ref();
        // Vec::from_iter(store.values().cloned())
    }

    async fn update(
        &self,
        id: i32,
        payload: UserUpdateBody,
    ) -> anyhow::Result<Option<db_users::Model>> {
        todo!()
        // let mut store = self.write_store_ref();
        // let user = store.get(&id).context(RepositoryError::NotFound(id))?;
        // let text = payload.text.unwrap_or(user.text.clone());
        // let completed = payload.completed.unwrap_or(user.completed);
        // let user = User {
        //     id,
        //     text,
        //     completed,
        // };
        // store.insert(id, user.clone());
        // Ok(user)
    }

    async fn delete(&self, id: i32) -> anyhow::Result<u64> {
        todo!()
        // let mut store = self.write_store_ref();
        // store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        // Ok(())
    }
}
