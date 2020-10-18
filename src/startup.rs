use crate::graphql_schema::{MutationRoot, QueryRoot};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use sqlx::PgPool;
use std::net::TcpListener;


pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {


    //GraphQL
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(db_pool.clone())
        .finish();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .data(schema.clone())
            .route("/graphql", web::post().to(graphql))
            .route("/graphql", web::get().to(graphql_playground))
    })
    .listen(listener)?
    .run();
    Ok(server)
}


// TODO: PlayGround
async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql"))))
}

pub type GQLSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn graphql(schema: web::Data<GQLSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}
