use crate::{errors::AppError, models::{Status}};
use crate::db;
use deadpool_postgres::{Pool, Client};
use actix_web::{HttpResponse, Responder, web};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status{status:"Ok".to_string()})
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {
    let client: Client = 
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::get_todos(&client).await;

    result.map(|todos| HttpResponse::Ok().json(todos))
}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = 
        db_pool.get().await.expect("Error connecting to the database");

    let result = db::get_items(&client, path.0).await;

    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}


pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodoList>) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.expect("Error connecting to the database");

    let result = db::create_todo(&Client, json.title.clone().await);

    match result{
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}
