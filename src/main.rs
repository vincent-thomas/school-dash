use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use schooldash::schools::School;
use schooldash::utils::serialize_day;

// NTI: MzMzODU1NjAtZGYyZS1mM2U2LTgzY2MtNDA0NGFjMmZjZjUw
// 2A: Y2MwYzVmYjktZjlkNy1mOWIzLThlN2MtMDNmNzIyNjVkNzJl
#[get("/show_school/{school_id}")]
async fn list_schools(_school_id: String) -> impl Responder {
    // TODO
    let school = School::default(); // School id måste vara satt i structen.

    HttpResponse::Ok().body("Hello world!")
}

#[get("/schema/{school_id}/{klass_id}/{day}")]
async fn echo(path: web::Path<(String, String, i8)>) -> impl Responder {
    let (school_id, klass_id, day) = path.into_inner();
    println!("School id: {}", school_id);
    println!("Class id: {}", klass_id);
    let school = School::default()
        .select_school(school_id.as_str())
        .select_class_from_id(klass_id.as_str());

    let result = match school.get_schema(/* TODO: serialize_day(day) */).await {
        Some(lessons) => lessons,
        None => {
            println!("No lessons found");
            vec![]
        }
    };

    HttpResponse::Ok().body(serde_json::to_string(&result).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut school = School::default();

    let shit = school
        .select_school("MzMzODU1NjAtZGYyZS1mM2U2LTgzY2MtNDA0NGFjMmZjZjUw")
        .select_class_from_name("2A")
        .await;

    println!("{shit:?}");

    HttpServer::new(|| App::new().service(list_schools).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
