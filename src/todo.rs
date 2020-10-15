use crate::result::Result;
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub body: String,
    pub complete: String,
}

impl Todo {
    pub async fn list(pool: &PgPool) -> Result<Vec<Todo>> {
        let mut todos = vec![];
        let recs = sqlx::query!(r#"SELECT * FROM todo"#)
            .fetch_all(pool)
            .await?;

        for rec in recs {
            todos.push(Todo {
                id: rec.id
                body:rec.body,
                complete: rec.complete
            });
        }

        Ok(todos)
    }
}
