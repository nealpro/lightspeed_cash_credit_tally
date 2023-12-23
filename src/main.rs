mod credit_card_charge;
mod payments;
mod tokens;

use chrono::{Duration, Utc};
use dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let today = (Utc::now().date_naive() - Duration::days(1))
        .format("%m/%d/%Y")
        .to_string();
    let client_id = dotenv::var("CLIENT_ID")?;
    let client_secret = dotenv::var("CLIENT_SECRET")?;
    let refresh_token = dotenv::var("REFRESH_TOKEN")?;
    let account_id = dotenv::var("ACCOUNT_ID")?;

    let mut total: f64 = 0.0;
    let mut credit_total: f64 = 0.0;

    // Await the result of get_access_token
    let access_token = tokens::get_access_token(&client_id, &client_secret, &refresh_token).await?;

    for payment in payments::get_payments_by_day(&account_id, &access_token)
        .await?
        .payments
    {
        if payment.date != today {
            continue;
        }
        println!(
            "Amount: {}, Type: {}, Date: {}",
            payment.amount, payment.payment_type_name, payment.date
        );

        total += payment.amount.parse::<f64>().unwrap();
    }

    println!("Total: {}", total);
    println!("It's {} today!", today);

    for charge in credit_card_charge::get_cc_charge(&account_id, &access_token)
        .await?
        .charges
    {
        // if charge.amount == "0" {
        //     continue;
        // }
        // if charge.is_debit == "true" {
        //     continue;
        // }
        println!(
            "Amount: {}, Debit? : {}, Timestamp: {}",
            charge.amount, charge.is_debit, charge.time_stamp
        );

        credit_total += charge.amount.parse::<f64>().unwrap();
    }

    println!("Credit Total: {}", credit_total);

    Ok(())
}

// ?timeStamp=>=,2021-10-05T14:18:46+00:00&sort=-timeStamp
// https://api.lightspeedapp.com/API/V3/Account/{{accountID}}/CCCharge.json?timeStamp=>,2023-12-01T00:00:00-0500
//
