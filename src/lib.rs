pub mod app;
pub mod config;
pub mod error;
pub mod imc;
pub mod logging;
pub mod state;
pub mod storage;

pub async fn solana_connect(
    data: &impl serde::Serialize,
    route: &str,
) -> Result<Option<String>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://finternet-solana-apis-production.up.railway.app{}",
        route
    );

    let res = client.post(&url).json(&data).send().await;

    match res {
        Ok(ires) => {
            let post_res = ires.json::<PostResponse>().await;
            match post_res {
                Ok(ires) => {
                    println!("Signature from response: {:?}", ires.signature);
                    Ok(Some(ires.signature))
                }
                Err(e) => {
                    println!("Failed to parse response: {:?}", e);
                    Ok(None)
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to Solana: {:?}", e);
            Err(e)
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PostResponse {
    // data: serde_json::Value,
    pub signature: String,
}
