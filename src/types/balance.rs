use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::Amount;

/// RU: Модель баланса счёта. EN: Account balance model.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    /// RU: Идентификатор счёта. EN: Account identifier.
    pub account_id: String,
    /// RU: Признак кредит/дебет. EN: Credit/debit indicator.
    pub credit_debit_indicator: CreditDebitIndicator,
    #[serde(rename = "type")]
    /// RU: Тип баланса по ISO20022. EN: Balance static type (ISO20022).
    pub balance_type: BalanceType,
    /// RU: Дата и время расчёта. EN: Timestamp of balance build.
    pub date_time: DateTime<Utc>,
    #[serde(rename = "Amount")]
    /// RU: Сумма и валюта. EN: Amount with currency.
    pub amount: Amount,
}

/// RU: Кредит/дебет индикатор. EN: Credit/debit indicator.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum CreditDebitIndicator {
    Credit,
    Debit,
}

/// RU: Тип баланса. EN: Balance type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BalanceType {
    OpeningAvailable,
    ClosingAvailable,
    Expected,
    OverdraftAvailable,
}

/// RU: Страница с балансами. EN: Balance list page.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BalancePageData {
    /// RU: Список балансов. EN: Balance collection.
    pub balance: Vec<Balance>,
}
