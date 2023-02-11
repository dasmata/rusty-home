use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CatFact {
    fact: String,
    length: i32
}

#[tokio::main]
async fn main() {
    let result_response = get_cat_fact().await;

    let fact: Response = match result_response {
        Ok(r) => r,
        Err(e) => panic!("Could not GET the cat fact! {:?}", e)
    };
    let json_response: reqwest::Result<CatFact> = fact.json().await;
    let cat_fact = match json_response {
        Ok(r) => r,
        Err(e) => panic!("Could not PARSE the cat fact! {:?}", e)
    };

    println!("fact: {}", cat_fact.fact);
    println!("length: {}", cat_fact.length);
}

async fn get_cat_fact() -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get("https://catfact.ninja/fact").send()
        .await?;

    Ok(body)
}
