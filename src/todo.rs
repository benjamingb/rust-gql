use crate::result::Result;
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, Debug, Clone, FromRow)]
pub struct Todo {
    pub id: i32,
    pub body: String,
    pub complete: String,
}

impl Todo {
    pub async fn list(pool: &PgPool) -> Result<Vec<Todo>> {
        let rowset = sqlx::query_as(r#"SELECT * FROM todo"#)
            .fetch_all(pool)
            .await?;

        Ok(rowset)
    }
}
