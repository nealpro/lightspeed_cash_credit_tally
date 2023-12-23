use chrono::{Duration, Utc};
use reqwest;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct CCChargeResponse {
    #[serde(rename = "CCCharge")]
    pub charges: Vec<CCCharge>,
}

#[derive(Deserialize, Debug)]
pub struct CCCharge {
    pub amount: String,
    #[serde(rename = "isDebit")]
    pub is_debit: String,
    pub declined: String,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
}

pub async fn get_cc_charge(
    account_id: &String,
    access_token: &String,
) -> Result<CCChargeResponse, Box<dyn Error>> {
    let yesterday = (Utc::now().date_naive() - Duration::days(1))
        .format("%Y-%m-%d")
        .to_string();

    let client = reqwest::Client::new();
    let _url_unused = format!(
        "https://api.lightspeedapp.com/API/V3/Account/{}/CCCharge.json?timeStamp=>,{}T00:00:00-0500",
        account_id, yesterday
    );

    let url = format!(
        "https://api.lightspeedapp.com/API/V3/Account/{}/CCCharge.json?timeStamp=>,2023-07-01T00:00:00",
        account_id
    );

    // let response_text = client
    //     .get(&url)
    //     .bearer_auth(access_token)
    //     .send()
    //     .await?
    //     .text()
    //     .await?;

    // println!("Response text: {}", response_text);

    let response = client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await?
        .json::<CCChargeResponse>()
        .await?;

    Ok(response)
}
