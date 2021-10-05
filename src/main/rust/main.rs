//use reqwest::Url;
//use serde_json::json;
use std::collections::HashMap;
//use exitfailure::ExitFailure;
mod keyboard;
mod prepare_request_body;

use crate::keyboard::keyboard::Key;
use crate::keyboard::keyboard::init as init_keyboard_grid;
use crate::prepare_request_body::prepare_request_body;

// // const PORT: &'static i32 = &27301;
const URL: &'static str = &"http://localhost:27301/api/1.0/signals";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let keyboard:Vec<Key> = init_keyboard_grid();
    for key in keyboard.iter() {
        let body = prepare_request_body(key.x, key.y, "#FF0000");
        send_to_keyboard(body).await;
    }
    Ok(())
}

async fn send_to_keyboard(body:HashMap<&str, String>) -> Result<(), reqwest::Error> {
    let _keyboard_request_response: serde_json::Value = reqwest::Client::new()
    .post(URL)
    .json(&body)
    .send()
    .await?
    .json()
    .await?;

    Ok(())
}
