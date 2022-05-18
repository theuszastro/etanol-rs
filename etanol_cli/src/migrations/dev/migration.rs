use std::fs::{create_dir, read_dir, write};
use std::path::PathBuf;

use etanol_databases::Migration;
use etanol_utils::{EtanolError, TableColumn};

use chrono::Local;

pub fn createMigration(name: String, content: String) -> String {
    let timestamp = Local::now().timestamp_millis();
    let name = format!("{}_{}", timestamp, name);

    let migrationDir = format!("etanol/migrations/{}", name);
    let migration = PathBuf::from(format!("{}/migration.sql", migrationDir));

    create_dir(PathBuf::from(migrationDir.clone())).unwrap();
    write(migration, content).unwrap();

    name
}

pub fn createTableMigration(migration: &mut Migration, name: String, columns: Vec<TableColumn>) {
    let table = migration.createTable(name.clone());

    for column in columns {
        table
            .addColumn(column.name, column.columnType)
            .primaryKey(column.isPrimary)
            .nullable(column.isOptional)
            .default(column.default);
    }
}

pub fn createMigrationFolder(name: String) {
    let migrations = "etanol/migrations";
    let migrationsPath = PathBuf::from(migrations);

    if migrationsPath.exists() {
        for file in read_dir(migrationsPath).unwrap() {
            let fileName = file.unwrap();

            if fileName.file_name().to_str().unwrap().contains(&name) {
                EtanolError::new(
                    format!("Migration '{}' already exists", name),
                    "MigrationNameCollision".to_string(),
                );
            }
        }
    } else {
        create_dir(migrationsPath).unwrap();
    }
}
