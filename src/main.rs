use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use schooldash::schools::{Day, School};
use schooldash::utils::serialize_day;

// NTI: MzMzODU1NjAtZGYyZS1mM2U2LTgzY2MtNDA0NGFjMmZjZjUw
// 2A: Y2MwYzVmYjktZjlkNy1mOWIzLThlN2MtMDNmNzIyNjVkNzJl
#[get("/show_school/{school_id}")]
async fn list_schools(_school_id: String) -> impl Responder {
    // TODO
    let school = School::new().await; // School id måste vara satt i structen.

    HttpResponse::Ok().body("Hello world!")
}

#[get("/schema/{school_id}/{klass_id}/{day}")]
async fn echo(path: web::Path<(String, String, i8)>) -> impl Responder {
    let (school_id, klass_id, day) = path.into_inner();
    println!("School id: {}", school_id);
    println!("Class id: {}", klass_id);
    let school = School::new()
        .await
        .select_school(school_id)
        .select_class_from_id(klass_id);

    let result = match school.get_day_schema(serialize_day(day)).await {
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
    let school = School::new().await;

    school
        .select_school("MzMzODU1NjAtZGYyZS1mM2U2LTgzY2MtNDA0NGFjMmZjZjUw".to_string())
        .select_class_from_id("Y2MwYzVmYjktZjlkNy1mOWIzLThlN2MtMDNmNzIyNjVkNzJl".to_string())
        .get_day_schema(Day::Måndag)
        .await;

    HttpServer::new(|| App::new().service(list_schools).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
