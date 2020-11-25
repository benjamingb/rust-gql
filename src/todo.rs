use crate::interface_todo::ITodoRepo;
use crate::result::Result;
use async_trait::async_trait;
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, Debug, Clone, FromRow)]
pub struct Todo {
    pub id: i32,
    pub body: String,
    pub complete: String,
}

#[async_trait]
impl<P> ITodoRepo<P> for Todo {
    async fn list(pool: &P) -> Result<Vec<Todo>> {
        let rowset = sqlx::query_as(r#"SELECT * FROM todo"#)
            .fetch_all(pool)
            .await?;

        Ok(rowset)
    }
}
