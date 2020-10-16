use anyhow::Context;
use app::startup::run;
use sqlx::postgres::PgPool;
use std::env;
use std::net::TcpListener;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
   
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let db_pool = PgPool::builder().max_size(5).build(database_url).await
    .map_err(anyhow::Error::from)
    .with_context(|| "Failed to connect to Postgres.")?;

    /*let db_pool = PgPool::new(&database_url)
    .await*/
   

    let listener = TcpListener::bind("127.0.0.1:5161")?;
    run(listener, db_pool)?.await?;
    Ok(())
}
