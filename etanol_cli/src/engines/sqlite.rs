use uuid::Uuid;

use etanol_databases::{Database, Sqlite};
use etanol_utils::{hash, EtanolError};

use super::migrationTable;

pub fn sqlite(path: String, name: String, content: String) {
    match Sqlite::createConnection(format!("etanol/{}", path)) {
        Ok(_) => {
            let id = Uuid::new_v4().to_string();
            let checksum = hash(content.clone());

            Sqlite::execute(migrationTable::<Sqlite>(), &vec![]).unwrap();
            Sqlite::execute(Sqlite::formatQuery(content), &vec![]).unwrap();
            Sqlite::execute(
                "INSERT INTO _etanol_migrations (id, migration_name, checksum) VALUES(?1, ?2, ?3);"
                    .to_string(),
                &vec![id, name, checksum],
            )
            .unwrap();
        }
        _ => {
            EtanolError::new(
                format!("Could not create database connection"),
                "DatabaseConnection".to_string(),
            );
        }
    }
}
