use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// RU: Ответ на возврат платежа. EN: Payment refund response.
#[derive(Validate, Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Refund {
    /// RU: Признак, что операция — возврат. EN: Is refund flag.
    pub is_refund: bool,
    /// RU: Идентификатор операции. EN: Operation ID.
    pub operation_id: uuid::Uuid,
    /// RU: Сумма возврата. EN: Refund amount.
    pub amount: f64,
    /// RU: Дата возврата. EN: Refund date.
    pub date: NaiveDate,
    /// RU: Номер заказа/операции возврата. EN: Refund order identifier.
    pub order_id: String,
}

/// RU: Тело запроса на возврат. EN: Refund request payload.
#[derive(Validate, Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundPayload {
    #[validate(range(min = 0.0))]
    pub amount: f64,
}
