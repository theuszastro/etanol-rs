#![allow(non_snake_case)]

mod methods;

// https://stackoverflow.com/questions/60396593/how-do-i-use-rusqlites-rowget-method-for-a-type-which-i-dont-know-at-compile

pub use etanol_databases::{Database, ModelWhere};
pub use etanol_utils::Env;

pub use methods::*;
