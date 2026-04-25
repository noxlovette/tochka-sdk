use crate::{ReceiptClient, ReceiptItem, TaxSystemCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;
use validator::Validate;

/// RU: Статус подписки. EN: Subscription status.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionStatus {
    /// RU: Активна. EN: Active.
    Active,
    /// RU: Приостановлена. EN: Suspended.
    Suspended,
    /// RU: Отменена. EN: Cancelled.
    Cancelled,
    /// RU: Завершена. EN: Completed.
    Completed,
}

/// RU: Подписка по карте. EN: Card-based subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    /// RU: Идентификатор операции. EN: Operation ID.
    pub operation_id: Uuid,
    /// RU: Уникальный код клиента. EN: Customer code.
    pub customer_code: Option<String>,
    /// RU: Идентификатор покупателя. EN: Consumer ID.
    pub consumer_id: Option<String>,
    /// RU: Идентификатор мерчанта. EN: Merchant ID.
    pub merchant_id: Option<String>,
    /// RU: Сумма. EN: Amount.
    pub amount: f64,
    /// RU: Назначение. EN: Purpose.
    pub purpose: Option<String>,
    /// RU: Статус подписки. EN: Subscription status.
    pub status: Option<SubscriptionStatus>,
    /// RU: Ссылка на первый платёж. EN: Payment link for the first charge.
    pub payment_link: Option<String>,
    /// RU: Дата создания. EN: Creation timestamp.
    pub created_at: Option<DateTime<Utc>>,
}

/// RU: Запрос создания подписки. EN: Create subscription request payload.
#[derive(Debug, Clone, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubscriptionPayload {
    /// RU: Уникальный код клиента (9 символов). EN: Customer code (9 chars).
    #[validate(length(equal = 9))]
    pub customer_code: String,
    /// RU: Сумма для списания. EN: Charge amount.
    #[validate(range(min = 0.01))]
    pub amount: f64,
    /// RU: Назначение платежа (до 210 символов). EN: Payment purpose (up to 210 chars).
    #[validate(length(min = 1, max = 210))]
    pub purpose: String,
    /// RU: Идентификатор покупателя. EN: Consumer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_id: Option<String>,
    /// RU: URL перенаправления после успешной оплаты. EN: Redirect URL on payment success.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// RU: URL при неуспешной оплате. EN: Redirect URL on payment failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_redirect_url: Option<String>,
    /// RU: Идентификатор мерчанта. EN: Merchant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
}

impl CreateSubscriptionPayload {
    /// RU: Создать запрос подписки. EN: Build a create-subscription request.
    pub fn new(
        customer_code: impl Into<String>,
        amount: f64,
        purpose: impl Into<String>,
    ) -> Self {
        Self {
            customer_code: customer_code.into(),
            amount,
            purpose: purpose.into(),
            consumer_id: None,
            redirect_url: None,
            fail_redirect_url: None,
            merchant_id: None,
        }
    }

    /// RU: Установить идентификатор покупателя. EN: Set consumer ID.
    pub fn consumer_id(mut self, id: impl Into<String>) -> Self {
        self.consumer_id = Some(id.into());
        self
    }

    /// RU: Установить URL успешного перенаправления. EN: Set success redirect URL.
    pub fn redirect_url(mut self, url: impl Into<String>) -> Self {
        self.redirect_url = Some(url.into());
        self
    }

    /// RU: Установить URL неуспешного перенаправления. EN: Set failure redirect URL.
    pub fn fail_redirect_url(mut self, url: impl Into<String>) -> Self {
        self.fail_redirect_url = Some(url.into());
        self
    }

    /// RU: Установить идентификатор мерчанта. EN: Set merchant ID.
    pub fn merchant_id(mut self, id: impl Into<String>) -> Self {
        self.merchant_id = Some(id.into());
        self
    }
}

/// RU: Запрос создания подписки с чеком. EN: Create subscription with receipt payload.
#[derive(Debug, Clone, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubscriptionWithReceiptPayload {
    /// RU: Уникальный код клиента (9 символов). EN: Customer code (9 chars).
    #[validate(length(equal = 9))]
    pub customer_code: String,
    /// RU: Сумма. EN: Amount.
    #[validate(range(min = 0.01))]
    pub amount: f64,
    /// RU: Назначение (до 210 символов). EN: Purpose (up to 210 chars).
    #[validate(length(min = 1, max = 210))]
    pub purpose: String,
    /// RU: Система налогообложения. EN: Tax system code.
    pub tax_system_code: TaxSystemCode,
    /// RU: Данные покупателя для чека. EN: Receipt client data.
    #[serde(rename = "Client")]
    pub client: ReceiptClient,
    /// RU: Список товаров/услуг. EN: Order items.
    pub items: Vec<ReceiptItem>,
    /// RU: URL успешного перенаправления. EN: Success redirect URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// RU: URL неуспешного перенаправления. EN: Failure redirect URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_redirect_url: Option<String>,
    /// RU: Идентификатор мерчанта. EN: Merchant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
}

/// RU: Запрос списания по подписке. EN: Charge subscription payload.
#[derive(Debug, Clone, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargeSubscriptionPayload {
    /// RU: Сумма для списания. EN: Charge amount.
    #[validate(range(min = 0.01))]
    pub amount: f64,
    /// RU: Назначение (до 210 символов). EN: Purpose (up to 210 chars).
    #[validate(length(min = 1, max = 210))]
    pub purpose: String,
}

impl ChargeSubscriptionPayload {
    /// RU: Создать запрос списания. EN: Build a charge-subscription request.
    pub fn new(amount: f64, purpose: impl Into<String>) -> Self {
        Self {
            amount,
            purpose: purpose.into(),
        }
    }
}

/// RU: Запрос установки статуса подписки. EN: Set subscription status payload.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSubscriptionStatusPayload {
    /// RU: Новый статус подписки. EN: New subscription status.
    pub status: SubscriptionStatus,
}

/// RU: Ответ статуса подписки. EN: Subscription status response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionStatusResponse {
    /// RU: Идентификатор операции. EN: Operation ID.
    pub operation_id: Uuid,
    /// RU: Текущий статус. EN: Current status.
    pub status: SubscriptionStatus,
}

/// RU: Страница подписок. EN: Subscription list page.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubscriptionPageData {
    /// RU: Список подписок. EN: Subscription collection.
    pub subscription: Vec<Subscription>,
}
