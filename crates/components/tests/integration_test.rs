#[cfg(test)]
mod tests {
    use components::dbs::conn::*;
    use components::entities::todos::{TodoBody, TodoUpdateBody};
    use components::entities::users::{UserBody, UserUpdateBody};
    use components::repositories::todos_diesel;
    use components::repositories::todos_diesel::TodoRepository;
    use components::repositories::users_diesel;
    use components::repositories::users_diesel::UserRepository;
    use components::schemas::diesel::schema;
    use components::schemas::diesel::todo_status::TodoStatus;
    use components::schemas::diesel::todos as diesel_todos;
    use components::schemas::diesel::users as diesel_users;
    use diesel::prelude::*;
    use validator::ValidateLength; // required

    // utility
    fn assert_found_user(result: anyhow::Result<Option<diesel_users::User>>) {
        match result {
            Ok(Some(user)) => {
                println!("Found user: {:?}", user); // debug
                assert_eq!(user.first_name, "JohnTest");
                assert_eq!(user.last_name, "DoeTest");
                assert_eq!(user.email, "john.doe.test@example.com");
                assert_eq!(user.password, "securepassword123");
                assert!(user.is_admin);
            }
            Ok(None) => {
                panic!("User must be returned");
            }
            Err(e) => {
                panic!("Failed to find user: {:?}", e);
            }
        }
    }

    fn assert_found_todo(result: anyhow::Result<Option<diesel_todos::Todo>>) {
        match result {
            Ok(Some(todo)) => {
                println!("Found todo: {:?}", todo); // debug
                assert_eq!(todo.title, "Study Rust");
                assert_eq!(
                    todo.description,
                    Some("read book of programming Rust".to_string())
                );
                assert!(matches!(todo.status, TodoStatus::Pending));
            }
            Ok(None) => {
                panic!("Todo must be returned");
            }
            Err(e) => {
                panic!("Failed to find todo: {:?}", e);
            }
        }
    }

    #[test]
    #[ignore] // integration test must be ignored as default
    fn test_diesel_connection() {
        let mut conn =
            get_diesel_conn_with_env().expect("Failed to establish a connection to database");

        let user_list = schema::users::table
            .load::<diesel_users::User>(&mut conn)
            .expect("Failed to select users table");

        // validation
        assert!(!user_list.is_empty(), "user list is empty");
    }

    #[test]
    #[ignore] // integration test must be ignored as default
    #[should_panic]
    fn test_diesel_connection_panic() {
        // TODO: get parameters from ENV in test
        let host = "127.0.0.1:5432";
        let dbname = "example";
        let user = "invalid";
        let password = "invalid";
        let mut conn = get_diesel_conn(user, password, host, dbname)
            .expect("Failed to establish a connection to database");

        let _user_list = schema::users::table
            .load::<diesel_users::User>(&mut conn)
            .expect("Failed to select users table");
    }

