use crate::todo::*;
use async_graphql::{Context, FieldResult};
use sqlx::postgres::PgPool;


#[async_graphql::Object]
impl Todo {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn body(&self) -> &str {
        &self.body
    }

    async fn complete(&self) -> &str {
        &self.complete
    }
}

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn todos(&self, ctx: &Context<'_>) -> FieldResult<Vec<Todo>> {
        let pool = ctx.data::<PgPool>()?;
        let items = Todo::list(pool).await?;
        Ok(items)
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {}