use std::fs::read_to_string;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, post, get, body};
use actix_web::web::Redirect;
use serde::{Deserialize};
use serde_json::{json, Value};
use crate::poll::{Poll, Question};
// Webpage to create a poll
#[post("/create_poll")]
async fn create_poll(body: String) -> impl Responder {
    println!("{:?}", body);
    let poll: Poll = serde_json::from_str(&body).unwrap();
    println!("{:?}", poll);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Poll created")
}

#[get("/")]
async fn index() -> HttpResponse {
    let body = read_to_string("src/index.html").unwrap();
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
    })
        .bind("127.60.20.1:7373")?
        .run()
        .await
}