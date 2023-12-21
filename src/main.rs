mod payments;
mod tokens;

use dotenv;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_id = dotenv::var("CLIENT_ID")?;
    let client_secret = dotenv::var("CLIENT_SECRET")?;
    let refresh_token = dotenv::var("REFRESH_TOKEN")?;
    let account_id = dotenv::var("ACCOUNT_ID")?;

    let mut total: f64 = 0.0;

    // Await the result of get_access_token
    let access_token = tokens::get_access_token(&client_id, &client_secret, &refresh_token).await?;

    for payment in payments::get_payments_by_day(account_id, access_token)
        .await?
        .payments
    {
        println!(
            "Amount: {}, Type: {}",
            payment.amount, payment.payment_type_name
        );

        total += payment.amount.parse::<f64>().unwrap();
    }

    println!("Total: {}", total);

    Ok(())
}
