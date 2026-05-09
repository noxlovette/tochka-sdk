use tochka_sdk::{Client, CreatePaymentForSignPayload, PaymentForSignQuery, PaymentRecipient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client = Client::new().await?.with_customer_code().await?;
    let customer_code = client.customer_code.clone().unwrap();

    // Get the first account ID to use as source.
    let accounts = client.get_accounts_list().await?;
    let account_id = std::env::var("ACCOUNT_ID")
        .ok()
        .or_else(|| accounts.data.account.first().map(|a| a.account_id.clone()))
        .expect("Provide ACCOUNT_ID env var or ensure you have at least one account");

    // List pending payments for sign.
    let list = client
        .get_payment_for_sign_list(
            PaymentForSignQuery::new().customer_code(&customer_code),
        )
        .await?;
    println!("Pending payments for sign: {}", list.data.payment.len());

    // Create a test payment for sign.
    let recipient = PaymentRecipient::new(
        "ООО Пример",
        "7700000000",
        "40702810000000000000",
        "044525225",
        "ПАО Сбербанк",
        "30101810400000000225",
    );

    let payload = CreatePaymentForSignPayload::new(
        &customer_code,
        &account_id,
        10000, // 100 RUB in kopeks
        "Тестовый платёж",
        recipient,
    );

    let payment = client.create_payment_for_sign(payload).await?;
    println!("Created payment: {:?}", payment.data.request_id);

    // Check its status.
    let status = client
        .get_payment_for_sign_status(payment.data.request_id.to_string())
        .await?;
    println!("Status: {:?}", status.data.status);

    Ok(())
}
