use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use dotenv::dotenv;

pub use etanol_compiler::TableColumn;
use etanol_compiler::{Compiler, DatabaseConfig, ParsedToken};

use crate::{Configs, EtanolError};

mod methods;
use methods::findEnv;

lazy_static::lazy_static! {
    static ref TOKENS: Arc<Mutex<Option<Vec<ParsedToken>>>> = Arc::new(Mutex::new(None));
    static ref CONFIGS: Arc<Mutex<Option<Configs>>> = Arc::new(Mutex::new(None));
    static ref TABLES: Arc<Mutex<Option<Vec<(String, Vec<TableColumn>)>>>> = Arc::new(Mutex::new(None));
}

pub fn readTokens() -> Vec<ParsedToken> {
    if let Some(tokens) = &*TOKENS.lock().unwrap() {
        return tokens.clone();
    }

    if !PathBuf::from("etanol/schema.etanol".to_string()).exists() {
        EtanolError::new(
            "Cannot find etanol/schema.etanol".to_string(),
            "SchemaFileNotFound".to_string(),
        );
    }

    let compiler = Compiler::new("etanol/schema.etanol".to_string());
    let tokens = compiler.run();

    *TOKENS.lock().unwrap() = Some(tokens.clone());

    return tokens;
}

pub fn readConfig() -> Configs {
    if let Some(configs) = &*CONFIGS.lock().unwrap() {
        return configs.clone();
    }

    // let tokens = TOKENS.lock().unwrap().clone().unwrap();

    let tokens = readTokens();

    let mut configs = Configs::new();

    for token in tokens {
        match token {
            ParsedToken::DatabaseConfigs(conf) => {
                dotenv().ok();

                for config in conf {
                    match config {
                        DatabaseConfig::Value(name, value) => configs.0.push((name, value)),
                        DatabaseConfig::Env(name, value) => {
                            if let Some(envValue) = findEnv(value.clone()) {
                                configs.0.push((name, envValue));

                                continue;
                            }

                            EtanolError::new(
                                format!("Cannot find '{}' in env", value),
                                "NotHaveInEnv".to_string(),
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }

    configs
}

pub fn readTables() -> Vec<(String, Vec<TableColumn>)> {
    let tokens = readTokens();

    tokens
        .iter()
        .filter(|x| match x {
            ParsedToken::Table(..) => true,
            _ => false,
        })
        .map(|x| match x {
            ParsedToken::Table(name, columns) => (name.clone(), columns.clone()),
            _ => ("".to_string(), vec![]),
        })
        .collect::<Vec<_>>()
}
