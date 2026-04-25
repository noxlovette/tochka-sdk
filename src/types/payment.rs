use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

/// RU: Варианты создания платежа. EN: Payment creation paths.
pub enum PaymentPath {
    /// RU: Обычная ссылка на оплату. EN: Standard payment link.
    Standard,
    /// RU: Создание с чеком. EN: Payment with receipt.
    WithReceipt,
}

impl PaymentPath {
    /// RU: Вернуть строковый путь для API. EN: Return API path as string.
    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentPath::Standard => "payments",
            PaymentPath::WithReceipt => "payments_with_receipt",
        }
    }
}

/// RU: Фискальный признак способа оплаты (полная оплата/предоплата). EN: Fiscal payment method flag.
#[derive(Serialize, Debug, Deserialize, EnumString, Display, Clone)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    FullPayment,
    FullPrepayment,
}

/// RU: Статус платежной операции. EN: Payment status.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    Created,
    Approved,
    #[serde(rename = "ON-REFUND")]
    OnRefund,
    Refunded,
    Expired,
    RefundedPartially,
    Authorized,
    WaitFullPayment,
}

/// RU: Способ оплаты клиента. EN: Payment mode.
#[derive(Deserialize, Serialize, Debug, EnumString, Display, PartialEq, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PaymentMode {
    Sbp,
    Card,
    Tinkoff,
    Dolyame,
}

/// RU: Признак предмета расчёта. EN: Payment object type.
#[derive(Deserialize, Serialize, Debug, EnumString, Display, Clone)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PaymentObject {
    Goods,
    Service,
    Work,
}
use crate::{ReceiptClient, ReceiptItem, Supplier, TaxSystemCode};
use chrono::{DateTime, NaiveDate, Utc};
use validator::Validate;

/// RU: Параметры фильтрации списка платежей. EN: Query params for payments list.
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentListQuery {
    /// Уникальный код клиента
    pub customer_code: Option<String>,
    /// Начало периода создания операций
    ///
    /// 2020-01-20
    pub from_date: Option<NaiveDate>,
    /// Конец периода создания операций
    ///
    /// 2020-01-20
    pub to_date: Option<NaiveDate>,
    /// Номер страницы
    pub page: Option<u32>,
    /// Количество записей на странице
    pub per_page: Option<u32>,
    /// Статус операции
    pub status: Option<PaymentStatus>,
}

impl PaymentListQuery {
    pub fn new(customer_code: Option<String>) -> Self {
        Self {
            customer_code: customer_code,
            ..Default::default()
        }
    }

    pub fn from_date(mut self, fd: impl Into<NaiveDate>) -> Self {
        self.from_date = Some(fd.into());
        self
    }

    pub fn to_date(mut self, td: impl Into<NaiveDate>) -> Self {
        self.to_date = Some(td.into());
        self
    }

    pub fn page(mut self, v: u32) -> Self {
        self.page = Some(v);
        self
    }

    pub fn status(mut self, v: PaymentStatus) -> Self {
        self.status = Some(v);
        self
    }
}

