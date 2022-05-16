struct Query {
    skip: Option<i64>,
    take: Option<i64>,
}

pub struct User {
    pub id: String,
    pub name: String,

    query: Query,
}

impl User {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            query: Query {
                skip: None,
                take: None,
            },
        }
    }
}

pub trait Model {
    fn toInsertSql(&mut self) -> String;
    fn skip(&mut self, amount: i64) -> &mut Self;
    fn take(&mut self, amount: i64) -> &mut Self;
    fn execute(&mut self);
}

impl Model for User {
    fn toInsertSql(&mut self) -> String {
        format!(
            "INSERT INTO users (id, name) VALUES ('{}', '{}')",
            self.id, self.name
        )
    }

    fn skip(&mut self, amount: i64) -> &mut Self {
        self.query = Query {
            skip: Some(amount),
            take: self.query.take,
        };

        self
    }

    fn take(&mut self, amount: i64) -> &mut Self {
        self.query = Query {
            skip: self.query.skip,
            take: Some(amount),
        };

        self
    }

    fn execute(&mut self) {
        let mut sql = self.toInsertSql();

        let Query { skip, take } = self.query;

        if let Some(skip) = skip {
            sql += &format!(" OFFSET {}", skip);
        }

        if let Some(take) = take {
            sql += &format!(" LIMIT {}", take);
        }

        println!("{}", sql);
    }
}
