use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;
use validator::Validate;

// ---------- QR Codes ----------

/// RU: Тип QR-кода СБП. EN: SBP QR code type.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum QrCodeType {
    /// RU: Статический QR-код (многоразовый, сумма вводится при сканировании). EN: Static QR — reusable, amount set at scan time.
    Static,
    /// RU: Динамический QR-код (одноразовый, фиксированная сумма). EN: Dynamic QR — single-use, fixed amount.
    Dynamic,
}

/// RU: Статус QR-кода СБП. EN: SBP QR code status.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QrCodeStatus {
    /// RU: Активен. EN: Active.
    Active,
    /// RU: Оплачен. EN: Paid.
    Paid,
    /// RU: Неактивен. EN: Inactive.
    Inactive,
    /// RU: Истёк. EN: Expired.
    Expired,
}

/// RU: Статус платёжной операции СБП. EN: SBP payment operation status.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SbpOperationStatus {
    /// RU: Успешно завершена. EN: Accepted/completed.
    #[serde(rename = "ACSC")]
    Acsc,
    /// RU: Отклонена. EN: Rejected.
    #[serde(rename = "RJCT")]
    Rjct,
    /// RU: В обработке. EN: Pending.
    #[serde(rename = "PDNG")]
    Pdng,
}

/// RU: QR-код СБП. EN: SBP QR code.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QrCode {
    /// RU: Идентификатор QR-кода. EN: QR code ID.
    pub qrc_id: String,
    /// RU: Тип QR-кода. EN: QR code type.
    pub qrc_type: QrCodeType,
    /// RU: Идентификатор мерчанта. EN: Merchant ID.
    pub merchant_id: String,
    /// RU: Идентификатор счёта. EN: Account ID.
    pub account_id: String,
    /// RU: Статус QR-кода. EN: QR code status.
    pub status: QrCodeStatus,
    /// RU: Данные QR (ссылка или строка). EN: QR payload (URL or raw string).
    pub payload: Option<String>,
    /// RU: Сумма в копейках (только для динамических). EN: Amount in kopeks (dynamic only).
    pub amount: Option<u64>,
    /// RU: Назначение платежа. EN: Payment purpose.
    pub purpose: Option<String>,
    /// RU: Дата создания. EN: Creation timestamp.
    pub created_at: Option<DateTime<Utc>>,
}

/// RU: Запрос регистрации QR-кода. EN: Register QR code request payload.
#[derive(Debug, Clone, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterQrCodePayload {
    /// RU: Тип QR-кода. EN: QR code type.
    pub qrc_type: QrCodeType,
    /// RU: Сумма в копейках (обязательна для динамических). EN: Amount in kopeks (required for dynamic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    /// RU: Назначение платежа (до 140 символов). EN: Payment purpose (up to 140 chars).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 140))]
    pub purpose: Option<String>,
    /// RU: Время жизни QR-кода в минутах (только для динамических). EN: TTL in minutes (dynamic only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u32>,
}

impl RegisterQrCodePayload {
    /// RU: Создать статический QR-код. EN: Build a static QR code payload.
    pub fn static_code() -> Self {
        Self {
            qrc_type: QrCodeType::Static,
            amount: None,
            purpose: None,
            ttl: None,
        }
    }

    /// RU: Создать динамический QR-код с суммой. EN: Build a dynamic QR code with fixed amount.
    pub fn dynamic(amount: u64) -> Self {
        Self {
            qrc_type: QrCodeType::Dynamic,
            amount: Some(amount),
            purpose: None,
            ttl: None,
        }
    }

    /// RU: Установить назначение платежа. EN: Set payment purpose.
    pub fn purpose(mut self, purpose: impl Into<String>) -> Self {
        self.purpose = Some(purpose.into());
        self
    }

    /// RU: Установить TTL в минутах. EN: Set TTL in minutes.
    pub fn ttl(mut self, minutes: u32) -> Self {
        self.ttl = Some(minutes);
        self
    }
}

/// RU: Страница QR-кодов. EN: QR code list page.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QrCodePageData {
    /// RU: Список QR-кодов. EN: QR code collection.
    pub qr_code: Vec<QrCode>,
}

/// RU: Статус оплаты по QR-коду. EN: Payment operation status for a QR code.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QrCodePaymentStatus {
    /// RU: Идентификатор QR-кода. EN: QR code ID.
    pub qrc_id: String,
    /// RU: Сумма операции в копейках. EN: Operation amount in kopeks.
    pub amount: Option<u64>,
    /// RU: Статус. EN: Status.
    pub status: SbpOperationStatus,
    /// RU: Идентификатор транзакции СБП. EN: SBP transaction ID.
    pub trx_id: Option<String>,
    /// RU: Время операции. EN: Operation time.
    pub operation_time: Option<DateTime<Utc>>,
}

// ---------- Merchants (TSP) ----------

