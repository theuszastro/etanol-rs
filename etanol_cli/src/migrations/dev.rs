use etanol_compiler::{ParsedToken, TableColumn};
use etanol_databases::{Migration, Sqlite};
use etanol_utils::EtanolError;

use std::fs::{create_dir, read_dir, write};
use std::path::PathBuf;

use chrono::Local;

use crate::utils::database::executeInDatabase;
use crate::utils::{compileSchema, findConfig};

pub fn migrate_dev(name: String) {
    let tokens = compileSchema();
    let configs = findConfig(&tokens);

    let mut migration = Migration::new();

    for (name, columns) in filterTables(&tokens) {
        let table = migration.createTable(name);

        for column in columns {
            table
                .addColumn(column.name, column.columnType)
                .primaryKey(column.isPrimary)
                .nullable(column.isOptional)
                .default(column.default);
        }
    }

    let content = migration.make::<Sqlite>().join("");

    executeInDatabase(&configs, name.clone(), content.clone());
    createMigrationFolder(name, content.clone());
}

fn createMigrationFolder(name: String, content: String) {
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

    let timestamp = Local::now().timestamp_millis();
    let name = format!("{}_{}", timestamp, name);
    let migrationDir = format!("{}/{}", migrations, name);
    let migration = PathBuf::from(format!("{}/migration.sql", migrationDir));

    create_dir(PathBuf::from(migrationDir.clone())).unwrap();
    write(migration, content).unwrap();
}

fn filterTables(tokens: &Vec<ParsedToken>) -> Vec<(String, Vec<TableColumn>)> {
    let mut tables = vec![];

    let filteredTokens = tokens
        .iter()
        .filter(|x| match x {
            ParsedToken::Table(..) => true,
            _ => false,
        })
        .collect::<Vec<_>>();

    for token in filteredTokens {
        match token.clone() {
            ParsedToken::Table(name, columns) => tables.push((name, columns)),
            _ => {}
        }
    }

    tables
}
