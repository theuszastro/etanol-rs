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

    let _users = User::find()
        // .field(ModelWhere::GreaterThan("age", 3))
        // .field(FindWhere::("age", "2"))
        // .skip(1)
        .take(1)
        // .many()
        .load()
        .unwrap();

    for user in &_users {
        println!("{:?}", user);
    }

    let result = User::update()
        .field(ModelWhere::Equal("id", "1"))
        .value(QueryValue("name".to_string(), "Jorge".to_string()))
        .execute();

    println!("{:?}", result);
}
