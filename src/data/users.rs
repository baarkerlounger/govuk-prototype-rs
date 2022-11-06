use crate::models::user::User;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref USERS: HashMap<&'static str, User> = {
        let mut map = HashMap::new();
        map.insert(
            "3e2dd4ae-3c37-40c6-aa64-7061f284ce28",
            User {
                // uuid: String::from("3e2dd4ae-3c37-40c6-aa64-7061f284ce28"),
                id: 5,
                age: 18,
                name: String::from("John Doe"),
                email: String::from("john.doe@example.com"),
                // age: 18,
                // grade: 1,
                // active: true,
            },
        );
        map
    };
}
