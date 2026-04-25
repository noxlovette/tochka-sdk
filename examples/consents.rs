use tochka_sdk::{Client, CreateConsentPayload, ExternalConsentTypeEnum};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client = Client::new().await?;

    // List existing consents.
    let consents = client.get_consents_list().await?;
    println!("Existing consents: {}", consents.data.consent.len());
    for c in &consents.data.consent {
        println!("  {} — {:?}", c.consent_id, c.status);
    }

    // Create a new consent requesting read access to accounts and balances.
    let new_consent = client
        .create_consent(CreateConsentPayload::new(vec![
            ExternalConsentTypeEnum::ReadAccountsDetail,
            ExternalConsentTypeEnum::ReadBalances,
            ExternalConsentTypeEnum::ReadTransactionsDetail,
        ]))
        .await?;

    println!("\nConsent created: {}", new_consent.data.consent_id);
    if let Some(url) = &new_consent.data.authorize_url {
        println!("Authorize at: {url}");
    }

    // Fetch details of the new consent.
    let info = client
        .get_consent_info(&new_consent.data.consent_id)
        .await?;
    println!("Consent status: {:?}", info.data.status);

    // Fetch any child consents.
    let children = client
        .get_child_consents(&new_consent.data.consent_id)
        .await?;
    println!("Child consents: {}", children.data.consent.len());

    Ok(())
}
