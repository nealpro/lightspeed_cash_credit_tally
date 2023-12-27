use chrono::{Duration, Utc};
use reqwest;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct PaymentsResponse {
    #[serde(rename = "@attributes")]
    pub attributes: Option<Attributes>,
    #[serde(rename = "Payments")]
    pub payments: Vec<Payment>,
}

#[derive(Deserialize, Debug)]
pub struct Attributes {
    pub next: Option<String>,
    pub previous: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Payment {
    pub date: String,
    #[serde(rename = "shopID")]
    pub shop_id: String,
    pub amount: String,
    #[serde(rename = "paymentTypeName")]
    pub payment_type_name: String,
    #[serde(rename = "paymentTypeID")]
    pub payment_type_id: String,
}

pub async fn get_payments_by_day(
    account_id: &String,
    access_token: &String,
) -> Result<PaymentsResponse, Box<dyn Error>> {
    let today = Utc::now().date_naive();
    let yesterday = Utc::now().date_naive() - Duration::days(1);

    let start_date = yesterday.format("%Y-%m-%d-0500").to_string();
    let end_date = today.format("%Y-%m-%d-0500").to_string();

    let url = format!(
        "https://api.lightspeedapp.com/API/V3/Account/{}/Reports/Accounting/PaymentsByDay.json?startDate={}&endDate={}",
        account_id, start_date, end_date
    );

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await?
        .json::<PaymentsResponse>()
        .await?;

    Ok(response)
}
