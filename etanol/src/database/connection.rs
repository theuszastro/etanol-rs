use etanol::{Database, Env};

pub fn create_connection() {
    Database::createConnection(Env::take("DATABASE_URL".to_string())).unwrap();
}
