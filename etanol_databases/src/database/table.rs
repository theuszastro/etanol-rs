use super::Column;

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

impl Table {
    pub fn addColumn(&mut self, name: String, columnType: String) -> &mut Column {
        let index = self.columns.len();

        self.columns.push(Column {
            name,
            columnType,
            isOptional: false,
            default: None,
            isPrimary: false,
        });

        &mut self.columns[index]
    }
}
