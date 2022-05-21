use crate::Value;

#[derive(Debug, Clone)]
pub enum FindWhere<T: Value> {
    Equal(&'static str, T),
    NotEqual(&'static str, T),
    GreaterThan(&'static str, T),
    GreaterThanOrEqual(&'static str, T),
    LessThan(&'static str, T),
    LessThanOrEqual(&'static str, T),
    Contains(&'static str, T),
}

impl<T: Value> FindWhere<T> {
    pub fn sql(&self, position: usize) -> String {
        match self {
            FindWhere::Equal(name, ..) => format!("{} = ?{}", name, position),
            FindWhere::NotEqual(name, ..) => format!("{} != ?{}", name, position),
            FindWhere::GreaterThan(name, ..) => format!("{} > ?{}", name, position),
            FindWhere::GreaterThanOrEqual(name, ..) => format!("{} >= ?{}", name, position),
            FindWhere::LessThan(name, ..) => format!("{} < ?{}", name, position),
            FindWhere::LessThanOrEqual(name, ..) => format!("{} <= ?{}", name, position),
            FindWhere::Contains(name, ..) => format!("{} LIKE ?{}", name, position),
        }
    }

    pub fn key(&self) -> String {
        match self {
            FindWhere::Equal(key, ..)
            | FindWhere::NotEqual(key, ..)
            | FindWhere::GreaterThan(key, ..)
            | FindWhere::GreaterThanOrEqual(key, ..)
            | FindWhere::LessThan(key, ..)
            | FindWhere::LessThanOrEqual(key, ..)
            | FindWhere::Contains(key, ..) => key.to_string(),
        }
    }

    pub fn value(self) -> String {
        match self {
            FindWhere::Equal(_, v)
            | FindWhere::NotEqual(_, v)
            | FindWhere::GreaterThan(_, v)
            | FindWhere::GreaterThanOrEqual(_, v)
            | FindWhere::LessThan(_, v)
            | FindWhere::LessThanOrEqual(_, v) => v.toValue(None),

            FindWhere::Contains(_, v) => {
                let value = v.toValue(None);

                if &value != "None" {
                    return format!("%{}%", value);
                }
                value
            }
        }
    }
}

#[derive(Debug)]
pub struct FindQuery {
    pub fields: Vec<String>,

    table: String,
    skip: Option<i64>,
    take: Option<i64>,
    values: Vec<String>,
}

impl FindQuery {
    pub fn new(table: String) -> Self {
        Self {
            table,
            skip: None,
            take: None,
            fields: vec![],
            values: vec![],
        }
    }

    pub fn take(&mut self, value: i64) -> &mut Self {
        self.take = Some(value);

        self
    }

    pub fn skip(&mut self, value: i64) -> &mut Self {
        self.skip = Some(value);

        self
    }

    pub fn field<T: Value>(&mut self, value: FindWhere<T>) -> &mut Self {
        self.fields.push(value.sql(self.values.len() + 1));
        self.values.push(value.value());

        self
    }

    pub fn sql(&self, keys: Vec<String>) -> String {
        let mut whereSql = String::new();

        if self.fields.len() > 0 {
            whereSql = format!("WHERE ");

            let mut wheres = vec![];

            for value in self.fields.clone() {
                wheres.push(value);
            }

            whereSql.push_str(&wheres.join(" AND "));
        }

        let mut sql = format!(
            "SELECT {} FROM \"{}\" {}",
            keys.join(", ").trim().to_string(),
            self.table,
            whereSql
        );

        if let Some(take) = self.take {
            if take >= 1 {
                sql.push_str(&format!(" LIMIT {}", take));
            }
        }

        if let Some(skip) = self.skip {
            if skip >= 1 {
                sql.push_str(&format!(" OFFSET {}", skip));
            }
        }

        println!("{}", sql);

        sql.trim().to_string()
    }

    pub fn params(&self) -> Vec<String> {
        if self.values.len() > 0 {
            let mut params = vec![];

            for value in self.values.clone() {
                params.push(value);
            }

            return params;
        }

        return vec![];
    }
}
