use chrono::{DateTime, NaiveDate};
use codes_iso_4217::CurrencyCode;
use tochka_sdk::{
    AccountIdentification, Data, FinancialInstitutionIdentification, PaginatedResponse,
    StatementPageData, StatementStatus, TransactionStatus,
};

#[test]
fn deserialize_statements_list_example() {
    let json = r#"
{
  "Data": {
    "Statement": [
      {
        "accountId": "40817810802000000008/044525104",
        "statementId": "23489",
        "status": "Ready",
        "startDateTime": "2019-01-01",
        "endDateTime": "2019-01-31",
        "creationDateTime": "2019-02-01T06:06:06.364+00:00",
        "startDateBalance": 1234.5,
        "endDateBalance": 2234.5
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

    let parsed: Data<StatementPageData> = serde_json::from_str(json).unwrap();
    let statement = &parsed.data.statement[0];

    assert_eq!(statement.account_id, "40817810802000000008/044525104");
    assert_eq!(
        statement.creation_date_time,
        DateTime::parse_from_rfc3339("2019-02-01T06:06:06.364+00:00")
            .unwrap()
            .with_timezone(&chrono::Utc)
    );
    assert!(matches!(statement.status, StatementStatus::Ready));
    assert_eq!(
        statement.start_date_time,
        NaiveDate::from_ymd_opt(2019, 1, 1).unwrap()
    );
    assert_eq!(statement.start_date_balance, Some(1234.5));
    assert_eq!(parsed.meta.total_pages, 1);
}

#[test]
fn deserialize_statement_with_transactions_example() {
    let json = r#"
{
  "Data": {
    "Statement": [
      {
        "accountId": "40817810802000000008/044525104",
        "statementId": "23489",
        "status": "Ready",
        "startDateTime": "2019-01-01",
        "endDateTime": "2019-01-31",
        "creationDateTime": "2019-02-01T06:06:06.364+00:00",
        "startDateBalance": 1234.5,
        "endDateBalance": 2234.5,
        "Transaction": [
          {
            "transactionId": "23489",
            "paymentId": "abcd-11234",
            "creditDebitIndicator": "Credit",
            "status": "Booked",
            "documentNumber": "123456",
            "transactionTypeCode": "Платежный ордер",
            "documentProcessDate": "2019-01-15",
            "description": "string",
            "Amount": {
              "amount": 1000.0,
              "currency": "RUB"
            },
            "DebtorParty": {
              "inn": "1234567890",
              "kpp": "123456789",
              "name": "ООО Ромашка"
            },
            "DebtorAccount": {
              "identification": "40702810900000000001",
              "schemeName": "RU.CBR.PAN"
            },
            "DebtorAgent": {
              "accountIdentification": "30101810200000000593",
              "identification": "044525593",
              "name": "АО \"Банк\"",
              "schemeName": "RU.CBR.BICFI"
            },
            "CreditorParty": {
              "inn": "9876543210",
              "kpp": "987654321",
              "name": "ООО Василек"
            },
            "CreditorAccount": {
              "identification": "40802810200000000002",
              "schemeName": "RU.CBR.PAN"
            },
            "CreditorAgent": {
              "accountIdentification": "30101810600000000653",
              "identification": "046577674",
              "name": "ПАО \"Другой банк\"",
              "schemeName": "RU.CBR.BICFI"
            },
            "TaxFields": {}
          }
        ]
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

    let parsed: PaginatedResponse<StatementPageData> = serde_json::from_str(json).unwrap();
    let tx = &parsed.data.statement[0].transaction.as_ref().unwrap()[0];

    assert_eq!(tx.transaction_id.as_deref(), Some("23489"));
    assert!(matches!(tx.status, TransactionStatus::Booked));
    assert_eq!(tx.subfields.amount.amount, 1000.0);
    assert_eq!(tx.subfields.amount.currency, CurrencyCode::RUB);
    assert_eq!(
        tx.subfields.debtor_account.scheme_name,
        AccountIdentification::RUCBRPAN
    );
    assert_eq!(
        tx.subfields.debtor_agent.scheme_name,
        FinancialInstitutionIdentification::RuCbrBicfi
    );
    assert_eq!(parsed.meta.total_pages, 2);
    assert_eq!(
        parsed.links.next.as_deref(),
        Some("https://enter.tochka.com/uapi?page=2")
    );
}
