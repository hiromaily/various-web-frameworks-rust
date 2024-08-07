#[cfg(test)]
mod tests {
    use components::dbs::conn::*;
    use components::entities::users::UserBody;
    use components::repositories::users_diesel;
    use components::repositories::users_diesel::UserRepository;
    use components::schemas::diesel::schema;
    use components::schemas::diesel::users as diesel_users;
    use diesel::prelude::*; // required

    // use super::*;
    // use diesel::prelude::*;
    // use diesel::r2d2::{self, ConnectionManager};
    // use my_project::test_helpers::{get_test_db_pool, setup_test_database};

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
            is_admin: false,
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
        match result {
            Ok(user) => {
                println!("Created user: {:?}", user); // debug
                assert_eq!(user.first_name, "JohnTest");
                assert_eq!(user.last_name, "DoeTest");
                assert_eq!(user.email, "john.doe.test@example.com");
                assert_eq!(user.password, "securepassword123");
                assert!(!user.is_admin);
            }
            Err(e) => panic!("Failed to create user: {:?}", e),
        }

        // find
        let email = "john.doe.test@example.com";
        let password = "securepassword123";
        let result = users_repo.find(email, password);
        match result {
            Ok(Some(user)) => {
                println!("Found user: {:?}", user); // debug
                assert_eq!(user.first_name, "JohnTest");
                assert_eq!(user.last_name, "DoeTest");
                assert_eq!(user.email, "john.doe.test@example.com");
                assert_eq!(user.password, "securepassword123");
                assert!(!user.is_admin);
            }
            Ok(None) => {
                panic!("User must be returned");
            }
            Err(e) => {
                panic!("Failed to find user: {:?}", e);
            }
        }
    }

    #[test]
    #[ignore] // integration test must be ignored as default
    fn test_diesel_user_repository_find() {
        let pool =
            get_diesel_pool_with_env().expect("Failed to establish a connection to database");

        let users_repo = users_diesel::UserRepositoryForDB::new(pool);
        let email = "john.doe.test@example.com";
        let password = "securepassword123";

        let result = users_repo.find(email, password);
        //   pub struct User {
        //     pub id: i32,
        //     pub first_name: String,
        //     pub last_name: String,
        //     pub email: String,
        //     pub password: String,
        //     pub is_admin: bool,
        //     pub created_at: Option<NaiveDateTime>,
        // }
        match result {
            Ok(Some(user)) => {
                println!("Found user: {:?}", user); // debug
                assert_eq!(user.first_name, "JohnTest");
                assert_eq!(user.last_name, "DoeTest");
                assert_eq!(user.email, "john.doe.test@example.com");
                assert_eq!(user.password, "securepassword123");
                assert!(!user.is_admin);
            }
            Ok(None) => {
                panic!("User must be returned");
            }
            Err(e) => {
                panic!("Failed to find user: {:?}", e);
            }
        }
    }
}
