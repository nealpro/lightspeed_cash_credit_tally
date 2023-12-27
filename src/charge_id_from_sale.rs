use reqwest;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct SalePaymentResponse {
    #[serde(rename = "SalePayment")]
    pub sale_payments: Vec<SalePayment>,
}

#[derive(Deserialize, Debug)]
pub struct SalePayment {
    #[serde(rename = "salePaymentID")]
    pub sale_payment_id: String,
    pub amount: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
    // archived: String,
    // remoteReference: String,
    #[serde(rename = "tipAmount")]
    pub tip_amount: String,
    // paymentID: String,
    // saleID: String,
    // paymentTypeID: String,
    #[serde(rename = "ccChargeID")]
    pub cccharge_id: String,
    // refPaymentID: String,
    // registerID: String,
    // employeeID: String,
    // creditAccountID: String,
}

// Function to get SalePayment data
pub async fn get_sale_payment(
    account_id: &str,
    access_token: &str,
) -> Result<SalePaymentResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.lightspeedapp.com/API/V3/Account/{}/SalePayment.json",
        account_id
    );

    let response = client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await?
        .json::<SalePaymentResponse>()
        .await?;

    Ok(response)
}
