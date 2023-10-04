
use reqwest::Client;
use schooldash::{get_key, get_lesson_info};

use std::fs::File;
use std::io::prelude::*;



#[tokio::main]
async fn main() -> std::io::Result<()> {
    let key = get_key().await;

    let lesson_info = get_lesson_info(Client::new(), key).await;

    let lesson_info_as_string = lesson_info.to_string();

    let mut file = File::create("lesson_info.json")?;

    file.write_all(lesson_info_as_string.as_bytes())?;
    
    Ok(())
}
