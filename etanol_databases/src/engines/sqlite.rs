use std::fs::File;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use etanol_utils::EtanolError;

use rusqlite::{params_from_iter, Connection, Error};

use crate::{Column, Database};

pub struct Sqlite {}
pub struct SqliteError(String);

lazy_static::lazy_static! {
    static ref DATABASE_CONNECTION: Arc<Mutex<Option<Connection>>> = Arc::new(Mutex::new(None));
}

impl Database for Sqlite {
    fn createConnection(url: String) -> Result<(), Error> {
        if !url.ends_with(".sqlite") {
            EtanolError::new(
                format!("Config database_url '{}' not supported", url),
                "DatabaseNotSupported".to_string(),
            );
        }

        createDatabase(url.clone());

        if DATABASE_CONNECTION.lock().unwrap().is_none() {
            let connection = Connection::open(url)?;

            *DATABASE_CONNECTION.lock().unwrap() = Some(connection);
        }

        Ok(())
    }

    fn getConnection() -> Arc<Mutex<Option<Connection>>> {
        if DATABASE_CONNECTION.lock().unwrap().is_none() {
            EtanolError::new(
                format!("Connection to database is not initialized. Please call `createConnection` first."),
                "DatabaseConnection".to_string(),
            );
        }

        DATABASE_CONNECTION.clone()
    }

    fn createTable(name: String, columns: Vec<Column>) -> String {
        let mut sql = String::from("--Create Table\n");

        sql.push_str(&format!("CREATE TABLE IF NOT EXISTS \"{}\" (\n", name));

        for column in columns {
            sql.push_str(&format!(
                "   \"{}\" {}",
                column.name,
                Sqlite::databaseType(column.columnType)
            ));

            if !column.isOptional {
                sql.push_str(" NOT NULL");
            }

            if column.isPrimary {
                sql.push_str(" PRIMARY KEY");
            }

            // if let Some(value) = column.default {
            //     sql.push_str(&format!(" DEFAULT \"{}\"", value));
            // }

            sql.push_str(",\n");
        }

        sql.remove(sql.len() - 2);
        sql.push_str(");\n");

        sql
    }

    fn databaseType(columnType: String) -> String {
        match columnType.as_str() {
            "Integer" => String::from("INTEGER"),
            "String" => String::from("TEXT"),
            "Boolean" => String::from("BOOLEAN"),
            _ => {
                panic!("Invalid column type: {}", columnType);
            }
        }
    }

    fn formatQuery(query: String) -> String {
        query
            .split("\n")
            .filter(|x| !x.to_string().starts_with("--"))
            .collect::<Vec<&str>>()
            .join("")
            .split("\"")
            .collect::<Vec<&str>>()
            .join("")
            .to_string()
    }

    fn execute(sql: String, params: &[String]) -> Result<(), Error> {
        let connection = Sqlite::getConnection();
        let connection = connection.lock().unwrap();

        if let Some(conn) = &*connection {
            conn.execute(&sql, params_from_iter(params.iter())).unwrap();
        }

        Ok(())
    }
}

fn createDatabase(path: String) {
    println!("Creating database: {}", path);

    let path = PathBuf::from(path);

    if !path.exists() {
        match File::create(path) {
            Ok(_) => {}
            Err(e) => {
                EtanolError::new(
                    format!("Could not create database file: {}", e),
                    "FileSystemError".to_string(),
                );
            }
        }
    }
}
