use tochka_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client = Client::new().await?;

    // Fetch every available account.
    let accounts = client.get_accounts_list().await?;
    println!("Accounts:\n{:#?}", accounts.data.account);

    // Use ACCOUNT_ID from env or fall back to the first account from the list.
    let account_id = std::env::var("ACCOUNT_ID")
        .ok()
        .or_else(|| accounts.data.account.first().map(|a| a.account_id.clone()))
        .expect("Provide ACCOUNT_ID env var or ensure you have at least one account");

    let account = client.get_account_into(&account_id).await?;
    println!("Account {account_id} details:\n{:#?}", account.data);

    Ok(())
}
