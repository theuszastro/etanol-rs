use etanol_utils::readConfig;
use etanol_utils::TableColumn;

use std::fs::{create_dir_all, write};

mod methods;

use methods::createFields;

pub fn createModel(name: String, columns: Vec<TableColumn>) {
    let config = readConfig();

    let mut fields = String::new();
    let mut defaultValues = String::new();
    let mut insertValues = String::new();

    for column in columns {
        let (field, default, values) = createFields(&column);

        fields.push_str(&field);
        defaultValues.push_str(&default);
        insertValues.push_str(&values);
    }

    defaultValues.pop();
    defaultValues.pop();

    insertValues.pop();
    insertValues.pop();

    let engine = match config.take("database".to_string()).unwrap().as_str() {
        "sqlite" => "Sqlite",
        _ => "",
    };

    let model = format!(
        r"use etanol::{{Insert, {}}};
use std::default::Default;

pub struct {} {{
    {}
}}
        
impl Default for {} {{
    fn default() -> Self {{
        Self {{
            {}
        }}
    }}
}}
        
impl {} {{
    pub fn insert(&self) -> Insert {{
        Insert::new::<{}>(
            String::from({}{}{}),
            vec![
                {}
            ],
            {} {{}}
        )
    }}
}}",
        engine,
        name,
        fields,
        name,
        defaultValues,
        name,
        engine,
        "\"",
        name,
        "\"",
        insertValues,
        engine
    );

    create_dir_all("src/database/models").unwrap();

    write(
        format!("src/database/models/{}.rs", name.to_lowercase()),
        model,
    )
    .unwrap();
}
