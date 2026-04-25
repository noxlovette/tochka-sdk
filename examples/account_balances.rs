use tochka_sdk::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = Client::new().await?;

    // Pick account ID either from env or from the first available account.
    let account_id = match std::env::var("ACCOUNT_ID") {
        Ok(val) => val,
        Err(_) => {
            let accounts = client.get_accounts_list().await?;
            accounts
                .data
                .account
                .first()
                .map(|a| a.account_id.clone())
                .expect("Provide ACCOUNT_ID env var or have at least one account available")
        }
    };

    let balance = client.get_balance_info(&account_id).await?;
    println!("Balance for {account_id}:\n{:#?}", balance.data);

    let card_transactions = client.get_authorized_card_transactions(&account_id).await?;
    println!(
        "Authorized card transactions for {account_id}:\n{:#?}",
        card_transactions.data.transactions
    );

    let balances_list = client.get_balances_list().await?;
    println!(
        "Balances across all accounts:\n{:#?}",
        balances_list.data.balance
    );

    Ok(())
}
