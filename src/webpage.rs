use actix_web::{web, App, HttpResponse, HttpServer, Responder, post, get};
use serde::Deserialize;
use crate::poll::Poll;
// Webpage including the poll

#[get("/{id}/{title}")]
async fn create_poll(poll: web::Path<Poll>) -> impl Responder {
    let poll = poll.into_inner();
    HttpResponse::Ok().body(format!("This is your Poll: {}", poll.title))
}
#[actix_web::main]
pub(crate) async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_poll)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
