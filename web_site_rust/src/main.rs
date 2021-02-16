mod models;
mod config;
mod handlers;
mod db;
mod errors;

use actix_web::{HttpServer, App, web};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::*;


#[actix_rt::main]
async fn main() -> io::Result<()>{
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Starting server at http://{}:{}", config.server.host, config.server.port);

    HttpServer::new(move | |{
        App::new()
        .data(pool.clone())
        .route("/", web::get().to(status))
        .route("/todos{_:/?}", web::get().to(get_todos))
        .route("/todos/{list_id}/items", web::get().to(get_items))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
