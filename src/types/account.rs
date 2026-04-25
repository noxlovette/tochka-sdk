use crate::ExternalType;
use chrono::{DateTime, NaiveDate, Utc};
use codes_iso_4217::CurrencyCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// RU: Модель счёта из API. EN: Account model from the API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    /// RU: Уникальный код клиента (9 символов). EN: Unique customer code (9 chars).
    #[validate(length(min = 9))]
    pub customer_code: String,
    /// RU: Идентификатор счёта. EN: Account identifier.
    pub account_id: String,
    /// RU: Транзитный счёт (при наличии). EN: Transit account if present.
    pub transit_account: Option<String>,
    /// RU: Статус счёта. EN: Account status.
    pub status: AccountStatus,
    /// RU: Дата/время обновления статуса. EN: Status update timestamp.
    pub status_update_date_time: DateTime<Utc>,
    /// RU: Валюта счёта (ISO 4217). EN: Account currency (ISO 4217).
    pub currency: CurrencyCode,
    /// RU: Тип клиента (физ/юр). EN: Customer type.
    pub account_type: ExternalType,
    /// RU: Подтип счёта. EN: Account subtype.
    pub account_sub_type: AccountSubType,
    /// RU: Дата регистрации счёта. EN: Account registration date.
    pub registration_date: NaiveDate,
    /// RU: Детали идентификаторов счёта. EN: Account identification details.
    pub account_details: Option<Vec<AccountDetail>>,
}

/// RU: Статус счёта. EN: Account status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountStatus {
    /// RU: Активен. EN: Enabled.
    Enabled,
    /// RU: Отключён. EN: Disabled.
    Disabled,
    /// RU: Удалён. EN: Deleted.
    Deleted,
    /// RU: Виртуальный/служебный. EN: Pro-forma.
    ProForma,
    /// RU: В ожидании. EN: Pending.
    Pending,
}

/// RU: Подтип счёта. EN: Account subtype.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountSubType {
    /// RU: Кредитная карта. EN: Credit card.
    CreditCard,
    /// RU: Текущий счёт. EN: Current account.
    CurrentAccount,
    /// RU: Кредит. EN: Loan.
    Loan,
    /// RU: Ипотека. EN: Mortgage.
    Mortgage,
    /// RU: Предоплаченная карта. EN: Prepaid card.
    PrePaidCard,
    /// RU: Сберегательный. EN: Savings.
    Savings,
    /// RU: Специальный. EN: Special.
    Special,
}

/// RU: Деталь идентификатора счёта. EN: Account identification detail.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AccountDetail {
    /// RU: Значение идентификатора. EN: Identification value.
    #[validate(length(max = 40))]
    pub identification: String,
    /// RU: Наименование идентификатора. EN: Identifier name.
    pub name: String,
    /// RU: Схема идентификации. EN: Identification scheme.
    pub scheme_name: String,
}

/// RU: Схема идентификации счёта. EN: Account identification scheme.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccountIdentification {
    #[serde(rename = "RU.CBR.PAN")]
    RUCBRPAN,
    #[serde(rename = "RU.CBR.CellphoneNumber")]
    RUCBRCellphoneNumber,
    #[serde(rename = "RU.CBR.BBAN")]
    RUCBRBBAN,
}

/// RU: Реквизиты счёта для контрагентов. EN: Cash account details for counterparties.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashAccount {
    /// RU: Номер счёта. EN: Account number.
    pub identification: Option<String>,
    /// RU: Схема идентификации. EN: Identification scheme.
    pub scheme_name: AccountIdentification,
}

/// RU: Страница со списком счетов. EN: Account list page payload.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountPageData {
    /// RU: Список счетов. EN: Accounts collection.
    pub account: Vec<Account>,
}
