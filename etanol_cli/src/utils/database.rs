use etanol_databases::{Database, Migration, Sqlite};
use etanol_utils::EtanolError;

use std::fs::File;
use std::path::PathBuf;

use uuid::Uuid;

use super::hash;

pub fn executeInDatabase(configs: &Vec<(String, String)>, name: String, content: String) {
    let database = configs.iter().find(|x| x.0.as_str() == "database").unwrap();
    let url = configs
        .iter()
        .find(|x| x.0.as_str() == "database_url")
        .unwrap();

    match database.1.as_str() {
        "sqlite" => sqlite(url.1.clone(), name, content),
        _ => {
            EtanolError::new(
                format!("Config database '{}' not supported", database.1),
                "DatabaseNotSupported".to_string(),
            );
        }
    }
}

fn migrationTable<T: Database>() -> String {
    let mut migration = Migration::new();

    let columns = vec![
        ("id".to_string(), "String".to_string(), true, false, None),
        (
            "checksum".to_string(),
            "String".to_string(),
            false,
            false,
            None,
        ),
        (
            "migration_name".to_string(),
            "String".to_string(),
            false,
            false,
            None,
        ),
    ];

    let table = migration.createTable("_etanol_migrations".to_string());

    for (name, columnType, isPrimary, isOptional, default) in columns {
        table
            .addColumn(name, columnType)
            .primaryKey(isPrimary)
            .nullable(isOptional)
            .default(default);
    }

    migration.make::<Sqlite>().join("")
}

fn sqlite(path: String, name: String, content: String) {
    if !path.ends_with(".sqlite") {
        EtanolError::new(
            format!("Config database_url '{}' not supported", path),
            "DatabaseNotSupported".to_string(),
        );
    }

    createDatabase(format!("etanol/{}", path));

    match Sqlite::createConnection(format!("etanol/{}", path)) {
        Ok(_) => {
            let connection = Sqlite::getConnection();
            let connection = connection.lock().unwrap();

            let id = Uuid::new_v4().to_string();
            let checksum = hash(content.clone());

            if let Some(conn) = &*connection {
                conn.execute(&migrationTable::<Sqlite>(), []).unwrap();
                conn.execute(&Sqlite::formatQuery(content), []).unwrap();

                conn.execute(
                    &"INSERT INTO _etanol_migrations VALUES(?1, ?2, ?3);",
                    [id, checksum, name],
                )
                .unwrap();
            }
        }
        _ => {
            EtanolError::new(
                format!("Could not create database connection"),
                "DatabaseConnection".to_string(),
            );
        }
    }
}

fn createDatabase(path: String) {
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
