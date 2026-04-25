use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// RU: Статус оплаты счёта. EN: Invoice payment status.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceStatus {
    /// RU: Создан, ожидает оплаты. EN: Created, awaiting payment.
    Created,
    /// RU: Оплачен. EN: Paid.
    Paid,
    /// RU: Отменён. EN: Cancelled.
    Cancelled,
    /// RU: Истёк срок действия. EN: Expired.
    Expired,
}

/// RU: Счёт на оплату. EN: Payment invoice.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    /// RU: Идентификатор документа. EN: Document ID.
    pub document_id: String,
    /// RU: Уникальный код клиента. EN: Customer code.
    pub customer_code: String,
    /// RU: Сумма счёта. EN: Invoice amount.
    pub amount: f64,
    /// RU: Назначение платежа. EN: Payment purpose.
    pub purpose: Option<String>,
    /// RU: Статус оплаты. EN: Payment status.
    pub status: InvoiceStatus,
    /// RU: Ссылка на оплату. EN: Payment URL.
    pub payment_url: Option<String>,
    /// RU: Дата создания. EN: Creation timestamp.
    pub created_at: Option<DateTime<Utc>>,
    /// RU: Срок действия счёта. EN: Invoice expiration date.
    pub expiration_date: Option<NaiveDate>,
}

/// RU: Запрос создания счёта на оплату. EN: Create invoice request payload.
#[derive(Debug, Clone, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvoicePayload {
    /// RU: Уникальный код клиента (9 символов). EN: Customer code (9 chars).
    #[validate(length(equal = 9))]
    pub customer_code: String,
    /// RU: Сумма счёта. EN: Invoice amount.
    #[validate(range(min = 0.01))]
    pub amount: f64,
    /// RU: Назначение платежа (до 210 символов). EN: Payment purpose (up to 210 chars).
    #[validate(length(min = 1, max = 210))]
    pub purpose: String,
    /// RU: Email покупателя. EN: Buyer email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// RU: Срок действия счёта. EN: Invoice expiration date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<NaiveDate>,
}

impl CreateInvoicePayload {
    /// RU: Создать запрос выставления счёта. EN: Build a create-invoice request.
    pub fn new(
        customer_code: impl Into<String>,
        amount: f64,
        purpose: impl Into<String>,
    ) -> Self {
        Self {
            customer_code: customer_code.into(),
            amount,
            purpose: purpose.into(),
            email: None,
            expiration_date: None,
        }
    }

    /// RU: Установить email покупателя. EN: Set buyer email.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// RU: Установить срок действия. EN: Set expiration date.
    pub fn expiration_date(mut self, date: NaiveDate) -> Self {
        self.expiration_date = Some(date);
        self
    }
}

/// RU: Запрос отправки счёта по email. EN: Send invoice to email request payload.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendDocumentEmailPayload {
    /// RU: Email получателя. EN: Recipient email.
    pub email: String,
}

impl SendDocumentEmailPayload {
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
        }
    }
}

/// RU: Страница счетов. EN: Invoice list page.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InvoicePageData {
    /// RU: Список счетов. EN: Invoice collection.
    pub invoice: Vec<Invoice>,
}

/// RU: Закрывающий документ (акт/счёт-фактура). EN: Closing document (act / invoice).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosingDocument {
    /// RU: Идентификатор документа. EN: Document ID.
    pub document_id: String,
    /// RU: Уникальный код клиента. EN: Customer code.
    pub customer_code: String,
    /// RU: Сумма. EN: Amount.
    pub amount: f64,
    /// RU: Назначение. EN: Purpose.
    pub purpose: Option<String>,
    /// RU: Дата создания. EN: Creation timestamp.
    pub created_at: Option<DateTime<Utc>>,
}

/// RU: Запрос создания закрывающего документа. EN: Create closing document request payload.
#[derive(Debug, Clone, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateClosingDocumentPayload {
    /// RU: Уникальный код клиента (9 символов). EN: Customer code (9 chars).
    #[validate(length(equal = 9))]
    pub customer_code: String,
    /// RU: Сумма. EN: Amount.
    #[validate(range(min = 0.01))]
    pub amount: f64,
    /// RU: Назначение (до 210 символов). EN: Purpose (up to 210 chars).
    #[validate(length(min = 1, max = 210))]
    pub purpose: String,
    /// RU: Email для отправки документа. EN: Email to send the document to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl CreateClosingDocumentPayload {
    /// RU: Создать запрос закрывающего документа. EN: Build a create-closing-document request.
    pub fn new(
        customer_code: impl Into<String>,
        amount: f64,
        purpose: impl Into<String>,
    ) -> Self {
        Self {
            customer_code: customer_code.into(),
            amount,
            purpose: purpose.into(),
            email: None,
        }
    }

    /// RU: Установить email. EN: Set email.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }
}

/// RU: Страница закрывающих документов. EN: Closing document list page.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClosingDocumentPageData {
    /// RU: Список закрывающих документов. EN: Closing document collection.
    pub closing_document: Vec<ClosingDocument>,
}
