use chrono::{DateTime, NaiveDate};
use codes_iso_4217::CurrencyCode;
use tochka_sdk::{
    Account, AccountDetail, AccountPageData, AccountStatus, AccountSubType, Data, ExternalType,
};

#[test]
fn deserialize_accounts_list_example() {
    let json = r#"
{
  "Data": {
    "Account": [
      {
        "customerCode": "300000092",
        "accountId": "40817810802000000008/044525104",
        "transitAccount": "7272727272",
        "status": "Enabled",
        "statusUpdateDateTime": "2019-01-01T06:06:06.364+00:00",
        "currency": "RUB",
        "accountType": "Personal",
        "accountSubType": "CurrentAccount",
        "registrationDate": "2020-10-20",
        "accountDetails": [
          {
            "schemeName": "RU.CBR.AccountNumber",
            "identification": "40817810802000000008/044525104",
            "name": "Основной текущий счёт"
          }
        ]
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

    let parsed: Data<AccountPageData> = serde_json::from_str(json).unwrap();
    let account = &parsed.data.account[0];

    assert_eq!(account.customer_code, "300000092");
    assert_eq!(account.status, AccountStatus::Enabled);
    assert_eq!(account.currency, CurrencyCode::RUB);
    assert_eq!(account.account_sub_type, AccountSubType::CurrentAccount);
    assert_eq!(account.account_type, ExternalType::Personal);
    assert_eq!(
        account.status_update_date_time,
        DateTime::parse_from_rfc3339("2019-01-01T06:06:06.364+00:00")
            .unwrap()
            .with_timezone(&chrono::Utc)
    );
    assert_eq!(
        account.registration_date,
        NaiveDate::from_ymd_opt(2020, 10, 20).unwrap()
    );
    let details: &AccountDetail = &account.account_details.as_ref().unwrap()[0];
    assert_eq!(details.scheme_name, "RU.CBR.AccountNumber");
    assert_eq!(details.identification, "40817810802000000008/044525104");
}

#[test]
fn deserialize_account_info_example() {
    let json = r#"
{
  "Data": {
    "customerCode": "300000092",
    "accountId": "40817810802000000008/044525104",
    "transitAccount": "7272727272",
    "status": "Enabled",
    "statusUpdateDateTime": "2019-01-01T06:06:06.364+00:00",
    "currency": "RUB",
    "accountType": "Personal",
    "accountSubType": "CurrentAccount",
    "registrationDate": "2020-10-20",
    "accountDetails": [
      {
        "schemeName": "RU.CBR.AccountNumber",
        "identification": "40817810802000000008/044525104",
        "name": "Основной текущий счёт"
      }
    ]
  },
  "Links": {
    "self": "https://enter.tochka.com/uapi/accounts/40817810802000000008/044525104"
  },
  "Meta": {
    "totalPages": 1
  }
}
    "#;

    let parsed: Data<Account> = serde_json::from_str(json).unwrap();
    let account = &parsed.data;

    assert_eq!(account.account_id, "40817810802000000008/044525104");
    assert_eq!(account.status, AccountStatus::Enabled);
    assert_eq!(account.currency, CurrencyCode::RUB);
    assert_eq!(parsed.meta.total_pages, 1);
    assert_eq!(
        account.account_details.as_ref().unwrap()[0].name,
        "Основной текущий счёт"
    );
}
