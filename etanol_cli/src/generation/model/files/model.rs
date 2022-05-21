use etanol_databases::Migration;
use etanol_utils::TableColumn;

use super::add;

use super::super::{createDefault, createField, createFrom, createInsertValue};

pub fn createModel(migration: &mut Migration, name: String, columns: Vec<TableColumn>) {
    let mut fields = vec![];
    let mut defaults = vec![];

    let mut findKeys = vec![];
    let mut findFrom = vec![];
    let mut insertValues = vec![];

    let table = migration.createTable(name.clone());

    for column in columns {
        fields.push(createField(&column));
        defaults.push(createDefault(&column));
        insertValues.push(createInsertValue(&column));

        findFrom.push(createFrom(&column));
        findKeys.push(format!("\"{}\".to_string()", column.name));

        table
            .addColumn(column.name, column.columnType)
            .primaryKey(column.isPrimary)
            .nullable(column.isOptional)
            .default(column.default);
    }

    let imports = createImports();
    let struct_ = createStruct(name.clone(), fields);
    let implStruct = createImpl(name.clone(), insertValues);
    let implDefault = createImplDefault(name.clone(), defaults);
    let implFind = createImplFind(name.clone(), findFrom, findKeys);

    add(
        format!("src/database/models/{}.rs", name.to_lowercase()),
        format!(
            "{}\n\n{}\n\n{}\n\n{}\n\n{}",
            imports, struct_, implStruct, implDefault, implFind
        ),
    )
}

fn createImports() -> String {
    r"
        use etanol::{{Find, FindTrait, Insert}};
        use std::default::Default;
    "
    .to_string()
}

fn createStruct(name: String, fields: Vec<String>) -> String {
    format!(
        r"
    #[derive(Debug)]
    pub struct {} {{
        {}
    }}
",
        name,
        fields.join(",\n")
    )
}

fn createImpl(name: String, keys: Vec<String>) -> String {
    format!(
        r"
    impl {} {{
        pub fn find() -> Find<Self> {{
            Find::new({}.to_string(), Self::default())
        }}

        pub fn insert(&self) -> Insert {{
            Insert::new(
                String::from({}{}{}),
                vec![{}]
            )
        }}
    }}
    ",
        name,
        format!("\"{}\"", name),
        "\"",
        name,
        "\"",
        keys.join(", ")
    )
}

fn createImplDefault(name: String, defaults: Vec<String>) -> String {
    format!(
        r"
    impl Default for {} {{
        fn default() -> Self {{
            Self {{
                {}
            }}
        }}
    }}
    ",
        name,
        defaults.join(",\n")
    )
}

fn createImplFind(name: String, from: Vec<String>, keys: Vec<String>) -> String {
    format!(
        r"
    impl FindTrait for {} {{
        fn from(values: Vec<(String, String)>) -> Self {{
            let mut _struct = Self::default();

            for (key, value) in values {{
                match key.as_str() {{
                    {}
                    _ => {{}},
                }}
            }}

            _struct
        }}

        fn keys() -> Vec<String> {{
            vec![{}]
        }}
    }}
    ",
        name,
        from.join(",\n"),
        keys.join(", ")
    )
}
