use crate::entities::users;
use crate::hashes::hash;
use crate::repositories::{todos as repo_todos, users as repo_users};
use crate::schemas::users as db_users;
use anyhow;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait AdminUsecase: Send + Sync + 'static {
    async fn get_user_list(&self) -> anyhow::Result<Vec<db_users::Model>>;
    async fn add_user(&self, user_body: users::UserBody) -> anyhow::Result<db_users::Model>;
    async fn get_user(&self, user_id: i32) -> anyhow::Result<Option<db_users::Model>>;
    async fn update_user(
        &self,
        user_id: i32,
        user_body: users::UserUpdateBody,
    ) -> anyhow::Result<Option<db_users::Model>>;
    async fn delete_user(&self, user_id: i32) -> anyhow::Result<u64>;
}

#[derive(Debug)]
pub struct AdminAction<T: hash::Hash> {
    pub todos_repo: Arc<dyn repo_todos::TodoRepository>, // for now, not used anywhere
    pub users_repo: Arc<dyn repo_users::UserRepository>,
    pub hash: T,
}

impl<T: hash::Hash> AdminAction<T> {
    pub fn new(
        todos_repo: Arc<dyn repo_todos::TodoRepository>,
        users_repo: Arc<dyn repo_users::UserRepository>,
        hash: T,
    ) -> Self {
        AdminAction {
            todos_repo,
            users_repo,
            hash,
        }
    }
}

#[async_trait]
impl<T: hash::Hash> AdminUsecase for AdminAction<T> {
    async fn get_user_list(&self) -> anyhow::Result<Vec<db_users::Model>> {
        let ret = self.users_repo.find_all().await?;
        Ok(ret)
        // vec![db_users::Model {
        //     id: 1,
        //     first_name: "John".to_string(),
        //     last_name: "Doe".to_string(),
        //     email: "john.doe@example.com".to_string(),
        //     password: "password".to_string(),
        //     is_admin: true,
        //     created_at: None,
        // }]
    }

    async fn add_user(&self, user_body: users::UserBody) -> anyhow::Result<db_users::Model> {
        // hash
        let hashed_password = self.hash.hash(user_body.password.as_bytes())?;

        // new object
        let updated_user_body = users::UserBody {
            password: hashed_password,
            ..user_body // Copy other fields from the original user_body
        };

        let ret = self.users_repo.create(updated_user_body).await?;
        Ok(ret)
        // Ok(db_users::Model {
        //     id: 1,
        //     first_name: "John".to_string(),
        //     last_name: "Doe".to_string(),
        //     email: "john.doe@example.com".to_string(),
        //     password: "password".to_string(),
        //     is_admin: true,
        //     created_at: None,
        // })
    }

    async fn get_user(&self, user_id: i32) -> anyhow::Result<Option<db_users::Model>> {
        let ret = self.users_repo.find_by_id(user_id).await?;
        Ok(ret)
        // match user_id {
        //     1 => Some(db_users::Model {
        //         id: 1,
        //         first_name: "John".to_string(),
        //         last_name: "Doe".to_string(),
        //         email: "john.doe@example.com".to_string(),
        //         password: "password".to_string(),
        //         is_admin: true,
        //         created_at: None,
        //     }),
        //     _ => None, // User with user_id not found
        // }
    }

    async fn update_user(
        &self,
        user_id: i32,
        user_body: users::UserUpdateBody,
    ) -> anyhow::Result<Option<db_users::Model>> {
        // if user_body contains password, it must be hashed
        let hashed_password = if let Some(_password) = &user_body.password {
            // Hash the password
            Some(self.hash.hash(user_body.password.unwrap().as_bytes())?)
        } else {
            None
        };

        // create new object
        let updated_user_body = users::UserUpdateBody {
            password: hashed_password,
            ..user_body // Copy other fields from the original user_body
        };

        let ret = self.users_repo.update(user_id, updated_user_body).await?;
        Ok(ret)
        // Ok(db_users::Model {
        //     id: 1,
        //     first_name: "John".to_string(),
        //     last_name: "Doe".to_string(),
        //     email: "john.doe@example.com".to_string(),
        //     password: "password".to_string(),
        //     is_admin: true,
        //     created_at: None,
        // })
    }

    async fn delete_user(&self, user_id: i32) -> anyhow::Result<u64> {
        let ret = self.users_repo.delete(user_id).await?;
        Ok(ret)
    }
}