    #[test]
    #[ignore] // integration test must be ignored as default
    fn test_diesel_user_repository() {
        let pool =
            get_diesel_pool_with_env().expect("Failed to establish a connection to database");

        let mut users_repo = users_diesel::UserRepositoryForDB::new(pool);

        // create
        let user_data = UserBody {
            first_name: String::from("JohnTest"),
            last_name: String::from("DoeTest"),
            email: String::from("john.doe.test@example.com"),
            password: String::from("securepassword123"),
            is_admin: true,
        };
        let result = users_repo.create(user_data);
        //   pub struct User {
        //     pub id: i32,
        //     pub first_name: String,
        //     pub last_name: String,
        //     pub email: String,
        //     pub password: String,
        //     pub is_admin: bool,
        //     pub created_at: Option<NaiveDateTime>,
        // }
        let user_id = match result {
            Ok(user) => {
                println!("Created user: {:?}", user); // debug
                assert_eq!(user.first_name, "JohnTest");
                assert_eq!(user.last_name, "DoeTest");
                assert_eq!(user.email, "john.doe.test@example.com");
                assert_eq!(user.password, "securepassword123");
                assert!(user.is_admin);
                user.id
            }
            Err(e) => panic!("Failed to create user: {:?}", e),
        };

        // find
        let email = "john.doe.test@example.com";
        let password = "securepassword123";
        let result = users_repo.find(email, password);
        assert_found_user(result);

        // find_with_is_admin
        let result = users_repo.find_with_is_admin(email, password, true);
        assert_found_user(result);

        // find_by_id
        let result = users_repo.find_by_id(user_id);
        assert_found_user(result);

        // find_all
        let result = users_repo.find_all();
        match result {
            Ok(users) => {
                println!("Found users: {:?}", users); // debug
                assert_eq!(users.length(), Some(2));
            }
            Err(e) => panic!("Failed to find users: {:?}", e),
        }

        // update
        let user_update = UserUpdateBody {
            first_name: Some("JohnUpdated".into()),
            last_name: Some("DoeUpdated".into()),
            email: Some("john.doe.updated@example.com".into()),
            password: Some("securepassword123456".into()),
            is_admin: Some(true),
        };
        let result = users_repo.update(user_id, user_update);
        match result {
            Ok(user) => {
                println!("Updated user: {:?}", user); // debug
                assert_eq!(user.first_name, "JohnUpdated");
                assert_eq!(user.last_name, "DoeUpdated");
                assert_eq!(user.email, "john.doe.updated@example.com");
                assert_eq!(user.password, "securepassword123456");
                assert!(user.is_admin);
            }
            Err(e) => {
                panic!("Failed to update user: {:?}", e);
            }
        }

        // delete
        let _ = users_repo.delete(user_id);
        let result = users_repo.find_by_id(user_id); // must be none
        match result {
            Ok(Some(_user)) => {
                panic!("User must not be returned");
            }
            Ok(None) => {
                println!("OK");
            }
            Err(e) => {
                panic!("Failed to find user: {:?}", e);
            }
        }
    }

    #[test]
    #[ignore] // integration test must be ignored as default
    fn test_diesel_todo_repository() {
        let pool =
            get_diesel_pool_with_env().expect("Failed to establish a connection to database");

        let mut todos_repo = todos_diesel::TodoRepositoryForDB::new(pool);

        // create
        let todo_data = TodoBody {
            title: String::from("Study Rust"),
            description: Some(String::from("read book of programming Rust")),
            status: String::from("pending"),
        };

        let user_id = 1;
        let result = todos_repo.create(user_id, todo_data);
        let todo_id = match result {
            Ok(todo) => {
                println!("Created todo: {:?}", todo); // debug
                assert_eq!(todo.title, "Study Rust");
                assert_eq!(
                    todo.description,
                    Some("read book of programming Rust".to_string())
                );
                assert!(matches!(todo.status, TodoStatus::Pending));
                todo.id
            }
            Err(e) => panic!("Failed to create todo: {:?}", e),
        };

        // find_by_id
        let result = todos_repo.find_by_id(todo_id);
        assert_found_todo(result);

        // find_all
        let result = todos_repo.find_all();
        match result {
            Ok(todos) => {
                println!("Found todos: {:?}", todos); // debug
                assert_eq!(todos.length(), Some(1));
            }
            Err(e) => panic!("Failed to find todos: {:?}", e),
        }

        // update
        let todo_update = TodoUpdateBody {
            title: Some("Study Rust Updated".into()),
            description: Some("read book of programming Rust updated".into()),
            status: Some("doing".into()),
        };
        let result = todos_repo.update(todo_id, todo_update);
        match result {
            Ok(todo) => {
                println!("Updated todo: {:?}", todo); // debug
                assert_eq!(todo.title, "Study Rust Updated");
                assert_eq!(
                    todo.description,
                    Some("read book of programming Rust updated".into())
                );
                assert!(matches!(todo.status, TodoStatus::Doing));
            }
            Err(e) => {
                panic!("Failed to update todo: {:?}", e);
            }
        }

        // delete
        let _ = todos_repo.delete(todo_id);
        let result = todos_repo.find_by_id(todo_id); // must be none
        match result {
            Ok(Some(_todo)) => {
                panic!("Todo must not be returned");
            }
            Ok(None) => {
                println!("OK");
            }
            Err(e) => {
                panic!("Failed to find todo: {:?}", e);
            }
        }
    }
}
