use std::io::{Read, stdin};
use tochka_sdk::{Client, CreatePaymentPayload, PaymentListQuery, PaymentMode, PaymentPath};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client = Client::new().await?;

    // Schritt 1: Zahlung erstellen
    let create = client
        .create_payment_operation(
            CreatePaymentPayload::new(10.0, client.customer_code.clone(), "Оплата услуг")
                .payment_modes([PaymentMode::Card, PaymentMode::Sbp]),
            PaymentPath::Standard,
        )
        .await?;

    println!("Zahlung erstellt:");
    println!("{:#?}", create);
    println!();
    println!("Öffne den Link, führe die Zahlung durch und drücke anschließend ENTER…");

    // Warten auf Benutzereingabe
    let mut buf = [0u8; 1];
    let _ = stdin().read(&mut buf)?;

    // Schritt 2: aktualisierte Info abfragen
    let operation = client
        .payment_operation_info(create.data.operation_id)
        .await?;

    println!("Aktueller Status:");
    println!("{:#?}", operation);

    println!();
    println!("ENTER drücken, um die Liste aller Operationen abzurufen…");
    let _ = stdin().read(&mut buf)?;

    // Schritt 3: Liste abrufen
    let list = client
        .payment_operation_list(PaymentListQuery::new(client.customer_code.clone()))
        .await?;

    println!("Operationen:");
    println!("{:#?}", list);

    Ok(())
}
