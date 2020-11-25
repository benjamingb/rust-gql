use async_trait::async_trait;
#[async_trait]
pub trait ITodoRepo<P> {
    async fn list(pool: P);
}
