
use reqwest::Client;
use schooldash::{get_key, get_lesson_info};



#[tokio::main]
async fn main() {
    let key = get_key().await;

    let lesson_info = get_lesson_info(Client::new(), key).await;

    println!("{:?}", lesson_info);
//  = match body_parsed.data.lessonInfo {
//         Some(lesson_info) => lesson_info,
//         None => {
//             panic!("{:?} {body} {}", body_parsed.data, serde_json::to_string(&body_to_send).unwrap());
//         }
//     };
    
//     println!("{:?}", lesson_info)
}
