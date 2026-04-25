use chrono::{Duration, Utc};
use tochka_sdk::{Client, StatementPayload};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = Client::new().await?;

    let account_id = std::env::var("ACCOUNT_ID")?;
    let end_date = Utc::now().date_naive();
    let start_date = end_date - Duration::days(30);

    // Create a new statement for the last 30 days.
    let created = client
        .init_statement(StatementPayload {
            account_id: account_id.clone(),
            start_date_time: start_date,
            end_date_time: end_date,
        })
        .await?;
    println!("Statement init response:\n{:#?}", created.data.statement);

    // List all statements available to the account.
    let statements = client.get_statements_list().await?;
    println!("Existing statements:\n{:#?}", statements.data.statement);

    // Pick the statement ID either from the init response, list, or env fallback.
    let statement_id = created
        .data
        .statement
        .first()
        .and_then(|s| s.statement_id.clone())
        .or_else(|| {
            statements
                .data
                .statement
                .first()
                .and_then(|s| s.statement_id.clone())
        })
        .or_else(|| std::env::var("STATEMENT_ID").ok())
        .expect("Provide STATEMENT_ID env var or create a statement with a returned id");

    let statement = client.get_statement(&account_id, &statement_id).await?;
    println!(
        "Statement {statement_id} details:\n{:#?}",
        statement.data.statement
    );

    Ok(())
}
