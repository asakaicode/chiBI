use std::io::Result;

use actix_cors::Cors;
use actix_web::{guard, web, App, HttpServer};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use log::info;

mod utils;
use utils::graphql_schema::{create_schema, AppSchema};

async fn graphql_handler(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> Result<()> {
    let schema = create_schema();

    println!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec!["content-type"])
            )
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql_handler))
            .service(web::resource("/graphql").guard(guard::Get()).to(graphql_handler))
    }).bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    info!("Server stopped");
    Ok(())
}
