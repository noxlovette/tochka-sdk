use chrono::DateTime;
use uuid::Uuid;

use tochka_sdk::{Data, PaymentMode, PaymentStatus, Refund, RegistryPageData, RetailerPageData};

#[test]
fn deserialize_refund_payment_example() {
    let json = r#"
{
  "Data": {
    "isRefund": true,
    "operationId": "48232c9a-ce82-1593-3cb6-5c85a1ffef8f",
    "amount": 500.0,
    "date": "2025-04-11",
    "orderId": "1"
  },
  "Links": {
    "self": "https://enter.tochka.com/uapi/acquiring/payments/48232c9a-ce82-1593-3cb6-5c85a1ffef8f/refund"
  },
  "Meta": {
    "totalPages": 1
  }
}
    "#;

    let parsed: Data<Refund> = serde_json::from_str(json).unwrap();

    assert!(parsed.data.is_refund);
    assert_eq!(
        parsed.data.operation_id,
        Uuid::parse_str("48232c9a-ce82-1593-3cb6-5c85a1ffef8f").unwrap()
    );
    assert_eq!(parsed.data.amount, 500.0);
    assert_eq!(parsed.data.order_id, "1");
}

#[test]
fn deserialize_payment_registry_example() {
    let json = r#"
{
  "Data": {
    "Registry": [
      {
        "purpose": "Футболка женская молочная",
        "status": "CREATED",
        "amount": 18548.39,
        "operationId": "48232c9a-ce82-1593-3cb6-5c85a1ffef8f",
        "time": "2022-10-18T08:28:59+00:00",
        "number": 123456,
        "commission": 100.0,
        "enrollmentAmount": 18448.39
      }
    ]
  },
  "Links": {
    "self": "https://enter.tochka.com/uapi/acquiring/registry"
  },
  "Meta": {
    "totalPages": 1
  }
}
    "#;

    let parsed: Data<RegistryPageData> = serde_json::from_str(json).unwrap();
    let item = &parsed.data.registry[0];

    assert_eq!(item.purpose, "Футболка женская молочная");
    assert!(matches!(item.status, PaymentStatus::Created));
    assert_eq!(item.amount, 18548.39);
    let parsed_time = DateTime::parse_from_rfc3339("2022-10-18T08:28:59+00:00")
        .unwrap()
        .with_timezone(&chrono::Utc);
    assert_eq!(item.time, parsed_time);
    assert_eq!(item.number, 123456);
    assert_eq!(item.enrollment_amount, 18448.39);
}

#[test]
fn deserialize_retailer_list_example() {
    let json = r#"
{
  "Data": {
    "Retailer": [
      {
        "status": "REG",
        "isActive": true,
        "mcc": "5111",
        "rate": 2.6,
        "name": "ООО Альтер",
        "url": "https://alter.ru",
        "merchantId": "200000000001056",
        "terminalId": "20000032",
        "paymentModes": [
          "sbp",
          "card"
        ],
        "cashbox": "businessRu"
      }
    ]
  },
  "Links": {
    "self": "https://enter.tochka.com/uapi/acquiring/retailers"
  },
  "Meta": {
    "totalPages": 1
  }
}
    "#;

    let parsed: Data<RetailerPageData> = serde_json::from_str(json).unwrap();
    let retailer = &parsed.data.retailer[0];

    assert!(retailer.is_active);
    assert_eq!(retailer.status, tochka_sdk::RetailerStatus::REG);
    assert_eq!(
        retailer.payment_modes,
        vec![PaymentMode::Sbp, PaymentMode::Card]
    );
    assert_eq!(retailer.merchant_id, "200000000001056");
    assert_eq!(parsed.meta.total_pages, 1);
}
