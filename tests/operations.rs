use chrono::DateTime;
use tochka_sdk::{
    Data, PaginatedResponse, PaymentMode, PaymentOperation, PaymentPageData, PaymentStatus,
};
use uuid::uuid;

#[test]
fn deserialize_payment_operation_creation_example() {
    let json = r#"
{
  "Data": {
    "purpose": "Футболка женская молочная",
    "status": "CREATED",
    "amount": 1234.00,
    "operationId": "48232c9a-ce82-1593-3cb6-5c85a1ffef8f",
    "paymentLink": "https://merch.example.com/order/?uuid=16ea4c54-bf1d-4e6a-a1ef-53ad55666e43",
    "consumerId": "fedac807-078d-45ac-a43b-5c01c57edbf8",
    "merchantId": "200000000001056",
    "preAuthorization": true,
    "ttl": 10080,
    "paymentLinkId": "order-123",
    "paymentMode": [
      "sbp",
      "card"
    ]
  },
  "Links": {
    "self": "https://enter.tochka.com/uapi/acquiring/payments"
  },
  "Meta": {
    "totalPages": 1
  }
}
    "#;

    let parsed: Data<PaymentOperation> = serde_json::from_str(json).unwrap();

    assert_eq!(
        parsed.data.purpose.as_deref(),
        Some("Футболка женская молочная")
    );
    assert!(matches!(parsed.data.status, PaymentStatus::Created));
    assert_eq!(parsed.data.amount, 1234.0);
    assert_eq!(
        parsed.data.operation_id,
        uuid!("48232c9a-ce82-1593-3cb6-5c85a1ffef8f")
    );
    assert_eq!(parsed.data.payment_link_id.as_deref(), Some("order-123"));
    assert_eq!(
        parsed.data.payment_mode.as_ref().unwrap().as_slice(),
        &[PaymentMode::Sbp, PaymentMode::Card]
    );
    assert_eq!(parsed.data.pre_authorization, Some(true));
}

#[test]
fn deserialize_payment_operation_list_example() {
    let json = r#"
{
  "Data": {
    "Operation": [
      {
        "customerCode": "300000092",
        "taxSystemCode": "osn",
        "paymentType": "card",
        "paymentId": "A22031016256670100000533E625FCB3",
        "transactionId": "48232c9a-ce82-1593-3cb6-5c85a1ffef8f",
        "createdAt": "2022-10-18T08:28:59+00:00",
        "paymentMode": [
          "sbp",
          "card",
          "tinkoff",
          "dolyame"
        ],
        "redirectUrl": "https://example.com",
        "failRedirectUrl": "https://example.com/fail",
        "purpose": "Перевод за оказанные услуги",
        "amount": 1234.00,
        "status": "CREATED",
        "operationId": "48232c9a-ce82-1593-3cb6-5c85a1ffef8f",
        "paymentLink": "https://merch.example.com/order/?uuid=16ea4c54-bf1d-4e6a-a1ef-53ad55666e43",
        "merchantId": "200000000001056",
        "consumerId": "fedac807-078d-45ac-a43b-5c01c57edbf8",
        "Order": [
          {
            "orderId": "ord-1",
            "type": "refund",
            "amount": 500.0,
            "time": "2023-01-01T11:00:00+00:00"
          }
        ],
        "preAuthorization": false,
        "paymentLinkId": "order-123"
      }
    ]
  },
  "Links": {
    "self": "https://enter.tochka.com/uapi/acquiring/payments",
    "next": "https://enter.tochka.com/uapi/acquiring/payments?page=2"
  },
  "Meta": {
    "totalPages": 2
  }
}
    "#;

    let parsed: PaginatedResponse<PaymentPageData> = serde_json::from_str(json).unwrap();
    let operation = &parsed.data.operation[0];

    assert_eq!(operation.customer_code.as_deref(), Some("300000092"));
    assert_eq!(
        operation.transaction_id,
        Some(uuid!("48232c9a-ce82-1593-3cb6-5c85a1ffef8f"))
    );
    assert!(matches!(operation.status, PaymentStatus::Created));
    assert_eq!(
        operation.payment_mode.as_deref().unwrap(),
        &[
            PaymentMode::Sbp,
            PaymentMode::Card,
            PaymentMode::Tinkoff,
            PaymentMode::Dolyame
        ]
    );
    assert_eq!(operation.amount, 1234.0);
    assert_eq!(operation.order.as_ref().unwrap().len(), 1);
    let created_at = DateTime::parse_from_rfc3339("2022-10-18T08:28:59+00:00")
        .unwrap()
        .with_timezone(&chrono::Utc);
    assert_eq!(operation.created_at, Some(created_at));
    assert_eq!(parsed.meta.total_pages, 2);
    assert_eq!(
        parsed.links.next.as_deref(),
        Some("https://enter.tochka.com/uapi/acquiring/payments?page=2")
    );
}
