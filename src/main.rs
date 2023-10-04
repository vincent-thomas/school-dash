use reqwest::Client;
use schooldash::{get_key, get_lesson_info, parse_lessons};

use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let key = get_key().await;

    let lesson_info = get_lesson_info(Client::new(), key).await;

    let lesson_info_as_string = parse_lessons(lesson_info);
    let mut file = File::create("lesson_info.json")?;

    let lesson_info_as_string = serde_json::to_string(&lesson_info_as_string);

    file.write_all(lesson_info_as_string.unwrap().as_bytes())?;

    Ok(())
}
