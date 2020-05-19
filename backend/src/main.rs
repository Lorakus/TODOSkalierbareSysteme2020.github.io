mod models;

use crate::models::Status;
use actix_web::{Responder, HttpServer, App, web};
use std::io;


async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status {status: "OK".to_string()})
}


#[actix_rt::main]
async fn main() -> io::Result<()> {

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(||{
        App::new()
            .route("/", web::get().to(status))
            
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}