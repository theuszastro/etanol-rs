#![allow(non_snake_case)]

use rusqlite::{params, Connection};
use std::fs;

use regex::Regex;

// use

pub struct Column {
    name: String,
    r#type: String,
    isOptional: bool,
    default: Option<String>,
    isPrimary: bool,
}

pub struct Table {
    name: String,
    columns: Vec<Column>,
}

pub struct Sqlite {
    pub filename: String,
    pub migrations: Vec<Migration>,
}

impl Table {
    pub fn createColumn(
        &mut self,
        name: String,
        r#type: String,
        isOptional: bool,
        isPrimary: bool,
        default: Option<String>,
    ) {
        let r#type = self.databaseType(r#type);

        self.columns.push(Column {
            name,
            r#type,
            isOptional,
            default,
            isPrimary,
        });
    }

    fn databaseType(&mut self, r#type: String) -> String {
        match r#type.as_str() {
            "Integer" => "INTEGER".to_string(),
            "String" => "TEXT".to_string(),
            "Boolean" => "BOOLEAN".to_string(),
            _ => {
                println!("Unsupported type: {}", r#type);

                std::process::exit(1);
            }
        }
    }
}

pub struct Migration {
    pub tables: Vec<Table>,
}

impl Sqlite {
    pub fn new(filename: String) -> Sqlite {
        Sqlite {
            migrations: vec![],
            filename,
        }
    }

    pub fn migration(&mut self) -> &mut Migration {
        let index = self.migrations.len();

        self.migrations.push(Migration { tables: Vec::new() });

        &mut self.migrations[index]
    }

    fn createDatabase(&self) {
        match self.filename.split(".").last().unwrap() {
            "db" | "sqlite" => {
                let path = std::path::PathBuf::from(format!("etanol/{}", self.filename));

                let file = fs::File::create(path).unwrap();
            }
            _ => {}
        }
    }

    pub fn execute(&mut self) {
        let allMigrations = fs::read_dir("etanol/migrations").unwrap();
        let mut migrations = vec![];

        self.createDatabase();

        let path = std::path::PathBuf::from(format!("etanol/{}", self.filename));

        let connection = Connection::open(path).unwrap();
        let migrations_table = format!("CREATE TABLE IF NOT EXISTS etanol_migrations\" {}\nid INTEGER PRIMARY KEY,\n name TEXT NOT NULL,\nchecksum TEXT NOT NULL,\ntimestamp INTEGER NOT NULL\n{}", "(", ");");
        let migrations_table = migrations_table
            .split("\n")
            .collect::<Vec<&str>>()
            .join(" ");
        let migrations_table = migrations_table.split("\"").collect::<Vec<&str>>().join("");

        connection.execute(&migrations_table, []).unwrap();

        for migration in allMigrations
            .map(|x| x.unwrap())
            .collect::<Vec<fs::DirEntry>>()
        {
            let name = migration.file_name().into_string().unwrap();
            let content =
                fs::read_to_string(format!("etanol/migrations/{}/migration.sql", name)).unwrap();

            let content = content.split("\n").collect::<Vec<&str>>().join(" ");
            let content = content.split("\"").collect::<Vec<&str>>().join("");

            migrations.push((name, content));
        }

        // println!(
        //     "Found {:?} migrations",
        //     allMigrations
        //         .map(|x| x.unwrap())
        //         .collect::<Vec<fs::DirEntry>>()
        // );
    }
}

impl Migration {
    pub fn createTable(&mut self, name: String) -> &mut Table {
        let index = self.tables.len();

        self.tables.push(Table {
            name,
            columns: Vec::new(),
        });

        &mut self.tables[index]
    }

    pub fn toSql(&mut self) -> String {
        let mut sql = String::from("");

        for table in &self.tables {
            let mut tableSql = String::from("--Create Table \n");

            tableSql.push_str("CREATE TABLE ");
            tableSql.push_str(&format!("\"{}\" (\n", table.name));

            for column in &table.columns {
                let mut columnSql = String::from("");

                columnSql.push_str(&format!("   \"{}\" {}", column.name, column.r#type));

                if !column.isOptional {
                    columnSql.push_str(" NOT NULL");
                }

                if let Some(default) = &column.default {
                    columnSql.push_str(&format!(" DEFAULT \"{}\"", default));
                }

                if column.isPrimary {
                    columnSql.push_str(" PRIMARY KEY");
                }

                columnSql.push_str(",");
                columnSql.push_str("\n");

                tableSql.push_str(&columnSql);
            }

            tableSql.push_str(");\n");

            sql.push_str(&tableSql);
            sql.push_str("\n");
        }

        sql.pop();
        sql.pop();

        sql
    }
}