/// RU: Статус ТСП в СБП. EN: SBP merchant (TSP) status.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum MerchantStatus {
    /// RU: Активен. EN: Active.
    Active,
    /// RU: Приостановлен. EN: Suspended.
    Suspended,
    /// RU: Ликвидирован. EN: Terminated.
    Terminated,
}

/// RU: ТСП (торгово-сервисное предприятие) в СБП. EN: SBP merchant (TSP).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SbpMerchant {
    /// RU: Идентификатор мерчанта. EN: Merchant ID.
    pub merchant_id: String,
    /// RU: Идентификатор юрлица. EN: Legal entity ID.
    pub legal_id: String,
    /// RU: Наименование мерчанта. EN: Merchant name.
    pub merchant_name: Option<String>,
    /// RU: Статус. EN: Status.
    pub status: Option<MerchantStatus>,
    /// RU: MCC код. EN: MCC code.
    pub mcc: Option<String>,
    /// RU: Адрес торговой точки. EN: Merchant address.
    pub address: Option<String>,
}

/// RU: Запрос регистрации ТСП. EN: Register merchant request payload.
#[derive(Debug, Clone, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterMerchantPayload {
    /// RU: Наименование мерчанта (до 256 символов). EN: Merchant name (up to 256 chars).
    #[validate(length(min = 1, max = 256))]
    pub merchant_name: String,
    /// RU: MCC код (4 символа). EN: MCC code (4 digits).
    #[validate(length(equal = 4))]
    pub mcc: String,
    /// RU: Адрес торговой точки. EN: Merchant address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl RegisterMerchantPayload {
    /// RU: Создать запрос регистрации ТСП. EN: Build a register-merchant request.
    pub fn new(merchant_name: impl Into<String>, mcc: impl Into<String>) -> Self {
        Self {
            merchant_name: merchant_name.into(),
            mcc: mcc.into(),
            address: None,
        }
    }

    /// RU: Установить адрес торговой точки. EN: Set merchant address.
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }
}

/// RU: Запрос изменения статуса ТСП. EN: Set merchant status payload.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMerchantStatusPayload {
    /// RU: Новый статус ТСП. EN: New merchant status.
    pub status: MerchantStatus,
}

/// RU: Страница мерчантов. EN: Merchant list page.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SbpMerchantPageData {
    /// RU: Список ТСП. EN: Merchant collection.
    pub merchant: Vec<SbpMerchant>,
}

// ---------- SBP Payments & Refunds ----------

/// RU: Платёж через СБП. EN: SBP payment entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SbpPayment {
    /// RU: Идентификатор транзакции. EN: SBP transaction ID.
    pub trx_id: String,
    /// RU: Идентификатор QR-кода. EN: QR code ID.
    pub qrc_id: Option<String>,
    /// RU: Сумма в копейках. EN: Amount in kopeks.
    pub amount: u64,
    /// RU: Статус операции. EN: Operation status.
    pub status: SbpOperationStatus,
    /// RU: Время операции. EN: Operation time.
    pub operation_time: Option<DateTime<Utc>>,
    /// RU: Наименование плательщика. EN: Payer name.
    pub payer_name: Option<String>,
}

/// RU: Параметры запроса платежей СБП. EN: SBP payments list query params.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SbpPaymentsQuery {
    /// RU: Фильтр по идентификатору QR-кода. EN: Filter by QR code ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qrc_id: Option<String>,
    /// RU: Начало периода (YYYY-MM-DD). EN: Period start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,
    /// RU: Конец периода (YYYY-MM-DD). EN: Period end date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,
    /// RU: Номер страницы. EN: Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// RU: Записей на странице. EN: Items per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

/// RU: Страница платежей СБП. EN: SBP payment list page.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SbpPaymentPageData {
    /// RU: Список платежей. EN: Payment collection.
    pub payment: Vec<SbpPayment>,
}

/// RU: Возврат через СБП. EN: SBP refund.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SbpRefund {
    /// RU: Идентификатор запроса возврата. EN: Refund request ID.
    pub request_id: Uuid,
    /// RU: Идентификатор исходной транзакции. EN: Original SBP transaction ID.
    pub trx_id: String,
    /// RU: Сумма возврата в копейках. EN: Refund amount in kopeks.
    pub amount: u64,
    /// RU: Статус. EN: Status.
    pub status: Option<SbpOperationStatus>,
    /// RU: Время операции. EN: Operation time.
    pub operation_time: Option<DateTime<Utc>>,
}

/// RU: Запрос на возврат через СБП. EN: SBP refund request payload.
#[derive(Debug, Clone, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SbpRefundPayload {
    /// RU: Идентификатор исходной транзакции. EN: Original SBP transaction ID.
    pub trx_id: String,
    /// RU: Сумма возврата в копейках (минимум 1). EN: Refund amount in kopeks (min 1).
    #[validate(range(min = 1))]
    pub amount: u64,
}

impl SbpRefundPayload {
    /// RU: Создать запрос возврата. EN: Build a refund request.
    pub fn new(trx_id: impl Into<String>, amount: u64) -> Self {
        Self {
            trx_id: trx_id.into(),
            amount,
        }
    }
}
