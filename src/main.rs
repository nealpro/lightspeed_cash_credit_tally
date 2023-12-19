mod secrets;

use reqwest;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct ApiResponse {
    cccharge: Vec<Charge>,
}

#[derive(Deserialize, Debug)]
struct Charge {
    amount: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let account_id = "your_account_id";
    let access_token = "your_access_token"; // Replace with your access token

    let client = reqwest::Client::new();
    let res = client
        .get(format!(
            "https://api.lightspeedapp.com/API/V3/Account/{}/CCCharge.json",
            account_id
        ))
        .bearer_auth(access_token)
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;

    for charge in res.cccharge {
        println!("Amount: {}", charge.amount);
    }

    Ok(())
}
