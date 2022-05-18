use etanol_utils::readConfig;
use std::fs::{read_dir, write};

use etanol_databases::{Migration, Sqlite};
use etanol_utils::{readTables, readTokens};

// mudar apra etanol_databases;
use crate::engines::executeInDatabase;

use crate::generation::createModel;

mod migration;
use migration::{createMigration, createMigrationFolder, createTableMigration};

pub fn migrate_dev(name: String) {
    readTokens();

    let configs = readConfig();

    createMigrationFolder(name.clone());

    let mut migration = Migration::new();

    for (name, columns) in readTables() {
        createTableMigration(&mut migration, name.clone(), columns.clone());

        createModel(name, columns);
    }

    createMods();
    createConnection();

    let database = configs.take("database".to_string()).unwrap();

    let content = {
        match database.as_str() {
            "sqlite" => migration.make::<Sqlite>().join(""),
            _ => "".to_string(),
        }
    };

    let migrationName = createMigration(name.clone(), content.clone());
    executeInDatabase(migrationName, content.clone());
}

fn createMods() {
    let mut content = String::new();

    let files = read_dir("src/database/models").unwrap();

    for file in files {
        let filename = file.unwrap().file_name();
        let filename = filename.to_str().unwrap();
        let name = filename.to_string().replace(".rs", "");

        content.push_str(&format!("pub mod {};\n", name));
    }

    write("src/database/models/mod.rs", content).unwrap();
    write(
        "src/database/mod.rs",
        "pub mod connection;\n\npub use connection::*;\n\nmod models;\n\npub use models::*;",
    )
    .unwrap();
}

fn createConnection() {
    let configs = readConfig();

    let database = configs.take("database".to_string()).unwrap();
    let url = configs.take("database_url".to_string()).unwrap();

    let content = {
        match database.as_str() {
            "sqlite" => format!(
                "use etanol::Sqlite;\n\npub fn create_connection() {{\n Sqlite::createConnection(\"etanol/{}\");\n }}",
                url
            ),
            _ => "".to_string(),
        }
    };

    write("src/database/connection.rs", content).unwrap();
}