/// RU: Операция интернет-эквайринга. EN: Acquiring payment operation.
#[derive(Deserialize, Validate, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaymentOperation {
    /// RU: Уникальный код клиента (в ответах GET). EN: Customer code (present in GET responses).
    #[validate(length(equal = 9))]
    pub customer_code: Option<String>,
    /// RU: Система налогообложения. EN: Tax system code.
    pub tax_system_code: Option<TaxSystemCode>,
    /// RU: Тип оплаты при проведённой операции. EN: Payment type when payment is processed.
    pub payment_type: Option<PaymentMode>,
    /// RU: Идентификатор платежа в процессинге/СБП. EN: Processor/SBP payment ID.
    pub payment_id: Option<String>,
    /// RU: Идентификатор транзакции СБП (для возврата). EN: SBP transaction ID (for refund).
    #[serde(rename = "transactionId")]
    pub transaction_id: Option<Uuid>,
    /// RU: Дата/время создания (ISO8601). EN: Creation timestamp (ISO8601).
    pub created_at: Option<DateTime<Utc>>,
    /// RU: Разрешённые способы оплаты. EN: Allowed payment modes.
    pub payment_mode: Option<Vec<PaymentMode>>,
    /// RU: URL после успешной оплаты. EN: Redirect URL on success.
    pub redirect_url: Option<String>,
    /// RU: URL после неуспешной оплаты. EN: Redirect URL on failure.
    pub fail_redirect_url: Option<String>,
    /// RU: Данные покупателя для чека. EN: Receipt client data.
    #[serde(rename = "Client")]
    pub client: Option<ReceiptClient>,
    /// RU: Список товаров. EN: Order items.
    pub items: Option<Vec<ReceiptItem>>,
    /// RU: Назначение платежа. EN: Payment purpose.
    pub purpose: Option<String>,
    /// RU: Сумма платежа. EN: Payment amount.
    pub amount: f64,
    /// RU: Статус платежа. EN: Payment status.
    pub status: PaymentStatus,
    /// RU: Идентификатор операции. EN: Operation ID.
    pub operation_id: Uuid,
    /// RU: Ссылка на оплату. EN: Payment link.
    pub payment_link: String,
    /// RU: Идентификатор торговой точки. EN: Merchant ID.
    pub merchant_id: Option<String>,
    /// RU: Идентификатор покупателя (UUID). EN: Consumer ID (UUID).
    pub consumer_id: Option<Uuid>,
    /// RU: Связанные операции. EN: Related operations.
    #[serde(rename = "Order")]
    pub order: Option<Vec<Order>>,
    /// RU: Данные поставщика. EN: Supplier data.
    #[serde(rename = "Supplier")]
    pub supplier: Option<Supplier>,
    /// RU: Признак двухэтапной оплаты. EN: Two-step payment flag.
    pub pre_authorization: Option<bool>,
    /// RU: Время оплаты. EN: Payment time.
    pub paid_at: Option<String>,
    /// RU: Внутренний номер заказа. EN: Payment link identifier.
    #[validate(length(min = 1, max = 45))]
    pub payment_link_id: Option<String>,
    /// RU: Сохранить карту. EN: Save card for future use.
    pub save_card: Option<bool>,
    /// RU: TTL ссылки в минутах. EN: Link TTL in minutes.
    pub ttl: Option<i64>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// Идентификатор платежа
    pub order_id: String,

    /// Тип операции
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Сумма операции
    pub amount: f64,

    /// Время операции
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug, EnumString, Display, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    Refund,
    Approval,
    Authorized,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentPayload {
    pub amount: f64,
    pub consumer_id: Option<String>,
    pub customer_code: Option<String>,
    pub fail_redirect_url: Option<String>,
    pub merchant_id: Option<String>,
    pub payment_link_id: Option<String>,
    pub payment_mode: Vec<PaymentMode>,
    pub pre_authorization: Option<bool>,
    pub purpose: String,
    pub redirect_url: Option<String>,
    pub save_card: Option<bool>,
    pub ttl: Option<i64>,
}

impl CreatePaymentPayload {
    pub fn new(amount: f64, customer_code: Option<String>, purpose: impl Into<String>) -> Self {
        Self {
            amount,
            customer_code: customer_code,
            purpose: purpose.into(),
            consumer_id: None,
            fail_redirect_url: None,
            merchant_id: None,
            payment_link_id: None,
            payment_mode: Vec::new(),
            pre_authorization: None,
            redirect_url: None,
            save_card: None,
            ttl: Some(10800),
        }
    }

    pub fn fail_redirect_url(mut self, fail_redirect_url: impl Into<String>) -> Self {
        self.fail_redirect_url = Some(fail_redirect_url.into());
        self
    }

    pub fn redirect_url(mut self, redirect_url: impl Into<String>) -> Self {
        self.redirect_url = Some(redirect_url.into());
        self
    }

    pub fn save_card(mut self, save_card: bool) -> Self {
        self.save_card = Some(save_card);
        self
    }

    pub fn consumer_id(mut self, id: impl Into<String>) -> Self {
        self.consumer_id = Some(id.into());
        self
    }

    pub fn merchant_id(mut self, id: impl Into<String>) -> Self {
        self.merchant_id = Some(id.into());
        self
    }

    pub fn payment_modes<I>(mut self, modes: I) -> Self
    where
        I: IntoIterator<Item = PaymentMode>,
    {
        self.payment_mode.extend(modes);
        self
    }

    pub fn payment_link_id(mut self, id: impl Into<String>) -> Self {
        self.payment_link_id = Some(id.into());
        self
    }

    pub fn pre_authorization(mut self, pa: bool) -> Self {
        self.pre_authorization = Some(pa);
        self
    }

    pub fn ttl(mut self, value: i64) -> Self {
        self.ttl = Some(value);
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentPageData {
    pub operation: Vec<PaymentOperation>,
}
