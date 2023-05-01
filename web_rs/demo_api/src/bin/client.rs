use std::collections::HashMap;

use demo_api::models::counties::County;
use reqwest::header::USER_AGENT;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://localhost:3000/all")
        .await?
        .json::<Vec<County>>()
        .await?;
    println!("{:#?}", resp);

    let resp = reqwest::get("http://localhost:3000/county/LDN")
        .await?
        .json::<County>()
        .await?;
    println!("{:#?}", resp);

    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:3000/county/LDY")
        .header(USER_AGENT, "demo-api-client")
        .send()
        .await?;
    let county: County = response.json().await?;
    println!("{:#?}", county);

    Ok(())
}
