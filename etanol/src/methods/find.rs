use etanol_databases::Database;
use std::fmt::Debug;

use etanol_databases::{FindQuery, ModelWhere, Value};

pub trait FindTrait {
    fn from(_: Vec<(String, String)>) -> Self;
    fn keys() -> Vec<String>;
}

#[derive(Debug)]
pub enum LoadError {
    FieldNotFound(String),
}

// #[derive(Debug)]
pub struct Find<M: FindTrait + Debug> {
    _model: M,
    query: FindQuery,
}

impl<M: FindTrait + Debug> Find<M> {
    pub fn new(table: String, _model: M) -> Self {
        Self {
            _model,
            query: FindQuery::new(table),
        }
    }

    pub fn field<V: Value>(&mut self, value: ModelWhere<V>) -> &mut Self {
        self.query.field(value);

        self
    }

    pub fn take(&mut self, value: i64) -> &mut Self {
        self.query.take(value);

        self
    }

    pub fn skip(&mut self, value: i64) -> &mut Self {
        self.query.skip(value);

        self
    }

    pub fn load(&mut self) -> Result<Vec<M>, LoadError> {
        let keys = M::keys();

        for field in &self.query.fields() {
            let field = field.split(" ").collect::<Vec<_>>()[0].to_string();

            if !keys.contains(&field) {
                return Err(LoadError::FieldNotFound(field));
            }
        }

        let users =
            Database::executeWithResults(self.query.sql(M::keys()), &self.query.params()).unwrap();

        return Ok(users
            .into_iter()
            .map(|row| M::from(row))
            .collect::<Vec<M>>());
    }
}
