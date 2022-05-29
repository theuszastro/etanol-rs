use etanol::{Find, FindTrait, Insert, Update};

use std::default::Default;



#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: Option<i64>,
    pub isAdmin: Option<bool>,
}

impl User {
    pub fn find() -> Find<Self> {
        Find::new("User".to_string(), Self::default())
    }

    pub fn update() -> Update<Self> {
        Update::new("User".to_string(), User::default())
    }

    pub fn insert(&self) -> Insert {
        Insert::new(
            String::from("User"),
            vec![
                ("id".to_string(), Insert::formatType(self.id.clone(), None)),
                (
                    "name".to_string(),
                    Insert::formatType(self.name.clone(), None),
                ),
                (
                    "age".to_string(),
                    Insert::formatType(self.age.clone(), None),
                ),
                (
                    "isAdmin".to_string(),
                    Insert::formatType(self.isAdmin.clone(), None),
                ),
            ],
        )
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            name: "".to_string(),
            age: None,
            isAdmin: None,
        }
    }
}

impl FindTrait for User {
    fn from(values: Vec<(String, String)>) -> Self {
        let mut _struct = Self::default();

        for (key, value) in values {
            match key.as_str() {
                "id" => _struct.id = value,
                "name" => _struct.name = value,
                "age" => match value.as_str() {
                    "None" => _struct.age = None,
                    _ => _struct.age = Some(value.parse::<i64>().unwrap()),
                },
                "isAdmin" => match value.as_str() {
                    "None" => _struct.isAdmin = None,
                    _ => _struct.isAdmin = Some(value.parse::<bool>().unwrap()),
                },

                _ => {}
            }
        }

        _struct
    }

    fn keys() -> Vec<String> {
        vec![
            "id".to_string(),
            "name".to_string(),
            "age".to_string(),
            "isAdmin".to_string(),
        ]
    }
}
