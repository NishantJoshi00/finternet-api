pub mod app;
pub mod config;
pub mod error;
pub mod imc;
pub mod logging;
pub mod state;
pub mod storage;

pub async fn solana_connect(data: &impl serde::Serialize) {
    let client = reqwest::Client::new();
    let res = client
        .post("https://finternet-solana.up.railway.app")
        .json(&data)
        .send()
        .await;

    match res {
        Ok(ires) => {
            let post_res = ires.json::<PostResponse>().await;
            match post_res {
                Ok(ires) => println!("Received signature: {:?}", ires.signature),
                Err(e) => println!("Failed to parse response: {:?}", e),
            }
        }
        Err(e) => println!("Failed to connect to Solana: {:?}", e),
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PostResponse {
    data: serde_json::Value,
    pub signature: String,
}
