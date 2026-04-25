use crate::TransactionStatement;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// RU: Выписка по счёту. EN: Account statement.
#[derive(Validate, Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statement {
    /// RU: Идентификатор счёта. EN: Account ID.
    #[validate(length(max = 40))]
    pub account_id: String,
    /// RU: Идентификатор выписки. EN: Statement ID.
    #[validate(length(max = 40))]
    pub statement_id: Option<String>,
    /// RU: Статус готовности. EN: Statement status.
    pub status: StatementStatus,
    /// RU: Начало периода. EN: Period start date.
    pub start_date_time: NaiveDate,
    /// RU: Конец периода. EN: Period end date.
    pub end_date_time: NaiveDate,
    /// RU: Время создания ресурса. EN: Resource creation time.
    pub creation_date_time: DateTime<Utc>,
    /// RU: Баланс на начало периода. EN: Starting balance.
    pub start_date_balance: Option<f64>,
    /// RU: Баланс на конец периода. EN: Closing balance.
    pub end_date_balance: Option<f64>,
    #[serde(rename = "Transaction")]
    /// RU: Транзакции выписки. EN: Statement transactions.
    pub transaction: Option<Vec<TransactionStatement>>,
}

/// RU: Запрос на создание выписки. EN: Statement creation payload.
#[derive(Validate, Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatementPayload {
    #[validate(length(max = 40))]
    pub account_id: String,
    pub start_date_time: NaiveDate,
    pub end_date_time: NaiveDate,
}

/// RU: Статус выписки. EN: Statement status.
#[derive(Serialize, Debug, Deserialize, Default)]
pub enum StatementStatus {
    #[default]
    Created,
    Processing,
    Error,
    Ready,
}

/// RU: Страница выписок. EN: Statement list page.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatementPageData {
    pub statement: Vec<Statement>,
}
