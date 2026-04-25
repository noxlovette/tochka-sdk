use tochka_sdk::{Client, RegisterMerchantPayload, RegisterQrCodePayload, SbpPaymentsQuery, SbpRefundPayload};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client = Client::new().await?;

    let legal_id = std::env::var("LEGAL_ID").expect("Set LEGAL_ID env var");
    let account_id = std::env::var("ACCOUNT_ID").expect("Set ACCOUNT_ID env var");

    // Register a merchant (TSP) for the legal entity.
    let merchant = client
        .register_sbp_merchant(
            &legal_id,
            RegisterMerchantPayload::new("ООО Пример Магазин", "5411")
                .address("г. Москва, ул. Примерная, д. 1"),
        )
        .await?;
    println!("Merchant registered: {}", merchant.data.merchant_id);

    let merchant_id = &merchant.data.merchant_id;

    // Register a static QR code for the merchant.
    let static_qr = client
        .register_qr_code(
            merchant_id,
            &account_id,
            RegisterQrCodePayload::static_code().purpose("Оплата в магазине"),
        )
        .await?;
    println!("Static QR registered: {}", static_qr.data.qrc_id);
    println!("Payload: {:?}", static_qr.data.payload);

    // Register a dynamic QR code for 500 RUB.
    let dynamic_qr = client
        .register_qr_code(
            merchant_id,
            &account_id,
            RegisterQrCodePayload::dynamic(50000)
                .purpose("Оплата заказа №123")
                .ttl(30),
        )
        .await?;
    println!("Dynamic QR registered: {}", dynamic_qr.data.qrc_id);

    // Check payment status for the dynamic QR.
    let statuses = client
        .get_qr_codes_payment_status(&dynamic_qr.data.qrc_id)
        .await?;
    println!("QR payment statuses: {:#?}", statuses.data);

    // List SBP payments.
    let payments = client
        .get_sbp_payments(SbpPaymentsQuery {
            qrc_id: Some(dynamic_qr.data.qrc_id.clone()),
            ..Default::default()
        })
        .await?;
    println!("SBP payments found: {}", payments.data.payment.len());

    // Initiate a refund if there are any payments.
    if let Some(payment) = payments.data.payment.first() {
        let refund = client
            .start_sbp_refund(SbpRefundPayload::new(&payment.trx_id, payment.amount))
            .await?;
        println!("Refund initiated: {}", refund.data.request_id);
    }

    Ok(())
}
