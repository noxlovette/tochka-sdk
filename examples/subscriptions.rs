use tochka_sdk::{ChargeSubscriptionPayload, Client, CreateSubscriptionPayload, SubscriptionStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client = Client::new().await?.with_client_code().await?;
    let customer_code = client.customer_code.clone().unwrap();

    // Create a subscription.
    let sub = client
        .create_subscription(
            CreateSubscriptionPayload::new(&customer_code, 299.0, "Ежемесячная подписка")
                .redirect_url("https://example.com/success")
                .fail_redirect_url("https://example.com/fail"),
        )
        .await?;

    println!("Subscription created: {}", sub.data.operation_id);
    println!("Payment link: {:?}", sub.data.payment_link);

    let operation_id = sub.data.operation_id;

    // Get its status.
    let status = client.get_subscription_status(operation_id).await?;
    println!("Status: {:?}", status.data.status);

    // List all subscriptions.
    let list = client.get_subscriptions_list().await?;
    println!("Total subscriptions: {}", list.data.subscription.len());

    // Charge the subscription.
    let charge = client
        .charge_subscription(
            operation_id,
            ChargeSubscriptionPayload::new(299.0, "Списание за март"),
        )
        .await?;
    println!("Charge result: {:?}", charge.data.status);

    // Suspend the subscription.
    client
        .set_subscription_status(operation_id, SubscriptionStatus::Suspended)
        .await?;
    println!("Subscription suspended");

    Ok(())
}
