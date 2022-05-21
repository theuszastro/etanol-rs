#![allow(non_snake_case)]

mod methods;

// https://stackoverflow.com/questions/60396593/how-do-i-use-rusqlites-rowget-method-for-a-type-which-i-dont-know-at-compile

pub use etanol_databases::{Database, FindWhere};
pub use etanol_utils::Env;

pub use methods::*;

pub enum Operator<V: Value> {
    Equal(V),
    NotEqual(V),
    GreaterThan(V),
    GreaterThanOrEqual(V),
    LessThan(V),
    LessThanOrEqual(V),
    Contains(V),
}

impl<V: Value> Operator<V> {
    fn value(&self) -> String {
        match self {
            Operator::Equal(value)
            | Operator::NotEqual(value)
            | Operator::Contains(value)
            | Operator::GreaterThan(value)
            | Operator::GreaterThanOrEqual(value)
            | Operator::LessThan(value)
            | Operator::LessThanOrEqual(value) => value.toValue(None),
        }
    }
}
