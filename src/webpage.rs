use std::fs::read_to_string;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, post, get, body};
use actix_web::web::{Query, Redirect};
use serde::{Deserialize};
use serde_json::{json, Value};
use sqlx::SqlitePool;
use crate::poll::{Poll, Question};
// Webpage to create a poll
#[post("/create_poll")]
async fn create_poll(body: String) -> impl Responder {
    let pool = SqlitePool::connect("identifier.sqlite").await.unwrap();
    let poll: Poll = serde_json::from_str(&body).unwrap();
    println!("{:?}", poll);
    insert_poll(pool.clone(), &poll).await.expect("TODO: panic message");
    let poll_id: i32 = get_id_from_database(pool, &poll).await;
    HttpResponse::Ok().json(json!({
        "poll_id": poll_id
    }))
}
#[get("/")]
async fn index() -> HttpResponse {
    let body = read_to_string("src/index.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
#[get("/poll/{poll_id}")]
async fn polli(poll_id: i32) -> HttpResponse {
    let pool = SqlitePool::connect("identifier.sqlite").await.unwrap();
    let poll: String = get_poll_from_database(pool, poll_id).await;
    println!("{:?}", poll);
    let body = read_to_string("src/poll.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
async fn get_id_from_database(pool: SqlitePool, poll: &Poll) -> i32 {
    let poll_id: i32 = sqlx::query_scalar("SELECT id FROM poll WHERE title = ?")
        .bind(&poll.title)
        .fetch_one(&pool)
        .await.unwrap();
    poll_id
}
async fn get_poll_from_database(pool: SqlitePool, poll_id: i32) -> Poll {
    let poll: Poll = sqlx::query_scalar("SELECT * FROM poll WHERE id = ?")
        .bind(&poll_id)
        .fetch_one(&pool)
        .await.unwrap()x;
    poll
}
async fn insert_poll(pool: SqlitePool, poll: &Poll) -> Result<(), std::io::Error> {
    // Insert the poll into the `poll` table
    let poll_id: i32 = sqlx::query_scalar("INSERT INTO poll (title) VALUES (?) RETURNING id")
        .bind(&poll.title)
        .fetch_one(&pool)
        .await.unwrap();

    // Insert each question and its options into the `question_option` table
    for question in &poll.questions {
        let question_id: i32 = sqlx::query_scalar("INSERT INTO question (poll_id, text, option1, option2, option3) VALUES (?, ?, ?, ?, ?) RETURNING id")
            .bind(&poll_id)
            .bind(&question.text)
            .bind(&question.options[0])
            .bind(&question.options[1])
            .bind(&question.options[2])
            .fetch_one(&pool)
            .await.unwrap();
    }

    Ok(())
}

#[allow(dead_code)]
async fn create_tables(pool: &SqlitePool) -> Result<(), std::io::Error> {
    // Create the `poll` table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS poll (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL
        )
        "#,
    )
        .execute(pool)
        .await.unwrap();

    // Create the `question` table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS question (
            id INTEGER PRIMARY KEY,
            poll_id INTEGER NOT NULL,
            text TEXT NOT NULL,
            FOREIGN KEY (poll_id) REFERENCES poll (id) ON DELETE CASCADE
        )
        "#,
    )
        .execute(pool)
        .await.unwrap();
    Ok(())
}

#[actix_web::main]
pub(crate) async fn main() -> std::io::Result<()> {
   HttpServer::new(|| {
       App::new()
           .service(index)
           .service(create_poll)
           .service(polli)
   })
       .bind("127.60.20.1:7373")?
       .run()
       .await
}