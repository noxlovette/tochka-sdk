use tochka_sdk::{Client, CreateClosingDocumentPayload, CreateInvoicePayload, SendDocumentEmailPayload};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client = Client::new().await?.with_client_code().await?;
    let customer_code = client.customer_code.clone().unwrap();

    // Create an invoice.
    let invoice = client
        .create_invoice(
            CreateInvoicePayload::new(&customer_code, 500.0, "Оплата услуг")
                .email("buyer@example.com"),
        )
        .await?;

    println!("Invoice created: {}", invoice.data.document_id);
    println!("Payment URL: {:?}", invoice.data.payment_url);

    // Send it to the buyer's email.
    client
        .send_invoice_email(
            &customer_code,
            &invoice.data.document_id,
            SendDocumentEmailPayload::new("buyer@example.com"),
        )
        .await?;
    println!("Invoice sent to buyer@example.com");

    // Check payment status.
    let status = client
        .get_invoice_payment_status(&customer_code, &invoice.data.document_id)
        .await?;
    println!("Invoice status: {:?}", status.data.status);

    // Create a closing document.
    let doc = client
        .create_closing_document(
            CreateClosingDocumentPayload::new(&customer_code, 500.0, "Акт выполненных работ"),
        )
        .await?;
    println!("Closing document created: {}", doc.data.document_id);

    Ok(())
}
