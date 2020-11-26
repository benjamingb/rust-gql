use async_trait::async_trait;

use crate::gql_type::TodoType;
use crate::result::Result;
#[async_trait]
pub trait ITodoRepo<'a, P> {
    async fn list(pool:&P) -> Result<Vec<TodoType>>;
   
}
