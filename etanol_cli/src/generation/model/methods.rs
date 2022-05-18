use etanol_utils::TableColumn;

fn getDefaultValue(type_: String, default: Option<String>) -> String {
    if let Some(value) = default {
        return format!("Some({})", value);
    }

    match type_.as_str() {
        "String" => "\"\".to_string()".to_string(),
        "Integer" => "0".to_string(),
        "Boolean" => "false".to_string(),
        _ => String::new(),
    }
}

fn createInsertValue(name: String, default: Option<String>) -> String {
    let value = if let Some(value) = default {
        format!("Some(\"{}\".to_string())", value)
    } else {
        "None".to_string()
    };

    format!(
        "(\"{}\".to_string(), Insert::formatType(self.{}.clone(), {})),\n",
        name, name, value
    )
}

pub fn createFields(column: &TableColumn) -> (String, String, String) {
    let TableColumn {
        columnType,
        name,
        default,
        isOptional,
        ..
    } = column;

    let type_ = parseType(columnType.clone());
    let formatedType = if isOptional | default.is_some() {
        format!("Option<{}>", type_)
    } else {
        type_
    };

    let defaultValue = getDefaultValue(columnType.to_string(), default.clone());
    let insertValues = createInsertValue(name.to_string(), default.clone());

    let field = format!("pub {}: {},\n", name, formatedType);
    let default = format!("{}: {},\n", name, defaultValue);

    (field, default, insertValues)
}

pub fn parseType(type_: String) -> String {
    match type_.as_str() {
        "String" => "String".to_string(),
        "Integer" => "i64".to_string(),
        "Boolean" => "bool".to_string(),
        _ => panic!("Unknown type: {}", type_),
    }
}
