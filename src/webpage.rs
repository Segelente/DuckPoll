use crate::database_functions::{
    get_id_from_database, get_poll_from_database, get_votes_from_db, insert_poll, save_votes_to_db,
};
use crate::poll::{Poll, Vote};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use liquid::{object, Template};
use serde_json::{json, Value};
use sqlx::SqlitePool;
use std::fs::read_to_string;
use std::path::Path;

pub(crate) fn liquid_parse(path: impl AsRef<Path>) -> Template {
    let compiler = liquid::ParserBuilder::with_stdlib()
        .build()
        .expect("Could not build liquid compiler");
    compiler
        .parse(read_to_string(path).unwrap().as_str())
        .unwrap()
}

// Webpage to create a poll
#[post("/create_poll")]
async fn create_poll(body: String) -> impl Responder {
    let pool = SqlitePool::connect("identifier.sqlite").await.unwrap();
    let poll: Poll = serde_json::from_str(&body).unwrap();
    insert_poll(pool.clone(), &poll)
        .await
        .expect("TODO: panic message");
    let poll_id: i32 = get_id_from_database(pool, &poll).await;
    HttpResponse::Ok().json(json!({ "poll_id": poll_id }))
}
// Webpage to save votes for a poll
#[post("/save_votes/{poll_id}")]
async fn save_votes(body: String, poll_id: web::Path<i32>) -> impl Responder {
    let pool = SqlitePool::connect("identifier.sqlite").await.unwrap();
    let votes: Vec<Vote> = serde_json::from_str(&body).unwrap();
    for vote in votes {
        println!("{:?}", vote);
        save_votes_to_db(pool.clone(), *poll_id, vote).await;
    }
    HttpResponse::Ok()
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
    let template = liquid_parse("src/poll.liquid");
    let id = poll_id.into_inner();
    let data = object!({ "poll": poll, "poll_id": id });
    let body = template.render(&data).unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
#[get("/submitted_successfully")]
async fn submitted_successfully() -> HttpResponse {
    let body = read_to_string("src/submit.liquid").unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
#[get("/poll_results/{poll_id}")]
async fn poll_results(poll_id: web::Path<i32>) -> HttpResponse {
    let pool = SqlitePool::connect("identifier.sqlite").await.unwrap();
    let votes: Vec<Vote> = get_votes_from_db(pool, *poll_id).await;
    let template = liquid_parse("src/results.liquid");
    let data = object!({ "votes": votes });
    let body = template.render(&data).unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

#[actix_web::main]
pub(crate) async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(create_poll)
            .service(poll_page)
            .service(save_votes)
            .service(poll_results)
            .service(submitted_successfully)
    })
    .bind("127.60.20.1:7373")?
    .run()
    .await
}
