use crate::errors::{AppError, AppErrorType};
use crate::models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, AppError>{
    let statement = client.prepare("select * from todo_list order by id desc")
                                .await
                                .map_err(|err| AppError{
                                    message: None,
                                    cause: Some(err.to_string()),
                                    error_type: AppErrorType::DbError
                                })?;

    let todos = client.query(&statement, &[])
        .await
        .expect("Error getting todo lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todos)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, io::Error> {

    let statement = client.prepare("select * from todo_item where list_id = $1 order by id").await.unwrap();

    let items = client.query(&statement, &[&list_id])
    .await
    .expect("Error getting todo lists")
    .iter()
    .map(|row| TodoItem::from_row_ref(row).unwrap())
    .collect::<Vec<TodoItem>>();

    Ok(items)
}

pub async fn create_todo(client: &Client, todo: CreateTodoList) -> None {

}