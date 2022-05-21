#![allow(non_snake_case)]

use etanol::FindWhere;

mod database;

use database::{create_connection, user::User};

fn main() {
    create_connection();

    let user = User {
        id: "4".to_string(),
        age: Some(3),
        name: "Random".to_string(),
        ..User::default()
    };

    user.insert().execute();

    let _users = User::find()
        .field(FindWhere::NotEqual("name", "Random"))
        // .field(FindWhere::Equal("age", "20"))
        .take(10)
        .load();

    match _users {
        Ok(users) => {
            for user in &users {
                println!("{:?}", user);
            }
        }
        Err(e) => eprintln!("{:?}", e),
    }
}
