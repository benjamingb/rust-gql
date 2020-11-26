use crate::gql_type::TodoType;
use crate::interface_todo::ITodoRepo;
use crate::todo::*;
use async_graphql::*;
use sqlx::postgres::PgPool;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn todos(&self, ctx: &Context<'_>) -> FieldResult<Vec<TodoType>> {
        let pool = ctx.data::<PgPool>()?;
        let items = TodoRepo::list(&pool).await?;
        Ok(items)
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {}
