use std::env;
use std::path::PathBuf;

use data_encoding::HEXUPPER;
use dotenv::dotenv;
use ring::digest::{Context, SHA256};

use etanol_compiler::{Compiler, DatabaseConfig, ParsedToken};
use etanol_utils::EtanolError;

pub fn hash(content: String) -> String {
    let mut context = Context::new(&SHA256);

    context.update(content.as_bytes());

    HEXUPPER.encode(context.finish().as_ref())
}

pub fn findConfig(tokens: &Vec<ParsedToken>) -> Vec<(String, String)> {
    let config = tokens
        .iter()
        .find(|x| match x {
            ParsedToken::DatabaseConfigs(_) => true,
            _ => false,
        })
        .unwrap();

    if let Some(configs) = config.tokenValue() {
        let mut formatedConfigs = vec![];

        dotenv().ok();

        for config in &configs {
            match config {
                DatabaseConfig::Env(name, value) => {
                    if let Some(envValue) = findEnv(value.clone()) {
                        formatedConfigs.push((name.clone(), envValue));

                        continue;
                    }

                    EtanolError::new(
                        format!("Cannot find '{}' in env", value),
                        "NotHaveInEnv".to_string(),
                    );
                }
                DatabaseConfig::Value(name, value) => {
                    formatedConfigs.push((name.clone(), value.clone()));
                }
            }
        }

        return formatedConfigs;
    }

    EtanolError::new(
        "Cannot find database configs".to_string(),
        "DatabaseConfigError".to_string(),
    );

    vec![]
}

pub fn compileSchema() -> Vec<ParsedToken> {
    let pathString = "etanol/schema.etanol";
    let path = PathBuf::from(pathString);

    if !path.exists() {
        EtanolError::new(
            "Cannot find etanol/schema.etanol".to_string(),
            "SchemaFileNotFound".to_string(),
        );
    }

    let compiler = Compiler::new(pathString.to_string());
    compiler.run()
}

fn findEnv(name: String) -> Option<String> {
    for (key, value) in env::vars() {
        if key == name {
            return Some(value);
        }
    }

    None
}
