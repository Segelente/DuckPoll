use std::fs::read_to_string;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, post, get, body};
use actix_web::web::Redirect;
use serde::{Deserialize};
use serde_json::{json, Value};
use crate::poll::{Poll, Question};
// Webpage to create a poll

#[post("/create_poll")]
async fn create_poll(body: String) -> impl Responder {
    let poll: Poll = serde_json::from_str(&body).unwrap();
    println!("{:?}", poll);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Poll created")
}
#[post("/poll_question")]
async fn question(body: String) -> impl Responder {
    let question: Question = serde_json::from_str(&body).unwrap();
    println!("{:?}", question);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Poll created")
}
#[post("/add_or_delete_question")]
async fn add_question(body: String) -> impl Responder {
    // add question to poll or delete question from poll


}

#[get("/")]
async fn button() -> HttpResponse {
    let body = read_to_string("src/index.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}
#[get("/poll_form")]
async fn poll_form() -> HttpResponse {
    let body = read_to_string("src/poll_template.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

#[actix_web::main]
pub(crate) async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(button)
            .service(create_poll)
            .service(poll_form)
            .service(question)
            .service(add_question)
    })
        .bind("127.60.20.1:7373")?
        .run()
        .await
}