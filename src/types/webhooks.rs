use crate::{PaymentMode, PaymentStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// RU: Настройка вебхуков приложения. EN: Application webhook configuration.
#[derive(Serialize, Debug, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    /// RU: Список подписок. EN: Subscribed webhook types.
    pub webhooks_list: Vec<WebhookType>,
    #[validate(length(max = 2083))]
    /// RU: URL для отправки вебхуков. EN: Callback URL.
    pub url: String,
}

/// RU: Типы вебхуков. EN: Webhook event types.
#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum WebhookType {
    IncomingPayment,
    OutgoingPayment,
    IncomingSbpPayment,
    AcquiringInternetPayment,
    IncomingSbpB2BPayment,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AcquiringClaims {
    pub customer_code: String,
    pub amount: String,
    pub payment_type: PaymentMode,
    pub webhook_type: WebhookType,
    pub operation_id: Uuid,
    pub purpose: String,
    pub merchant_id: String,
    pub status: PaymentStatus,

    // optional fields depending on paymentType
    pub consumer_id: Option<Uuid>,    // present for `card`
    pub transaction_id: Option<Uuid>, // present for `sbp`
    pub qrc_id: Option<String>,       // sbp only
    pub payer_name: Option<String>,   // sbp only
}
