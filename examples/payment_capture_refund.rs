use tochka_sdk::{Client, RefundPayload};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = Client::new().await?;

    let operation_id = std::env::var("OPERATION_ID")
        .expect("Set OPERATION_ID with a two-step payment operation id");

    let capture = client.capture_payment(&operation_id).await?;
    println!("Capture result:\n{:#?}", capture.data);

    let refund_amount = std::env::var("REFUND_AMOUNT")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(1.0);

    let refund = client
        .refund_payment_operation(
            &operation_id,
            RefundPayload {
                amount: refund_amount,
            },
        )
        .await?;
    println!("Refund result:\n{:#?}", refund.data);

    let status = client.payment_operation_info(&operation_id).await?;
    println!(
        "Operation status after capture/refund:\n{:#?}",
        status.data.operation
    );

    Ok(())
}
