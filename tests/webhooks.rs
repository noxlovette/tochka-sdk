use tochka_sdk::{Data, Webhook, WebhookType};

#[test]
fn deserialize_webhook_response_example() {
    let json = r#"
{
  "Data": {
    "webhooksList": [
      "incomingPayment",
      "acquiringInternetPayment"
    ],
    "url": "https://example.com/webhook"
  },
  "Links": {
    "self": "https://enter.tochka.com/uapi/webhook/v1.0/4ZY5qFuPsWdz3BfcG1RR5F4ZWOOCwLFI"
  },
  "Meta": {
    "totalPages": 1
  }
}
    "#;

    let parsed: Data<Webhook> = serde_json::from_str(json).unwrap();

    assert_eq!(parsed.data.url, "https://example.com/webhook");
    assert!(matches!(
        parsed.data.webhooks_list.as_slice(),
        [
            WebhookType::IncomingPayment,
            WebhookType::AcquiringInternetPayment
        ]
    ));
}
