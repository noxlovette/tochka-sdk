use crate::{PaymentMode, PaymentStatus};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// RU: Параметры запроса реестра платежей. EN: Payment registry query params.
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRegistryQuery {
    /// RU: Код клиента. EN: Customer code.
    pub customer_code: String,
    /// RU: Идентификатор мерчанта. EN: Merchant ID.
    pub merchant_id: String,
    /// RU: Идентификатор платежа. EN: Payment ID.
    pub payment_id: String,
    /// RU: Дата реестра. EN: Registry date.
    pub date: NaiveDate,
}

impl PaymentRegistryQuery {
    pub fn new(
        customer_code: impl Into<String>,
        merchant_id: impl Into<String>,
        payment_id: impl Into<String>,
        date: NaiveDate,
    ) -> Self {
        Self {
            customer_code: customer_code.into(),
            merchant_id: merchant_id.into(),
            payment_id: payment_id.into(),
            date,
        }
    }
}
/// RU: Запись реестра (агрегированная). EN: Registry bucket entry.
#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Registry {
    /// RU: Способ оплаты. EN: Payment mode.
    pub payment_type: PaymentMode,
    /// RU: Сумма всех позиций. EN: Total amount.
    pub total_amount: f64,
    /// RU: Идентификатор платежа. EN: Payment ID.
    pub payment_id: Option<String>,
    /// RU: Детализация платежей. EN: Payments breakdown.
    pub payments: RegistryPayment,
}

/// RU: Детализация платежа в реестре. EN: Payment line in registry.
#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryPayment {
    /// RU: Назначение платежа. EN: Payment purpose.
    pub purpose: String,
    /// RU: Статус платежа. EN: Payment status.
    pub status: PaymentStatus,
    /// RU: Сумма платежа. EN: Payment amount.
    pub amount: f64,
    /// RU: Идентификатор операции. EN: Operation ID.
    pub operation_id: Uuid,
    /// RU: Время платежа. EN: Payment time.
    pub time: DateTime<Utc>,
    /// RU: Номер платежа. EN: Payment number.
    pub number: u32,
    /// RU: Комиссия. EN: Commission.
    pub commission: f64,
    /// RU: Сумма к зачислению. EN: Enrollment amount.
    pub enrollment_amount: f64,
}

/// RU: Страница с реестром платежей. EN: Registry page payload.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegistryPageData {
    /// RU: Список платежей реестра. EN: Registry payments list.
    pub registry: Vec<RegistryPayment>,
}
