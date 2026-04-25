use tochka_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = Client::new().await?;
    let customer_code = std::env::var("CUSTOMER_CODE")?;

    let list = client.get_customers_list().await?;
    println!("All customers:\n{:#?}", list.data.customer);

    let customer = client.get_customer_info(&customer_code).await?;
    println!("Selected customer {customer_code}:\n{:#?}", customer.data);

    Ok(())
}
