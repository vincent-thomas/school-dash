use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use schooldash::{get_key, get_lesson_info, parse_lessons};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/schema")]
async fn echo() -> impl Responder {
    let key = get_key().await;
    let lesson_info = get_lesson_info(Client::new(), key).await;

    let lesson_info_as_string = parse_lessons(lesson_info);

    let lesson_info_as_string = serde_json::to_string(&lesson_info_as_string);

    HttpResponse::Ok().body(lesson_info_as_string.unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
