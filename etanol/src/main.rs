#![allow(non_snake_case)]

use etanol::{ModelWhere, QueryValue};

mod database;

use database::{create_connection, user::User};

fn main() {
    create_connection();

    // let user = User {
    //     id: "5".to_string(),
    //     age: Some(5),
    //     name: "Teste".to_string(),
    //     ..User::default()
    // };

    // user.insert().execute();

    // for user in &_users {
    //     println!("{:?}", user);
    // }

    User::delete().field(ModelWhere::Equal("id", "1")).execute();

    let _users = User::find()
        // .field(ModelWhere::GreaterThan("random", 3))
        // .skip(1)
        // .take(1)
        .many()
        .load()
        .unwrap();

    for user in &_users {
        println!("{:?}", user);
    }
}
