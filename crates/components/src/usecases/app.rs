use crate::entities::todos;
use crate::repositories::{todos as repo_todos, users as repo_users};
use crate::schemas::todos as db_todos;
use anyhow;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait AppUsecase: Send + Sync + 'static {
    async fn get_user_todo_list(&self, user_id: i32) -> anyhow::Result<Vec<db_todos::Model>>;
    async fn add_user_todo(
        &self,
        user_id: i32,
        user_body: todos::TodoBody,
    ) -> anyhow::Result<db_todos::Model>;
    async fn get_user_todo(
        &self,
        user_id: i32,
        todo_id: i32,
    ) -> anyhow::Result<Option<db_todos::Model>>;
    async fn update_user_todo(
        &self,
        user_id: i32,
        todo_id: i32,
        todo_body: todos::TodoUpdateBody,
    ) -> anyhow::Result<Option<db_todos::Model>>;
    async fn delete_user_todo(&self, user_id: i32, todo_id: i32) -> anyhow::Result<u64>;
}

#[derive(Debug)]
pub struct AppAction {
    pub todos_repo: Arc<dyn repo_todos::TodoRepository>,
    pub users_repo: Arc<dyn repo_users::UserRepository>, // for now, not used anywhere
}

impl AppAction {
    pub fn new(
        todos_repo: Arc<dyn repo_todos::TodoRepository>,
        users_repo: Arc<dyn repo_users::UserRepository>,
    ) -> Self {
        AppAction {
            todos_repo,
            users_repo,
        }
    }
}

#[async_trait]
impl AppUsecase for AppAction {
    async fn get_user_todo_list(&self, user_id: i32) -> anyhow::Result<Vec<db_todos::Model>> {
        let ret = self.todos_repo.find_all(user_id).await?;
        Ok(ret)
        // vec![db_todos::Model {
        //     id: 1,
        //     user_id: 1,
        //     title: "Do something".to_string(),
        //     description: Some("foo bar foo bar".to_string()),
        //     status: sea_orm_active_enums::TodoStatus::Pending,
        //     created_at: None,
        //     updated_at: None,
        // }]
    }

    async fn add_user_todo(
        &self,
        user_id: i32,
        todo_body: todos::TodoBody,
    ) -> anyhow::Result<db_todos::Model> {
        let ret = self.todos_repo.create(user_id, todo_body).await?;
        Ok(ret)
        // Ok(db_todos::Model {
        //     id: 1,
        //     user_id: 1,
        //     title: "Do something".to_string(),
        //     description: Some("foo bar foo bar".to_string()),
        //     status: sea_orm_active_enums::TodoStatus::Pending,
        //     created_at: None,
        //     updated_at: None,
        // })
    }

    async fn get_user_todo(
        &self,
        _user_id: i32,
        todo_id: i32,
    ) -> anyhow::Result<Option<db_todos::Model>> {
        let ret = self.todos_repo.find_by_id(todo_id).await?;
        Ok(ret)
        // match user_id {
        //     1 => Some(db_todos::Model {
        //         id: 1,
        //         user_id: 1,
        //         title: "Do something".to_string(),
        //         description: Some("foo bar foo bar".to_string()),
        //         status: sea_orm_active_enums::TodoStatus::Pending,
        //         created_at: None,
        //         updated_at: None,
        //     }),
        //     _ => None, // User with user_id not found
        // }
    }

    async fn update_user_todo(
        &self,
        _user_id: i32,
        todo_id: i32,
        todo_body: todos::TodoUpdateBody,
    ) -> anyhow::Result<Option<db_todos::Model>> {
        let ret = self.todos_repo.update(todo_id, todo_body).await?;
        Ok(ret)
        // Ok(db_todos::Model {
        //     id: 1,
        //     user_id: 1,
        //     title: "Do something".to_string(),
        //     description: Some("foo bar foo bar".to_string()),
        //     status: sea_orm_active_enums::TodoStatus::Pending,
        //     created_at: None,
        //     updated_at: None,
        // })
    }

    async fn delete_user_todo(&self, _user_id: i32, todo_id: i32) -> anyhow::Result<u64> {
        let ret = self.todos_repo.delete(todo_id).await?;
        Ok(ret)
    }
}
