use crate::poll::{Poll, Question};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use liquid::{object, Template};
use serde_json::json;
use sqlx::{Row, SqlitePool};
use std::fs::read_to_string;
use std::path::Path;

// Webpage to create a poll
#[post("/create_poll")]
async fn create_poll(body: String) -> impl Responder {
    let pool = SqlitePool::connect("identifier.sqlite").await.unwrap();
    let poll: Poll = serde_json::from_str(&body).unwrap();
    println!("{:?}", poll);
    insert_poll(pool.clone(), &poll)
        .await
        .expect("TODO: panic message");
    let poll_id: i32 = get_id_from_database(pool, &poll).await;
    HttpResponse::Ok().json(json!({ "poll_id": poll_id }))
}
#[get("/")]
async fn index() -> HttpResponse {
    let body = read_to_string("src/index.liquid").unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
#[get("/poll/{poll_id}")]
async fn poll_page(poll_id: web::Path<i32>) -> HttpResponse {
    let pool = SqlitePool::connect("identifier.sqlite").await.unwrap();
    let poll: Poll = get_poll_from_database(pool, *poll_id).await;
    println!("{:?}", poll);
    let template = liquid_parse("src/poll.liquid");
    let data = object!({ "poll": poll });
    let body = template.render(&data).unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
async fn get_id_from_database(pool: SqlitePool, poll: &Poll) -> i32 {
    let poll_id: i32 = sqlx::query_scalar("SELECT id FROM poll WHERE title = ?")
        .bind(&poll.title)
        .fetch_one(&pool)
        .await
        .unwrap();
    poll_id
}
async fn get_questions_from_database(pool: SqlitePool, poll_id: i32) -> Vec<Question> {
    let questions_raw = sqlx::query("SELECT * FROM question WHERE poll_id = ?")
        .bind(poll_id)
        .fetch_all(&pool)
        .await
        .unwrap();
    let questions: Vec<Question> = questions_raw
        .iter()
        .map(|row| {
            let text: String = row.get("text");
            let option1: String = row.get("option1");
            let option2: String = row.get("option2");
            let option3: String = row.get("option3");
            Question::new(text, vec![option1, option2, option3])
        })
        .collect();
    questions
}

async fn get_poll_from_database(pool: SqlitePool, poll_id: i32) -> Poll {
    let poll_row_raw = sqlx::query("SELECT * FROM poll WHERE id = ?")
        .bind(&poll_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let _id: i32 = poll_row_raw.get("id");
    let title: String = poll_row_raw.get("title");

    let questions: Vec<Question> = get_questions_from_database(pool.clone(), poll_id).await;

    Poll { title, questions }
}

async fn insert_poll(pool: SqlitePool, poll: &Poll) -> Result<(), std::io::Error> {
    // Insert the poll into the `poll` table
    let poll_id: i32 = sqlx::query_scalar("INSERT INTO poll (title) VALUES (?) RETURNING id")
        .bind(&poll.title)
        .fetch_one(&pool)
        .await
        .unwrap();

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
    .await
    .unwrap();

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
    .await
    .unwrap();
    Ok(())
}

#[actix_web::main]
pub(crate) async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(create_poll)
            .service(poll_page)
    })
    .bind("127.60.20.1:7373")?
    .run()
    .await
}
pub(crate) fn liquid_parse(path: impl AsRef<Path>) -> Template {
    let compiler = liquid::ParserBuilder::with_stdlib()
        .build()
        .expect("Could not build liquid compiler");
    compiler
        .parse(read_to_string(path).unwrap().as_str())
        .unwrap()
}
