pub struct TodoType{
    pub id: i32,
    pub body: String,
    pub complete: String,
}
#[async_graphql::Object]
impl TodoType {
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