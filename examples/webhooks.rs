use tochka_sdk::{Client, Webhook, WebhookType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = Client::new().await?;

    let callback_url =
        std::env::var("WEBHOOK_URL").unwrap_or_else(|_| "https://example.com/webhook".into());

    let payload = Webhook {
        url: callback_url.clone(),
        webhooks_list: vec![
            WebhookType::IncomingPayment,
            WebhookType::AcquiringInternetPayment,
        ],
    };

    let created = client.create_webhook(payload.clone()).await?;
    println!("Created webhook:\n{:#?}", created.data);

    let updated = client
        .edit_webhook(Webhook {
            url: format!("{callback_url}?v=2"),
            webhooks_list: vec![WebhookType::OutgoingPayment],
        })
        .await?;
    println!("Updated webhook:\n{:#?}", updated.data);

    let current = client.get_webhooks().await?;
    println!("Current webhook config:\n{:#?}", current.data);

    let test_send = client.send_webhook(WebhookType::IncomingPayment).await?;
    println!("Webhook test send:\n{:#?}", test_send.data);

    let deleted = client.delete_webhook().await?;
    println!("Webhook deleted:\n{:#?}", deleted.data);

    Ok(())
}
