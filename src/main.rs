use app::startup::run;
use sqlx::postgres::PgPool;
use std::env;
use std::net::TcpListener;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
   
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    
    let connection_pool = PgPool::connect(&database_url)
    .await
    .expect("Failed to connect to Postgres.");
   

    let listener = TcpListener::bind("127.0.0.1:7161")?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
