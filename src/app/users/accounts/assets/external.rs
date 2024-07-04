use error_stack::{ensure, ResultExt};
use serde::{Deserialize, Serialize};

use crate::error::{ApiClientError, SResult};
use crate::logging::prelude::*;

const API_URL: &str = "https://sm.cord.network/api/v1/finternet";

pub async fn create_token(addr: String, value: usize) -> SResult<(), ApiClientError> {
    #[derive(Serialize)]
    struct Request {
        address: String,
        amount: usize,
    }

    let request = Request {
        address: addr,
        amount: value,
    };

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/contract/tokenize", API_URL))
        .json(&request)
        .send()
        .await
        .change_context(ApiClientError::RequestError)?;

    ensure!(
        response.status().is_success(),
        ApiClientError::NonSuccessStatusError
    );

    let body = response
        .text()
        .await
        .change_context(ApiClientError::DeserializationError)?;

    info!("Response: {}", body);

    Ok(())
}

pub async fn transfer_token(
    from_addr: String,
    to_addr: String,
    value: usize,
) -> SResult<(), ApiClientError> {
    #[derive(Serialize)]
    struct Request {
        #[serde(rename(serialize = "fromAddress"))]
        from_address: String,
        #[serde(rename(serialize = "toAddress"))]
        to_address: String,
        amount: usize,
    }

    let request = Request {
        from_address: from_addr,
        to_address: to_addr,
        amount: value,
    };

    info!("{}", serde_json::to_string(&request).unwrap_or_default());

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/contract/transfer", API_URL))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .change_context(ApiClientError::RequestError)?;

    ensure!(
        response.status().is_success(),
        ApiClientError::NonSuccessStatusError
    );

    let body = response
        .text()
        .await
        .change_context(ApiClientError::DeserializationError)?;

    info!("Response: {}", body);

    Ok(())
}

pub async fn burn_token(addr: String, value: usize) -> SResult<(), ApiClientError> {
    #[derive(Serialize)]
    struct Request {
        address: String,
        amount: usize,
    }

    let request = Request {
        address: addr,
        amount: value,
    };

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/contract/detokenize", API_URL))
        .json(&request)
        .send()
        .await
        .change_context(ApiClientError::RequestError)?;

    ensure!(
        response.status().is_success(),
        ApiClientError::NonSuccessStatusError
    );

    let body = response
        .text()
        .await
        .change_context(ApiClientError::DeserializationError)?;

    info!("Response: {}", body);

    Ok(())
}

pub async fn get_balance(addr: String) -> SResult<usize, ApiClientError> {
    let response = reqwest::get(format!("{}/contract/balance/{}", API_URL, addr))
        .await
        .change_context(ApiClientError::RequestError)?;

    #[derive(Deserialize)]
    struct Res {
        address: String,
        balance: String,
    }

    #[derive(Deserialize)]
    struct Response {
        status: usize,
        result: Res,
    }

    ensure!(
        response.status().is_success(),
        ApiClientError::NonSuccessStatusError
    );

    let response: Response = response
        .json()
        .await
        .change_context(ApiClientError::DeserializationError)?;

    Ok(response
        .result
        .balance
        .replace(",", "")
        .parse::<usize>()
        .change_context(ApiClientError::DeserializationError)?)
}
