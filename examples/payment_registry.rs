use chrono::{NaiveDate, Utc};
use tochka_sdk::{Client, PaymentRegistryQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = Client::new().await?;

    let customer_code = std::env::var("CUSTOMER_CODE")?;
    let merchant_id = std::env::var("MERCHANT_ID")?;
    let payment_id = std::env::var("PAYMENT_ID")?;

    let registry_date = std::env::var("REGISTRY_DATE")
        .ok()
        .and_then(|v| NaiveDate::parse_from_str(&v, "%Y-%m-%d").ok())
        .unwrap_or_else(|| Utc::now().date_naive());

    let registry = client
        .get_payment_registry(PaymentRegistryQuery::new(
            &customer_code,
            &merchant_id,
            &payment_id,
            registry_date,
        ))
        .await?;
    println!(
        "Payment registry {registry_date}:\n{:#?}",
        registry.data.registry
    );

    let retailers = client.get_retailers(&customer_code).await?;
    println!(
        "Retailers for {customer_code}:\n{:#?}",
        retailers.data.retailer
    );

    Ok(())
}
