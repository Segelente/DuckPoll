use std::fs::read_to_string;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, post, get, body};
use actix_web::web::Redirect;
use serde::{Deserialize};
use serde_json::{json, Value};
use crate::poll::Poll;
// Webpage to create a poll

#[post("/create_poll")]
async fn create_poll(body: String) -> impl Responder {
    let poll: Poll = serde_json::from_str(&body).unwrap();
    println!("{:?}", poll);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("Poll created")
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
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}