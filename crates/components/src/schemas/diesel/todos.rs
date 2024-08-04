// Generated by diesel_ext
// then edited

#![allow(unused)]
#![allow(clippy::all)]

use crate::schemas::diesel::schema::todos as diesel_todos;
use crate::schemas::diesel::todo_status::TodoStatus;
use chrono::NaiveDateTime;
use diesel::deserialize::{self, FromSql, Queryable};
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Todo {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: TodoStatus,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = diesel_todos)]
pub struct CreateTodo {
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: TodoStatus,
}
