use crate::utils::readString;
use crate::utils::{ParsedToken, Pointer, TableColumn, Token};

fn readDecorators(
    pointer: &mut Pointer,
    columnType: String,
    columns: &Vec<TableColumn>,
) -> Vec<(String, String)> {
    let mut decorators: Vec<(String, String)> = vec![];

    loop {
        match pointer.token.clone() {
            Some(Token::Decorator(name)) => match name.as_str() {
                "id" => {
                    if columns.iter().find(|x| x.isPrimary).is_some()
                        || decorators.contains(&("id".to_string(), "".to_string()))
                    {
                        pointer.error("duplicate primary key");
                    }

                    pointer.take("Decorator");

                    decorators.push((name, "".to_string()));
                }
                "default" => {
                    if decorators.iter().find(|x| x.0 == "default").is_some() {
                        pointer.error("duplicate default");
                    }

                    pointer.take("Decorator");

                    let startBracket = pointer.toEqual("Brackets", "(");
                    if !startBracket {
                        pointer.error("Expected a '('");
                    }

                    let mut decorator = (name.clone(), "".to_string());

                    match pointer.token.clone() {
                        Some(Token::Identifier(value)) => {
                            pointer.take("Identifier");

                            decorator = (name, value);
                        }
                        Some(Token::Punctuation(pun)) if pun == "\"" => {
                            let value = readString(pointer);

                            if value.is_empty() {
                                pointer.error("Expected a value for the default");
                            }

                            decorator = (name, value);
                        }
                        _ => {
                            pointer.error("Expected a default value");
                        }
                    }

                    match columnType.as_str() {
                        "String" => {}
                        "Integer" => {
                            if !decorator.1.parse::<i32>().is_ok() {
                                pointer
                                    .error(&format!("Expected a integer found '{}'", decorator.1));
                            }
                        }
                        "Boolean" => {
                            if !decorator.1.parse::<bool>().is_ok() {
                                pointer.error(&format!(
                                    "Expected a true or false found '{}'",
                                    decorator.1
                                ));
                            }
                        }
                        _ => {}
                    }

                    let endBracket = pointer.toEqual("Brackets", ")");
                    if !endBracket {
                        pointer.error("Expected a ')'");
                    }

                    decorators.push(decorator);
                }
                _ => {}
            },
            _ => break,
        }
    }

    return decorators;
}

pub fn table(pointer: &mut Pointer) {
    pointer.take("Keyword");

    let tableName = pointer.take("Identifier");
    if tableName.is_none() {
        pointer.error("Expected a 'Identifier'");
    }

    let startBracket = pointer.toEqual("Brackets", "{");
    if !startBracket {
        pointer.error(&format!("Expected a '{}'", "{"));
    }

    let mut columns: Vec<TableColumn> = Vec::new();

    loop {
        if pointer.toEqual("Brackets", "}") {
            break;
        }

        match pointer.token.clone() {
            None | Some(Token::EOF) => {
                pointer.error(&format!("Expected a '{}'", "}"));
            }
            Some(Token::Identifier(name)) => {
                if columns.iter().find(|x| x.name == name).is_some() {
                    pointer.error(&format!("duplicate column '{}'", name));
                }

                match pointer.previewNext(true) {
                    Some(Token::TableType(_)) => {}
                    _ => {
                        pointer.error(&format!(
                            "Expected a '{}'",
                            pointer.tokenizer.tableTypes.join(" or ")
                        ));
                    }
                }

                pointer.take("Identifier");

                let columnType = pointer.take("TableType").unwrap().tokenValue();
                let isOptional = pointer.toEqual("Punctuation", "?");

                let decorators = readDecorators(pointer, columnType.clone(), &columns);
                let isPrimary = decorators.contains(&(String::from("id"), String::new()));

                let default = {
                    match decorators.iter().find(|x| x.0 == "default") {
                        Some(data) => Some(data.1.clone()),
                        _ => None,
                    }
                };

                let tableColumn = TableColumn {
                    columnType,
                    name,
                    isOptional,
                    isPrimary,
                    default,
                };

                columns.push(tableColumn);
            }
            _ => break,
        }
    }

    pointer
        .tokens
        .push(ParsedToken::Table(tableName.unwrap().tokenValue(), columns));
}
