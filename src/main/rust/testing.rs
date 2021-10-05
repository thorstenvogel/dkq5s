//use reqwest::Url;
//use serde_json::json;
use std::collections::HashMap;
//use exitfailure::ExitFailure;

const PID: &'static str = &"DK5QPID";
// // const PORT: &'static i32 = &27301;
const URL: &'static str = &"http://localhost:27301/api/1.0/signals";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    // pipe left: 0,0
    // pipe right: 23,5
    // keys range x: 1 - 22
    // keys range y: 0 - 5

    const key_x:i32 = 3;
    const key_y:i32 = 0;
    const color:&str = "#f00";

    let location_str = format!("{},{}", key_x, key_y);
    let mut body_fields = HashMap::new();
    body_fields.insert("pid", PID.to_string());
    body_fields.insert("zoneId", location_str);
    body_fields.insert("color", color.to_string());
    body_fields.insert("effect", "SET_COLOR".to_string());

    let keyboard_request_response: serde_json::Value = reqwest::Client::new()
        .post(URL)
        .json(&body_fields)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", keyboard_request_response);
    Ok(())
}

// fn main() {
//     set_key_color(2, 4, "#ff0000");
// }

// async fn set_key_color(key_x: i32, key_y: i32, color: &str) {

//     let location_str = format!("{},{}", key_x, key_y);

//     let mut body_fields = HashMap::new();
//     body_fields.insert("pid", PID.to_string());
//     body_fields.insert("zoneId", location_str);
//     body_fields.insert("color", color.to_string());
//     body_fields.insert("effect", "SET_COLOR".to_string());

//     // println!("{}", body_fields);
//     println!("{}", URL);

//     let client = reqwest::Client::new();
//     return client.post(URL)
//         .json(&body_fields)
//         .send().await?;

//     //println!("{:?}", res);
// }

// impl CompanyQuote {
//     async fn get(symbol: &String, api_key: &String) -> Result<Self, ExitFailure> {
//         let url = format!(
//             "https://finnhub.io/api/v1/quote?symbol={}&token={}",
//             symbol, api_key
//         );

//         let url = Url::parse(&*url)?;
//         let res = reqwest::get(url).await?.json::<CompanyQuote>().await?;

//         Ok(res)
//     }
// }
