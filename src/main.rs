use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use schooldash::{
    get_lesson_info, parse_lessons,
    schools::{Day, School},
};

// NTI: MzMzODU1NjAtZGYyZS1mM2U2LTgzY2MtNDA0NGFjMmZjZjUw
// 2A: Y2MwYzVmYjktZjlkNy1mOWIzLThlN2MtMDNmNzIyNjVkNzJl
#[get("/show_school/{school_id}")]
async fn list_schools(school_id: String) -> impl Responder {
    let school = School::new().await; // School id måste vara satt i structen.

    HttpResponse::Ok().body("Hello world!")
}

#[get("/schema/{school_id}/{klass_id}")]
async fn echo(school_id: String, klass_id: String) -> impl Responder {
    let school = School::new().await;

    let result = match school.get_day_schema(klass_id, Day::Måndag).await {
        Some(lessons) => lessons,
        None => {
            println!("No lessons found");
            vec![]
        }
    };
    // let lesson_info = get_lesson_info(Client::new(), key).await;

    // let lesson_info_as_string = parse_lessons(lesson_info);

    // let lesson_info_as_string = serde_json::to_string(&lesson_info_as_string);

    HttpResponse::Ok().body(serde_json::to_string(&result).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(list_schools).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
