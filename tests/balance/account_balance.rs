use chrono::DateTime;
use codes_iso_4217::CurrencyCode;
use tochka_sdk::{
    BalancePageData, BalanceType, CreditDebitIndicator, Data, PaginatedResponse,
    TransactionPageData,
};

#[test]
fn deserialize_authorized_card_transactions_example() {
    let json = r#"
{
  "Data": {
    "Transactions": [
      {
        "accountId": "40817810802000000008/044525104",
        "pan": "string",
        "dateTime": "2019-01-01T06:06:06.364+00:00",
        "Amount": {
          "amount": 1234.56,
          "currency": "RUB"
        },
        "AccountAmount": {
          "amount": 1234.56,
          "currency": "RUB"
        },
        "TerminalData": {
          "city": "Perm",
          "location": "Ekaterinburg",
          "owner": "string"
        }
      }
    ]
  },
  "Links": {
    "self": "https://enter.tochka.com/uapi"
  },
  "Meta": {
    "totalPages": 1
  }
}
    "#;

    let parsed: Data<TransactionPageData> = serde_json::from_str(json).unwrap();
    let tx = &parsed.data.transactions[0];

    let expected_date = DateTime::parse_from_rfc3339("2019-01-01T06:06:06.364+00:00")
        .unwrap()
        .with_timezone(&chrono::Utc);
    assert_eq!(tx.account_id, "40817810802000000008/044525104");
    assert_eq!(tx.pan, "string");
    assert_eq!(tx.date_time, expected_date);
    assert_eq!(tx.amount.amount, 1234.56);
    assert_eq!(tx.amount.currency, CurrencyCode::RUB);
    assert_eq!(tx.account_amount.currency, CurrencyCode::RUB);
    assert_eq!(tx.terminal_data.city.as_deref(), Some("Perm"));
    assert_eq!(tx.terminal_data.owner.as_deref(), Some("string"));
}

#[test]
fn deserialize_balance_info_example() {
    let json = r#"
{
  "Data": {
    "Balance": [
      {
        "accountId": "40817810802000000008/044525104",
        "creditDebitIndicator": "Credit",
        "type": "ClosingAvailable",
        "dateTime": "2019-01-01T06:06:06.364+00:00",
        "Amount": {
          "amount": 1234.56,
          "currency": "RUB"
        }
      }
    ]
  },
  "Links": {
    "self": "https://enter.tochka.com/uapi"
  },
  "Meta": {
    "totalPages": 1
  }
}
    "#;

    let parsed: Data<BalancePageData> = serde_json::from_str(json).unwrap();
    let balance = &parsed.data.balance[0];

    assert_eq!(balance.account_id, "40817810802000000008/044525104");
    assert_eq!(balance.credit_debit_indicator, CreditDebitIndicator::Credit);
    assert_eq!(balance.balance_type, BalanceType::ClosingAvailable);
    assert_eq!(balance.amount.amount, 1234.56);
    assert_eq!(balance.amount.currency, CurrencyCode::RUB);
}

#[test]
fn deserialize_balances_list_example() {
    let json = r#"
{
  "Data": {
    "Balance": [
      {
        "accountId": "40817810802000000008/044525104",
        "creditDebitIndicator": "Credit",
        "type": "OpeningAvailable",
        "dateTime": "2019-01-01T06:06:06.364+00:00",
        "Amount": {
          "amount": 1234.56,
          "currency": "RUB"
        }
      },
      {
        "accountId": "40702810500000000001/044525104",
        "creditDebitIndicator": "Debit",
        "type": "Expected",
        "dateTime": "2019-02-02T10:10:10.100+00:00",
        "Amount": {
          "amount": 6543.21,
          "currency": "RUB"
        }
      }
    ]
  },
  "Links": {
    "self": "https://enter.tochka.com/uapi",
    "next": "https://enter.tochka.com/uapi?page=2"
  },
  "Meta": {
    "totalPages": 2
  }
}
    "#;

    let parsed: PaginatedResponse<BalancePageData> = serde_json::from_str(json).unwrap();

    assert_eq!(parsed.data.balance.len(), 2);
    assert_eq!(parsed.links.this, "https://enter.tochka.com/uapi");
    assert_eq!(
        parsed.links.next.as_deref(),
        Some("https://enter.tochka.com/uapi?page=2")
    );
    assert_eq!(parsed.meta.total_pages, 2);
    assert_eq!(
        parsed.data.balance[1].credit_debit_indicator,
        CreditDebitIndicator::Debit
    );
    assert_eq!(parsed.data.balance[1].balance_type, BalanceType::Expected);
    assert_eq!(parsed.data.balance[1].amount.currency, CurrencyCode::RUB);
}
