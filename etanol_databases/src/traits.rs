use rusqlite::{Connection, Error};

use std::sync::{Arc, Mutex};

use crate::Column;

pub trait Database {
    fn createTable(name: String, columns: Vec<Column>) -> String;
    fn databaseType(columnType: String) -> String;
    fn formatQuery(query: String) -> String;

    fn createConnection(url: String) -> Result<(), Error>;
    fn getConnection() -> Arc<Mutex<Option<Connection>>>;
    fn execute(sql: String, params: &[String]) -> Result<(), Error>;
}
