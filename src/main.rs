pub mod collectors;
pub mod context;
pub mod database;
pub mod graphql;
pub mod loaders;
pub mod models;
pub mod mutations;
pub mod mutators;
pub mod queries;
pub mod schema;

use actix_web::middleware::Logger;
use actix_web::{get, route, web, App, Error, HttpResponse, HttpServer, Responder};
use actix_web_lab::respond::Html;
use context::Context;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use loaders::get_user_loader;
use std::sync::Arc;

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql_endpoint(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    schema: web::Data<Arc<graphql::Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        pool: pool.get_ref().to_owned(),
        user_loader: get_user_loader(pool.get_ref().to_owned()),
    };
    let res = data.execute(&schema, &ctx).await;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

pub fn register(config: &mut web::ServiceConfig) {
    config
        .app_data(web::Data::new(database::get_pool().clone()))
        .app_data(web::Data::new(Arc::new(graphql::create_schema())))
        .service(graphql_endpoint)
        .service(graphql_playground);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().configure(register).wrap(Logger::default()))
        .workers(2)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
