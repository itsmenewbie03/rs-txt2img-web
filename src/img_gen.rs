use std::collections::HashMap;

use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ImageResponse {
    execution_time: f64,
    image: String,
    text: String,
}

pub async fn get_image(prompt: String) -> String {
    let cli = reqwest::Client::new();
    let endpoint = "https://weopi.com/bot";
    let req_map = HashMap::from([("searchText", format!("#image {}", prompt))]);
    // NOTE: this a toy project to I'm just gonna unwrap it xD
    let resp = cli
        .post(endpoint)
        .json(&req_map)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    let text = match resp {
        Ok(txt) => txt,
        Err(_) => "".to_string(),
    };
    let image_res: ImageResponse = serde_json::from_str(&text).unwrap();
    let out_text = image_res.text;
    // TODO: parse the resp
    out_text.split("src=\"").collect::<Vec<&str>>()[1]
        .split("\"")
        .collect::<Vec<&str>>()[0]
        .to_string()
}
