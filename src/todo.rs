use crate::gql_type::TodoType;
use crate::interface_todo::ITodoRepo;
use crate::result::Result;
use async_trait::async_trait;
use sqlx::*;

#[derive(FromRow, Debug)]
pub struct TodoSchema {
    pub id: i32,
    pub body: Option<String>,
    pub complete: Option<String>,
}

pub struct TodoRepo;
#[async_trait]
impl<P> ITodoRepo<P> for TodoRepo {
    async fn list(pool: &P) -> Result<Vec<TodoType>> {
        let rowset = sqlx::query_as!(TodoSchema, r#"SELECT * FROM todo"#)
            .fetch_all(pool)
            .await?
            .iter()
            .map(|row| hydrate(row))
            .collect();

        Ok(rowset)
    }
}

fn hydrate(row: &TodoSchema) -> TodoType {
    TodoType {
        id: row.id,
        body: row.body.to_owned().unwrap_or("".to_owned()),
        complete: row.complete.to_owned().unwrap_or("".to_owned()),
    }
}
