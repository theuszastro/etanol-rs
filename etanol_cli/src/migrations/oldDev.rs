use etanol_compiler::{Compiler, DatabaseConfig, ParsedToken};
use etanol_databases::Sqlite;

use chrono::Local;

use std::fs::{create_dir, metadata, write};
use std::path::PathBuf;

fn exists(path: &str) -> bool {
    match metadata(PathBuf::from(path)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn migrate_dev(name: String) {
    let compiler = Compiler::new("etanol/models.etanol".to_string());
    let result = compiler.run();

    let databaseConfig = result
        .iter()
        .find(|x| match x {
            ParsedToken::DatabaseConfigs(_) => true,
            _ => false,
        })
        .unwrap();

    match databaseConfig {
        ParsedToken::DatabaseConfigs(configs) => {
            let database = configs.iter().find(|x| x.key() == "database").unwrap();
            let url = configs.iter().find(|x| x.key() == "database_url").unwrap();

            let basePath = "etanol/migrations";
            let name = format!("{}_{}", Local::now().timestamp_millis(), name);
            let folder = format!("{}/{}", basePath, name);
            let path = format!("{}/{}/migration.sql", basePath, name);

            if !exists(basePath) {
                create_dir(basePath).unwrap();
            }

            let mut file = None;

            match url {
                DatabaseConfig::Env(_, value) => {}
                DatabaseConfig::Value(_, value) => {
                    file = Some(value.clone());
                }
            }

            let mut sqlite = Sqlite::new(file.unwrap());
            let migration = sqlite.migration();

            for token in result.iter() {
                match token {
                    ParsedToken::Table(tableName, columns) => {
                        let table = migration.createTable(tableName.clone());

                        for column in columns {
                            table.createColumn(
                                column.name.clone(),
                                column.columnType.clone(),
                                column.optional,
                                column.primaryKey,
                                column.default.clone(),
                            );
                        }
                    }
                    _ => {}
                }
            }

            create_dir(PathBuf::from(folder)).unwrap();

            write(path, migration.toSql().as_bytes()).unwrap();

            sqlite.execute();
        }
        _ => {}
    }
}
