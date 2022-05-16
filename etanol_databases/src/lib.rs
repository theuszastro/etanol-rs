#![allow(non_snake_case)]

mod engines;
mod traits;

pub use engines::sqlite::*;
pub use traits::*;

#[derive(Debug, Clone)]
pub struct Column {
    name: String,
    columnType: String,
    isOptional: bool,
    default: Option<String>,
    isPrimary: bool,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

pub struct Migration {
    pub tables: Vec<Table>,
}

impl Column {
    pub fn default(&mut self, value: Option<String>) -> &mut Self {
        self.default = value;

        self
    }

    pub fn primaryKey(&mut self, value: bool) -> &mut Self {
        self.isPrimary = value;

        self
    }

    pub fn nullable(&mut self, value: bool) -> &mut Self {
        self.isOptional = value;

        self
    }
}

impl Table {
    pub fn addColumn(&mut self, name: String, columnType: String) -> &mut Column {
        let index = self.columns.len();

        self.columns.push(Column {
            name,
            columnType,
            isOptional: false,
            default: None,
            isPrimary: false,
        });

        &mut self.columns[index]
    }
}

impl Migration {
    pub fn new() -> Migration {
        Migration { tables: vec![] }
    }

    pub fn createTable(&mut self, name: String) -> &mut Table {
        let index = self.tables.len();

        self.tables.push(Table {
            name,
            columns: vec![],
        });
        &mut self.tables[index]
    }

    pub fn make<T: Database>(&mut self) -> Vec<String> {
        let mut tables = vec![];

        for table in self.tables.clone() {
            tables.push(T::createTable(table.name, table.columns));
        }

        tables
    }
}
