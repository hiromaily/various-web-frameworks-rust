// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "todo_status"))]
    pub struct TodoStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TodoStatus;

    todos (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        description -> Nullable<Text>,
        status -> TodoStatus,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 50]
        email -> Varchar,
        #[max_length = 100]
        password -> Varchar,
        is_admin -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(todos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(todos, users,);
