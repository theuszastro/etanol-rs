use etanol_databases::{Database, Migration, Sqlite};
use etanol_utils::{readConfig, EtanolError};

mod sqlite;
use sqlite::*;

pub fn executeInDatabase(name: String, content: String) {
    let configs = readConfig();

    let database = configs.take("database".to_string()).unwrap();
    let url = configs.take("database_url".to_string()).unwrap();

    match database.as_str() {
        "sqlite" => sqlite(url.clone(), name, content),
        _ => {
            EtanolError::new(
                format!("Config database '{}' not supported", database),
                "DatabaseNotSupported".to_string(),
            );
        }
    }
}

pub fn migrationTable<T: Database>() -> String {
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
