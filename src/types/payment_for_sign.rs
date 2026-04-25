use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;
use validator::Validate;

/// RU: Статус платёжного поручения. EN: Payment-for-sign status.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, EnumString, Display)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentForSignStatus {
    /// RU: Создан, ожидает подписи. EN: Created, awaiting signature.
    New,
    /// RU: Подписан. EN: Signed by the account holder.
    Signed,
    /// RU: Отправлен в банк. EN: Sent to the bank.
    Sent,
    /// RU: Исполнен. EN: Executed.
    Executed,
    /// RU: Ошибка при исполнении. EN: Execution error.
    Error,
    /// RU: Отозван. EN: Cancelled.
    Cancelled,
}

/// RU: Реквизиты получателя платёжного поручения. EN: Recipient bank details.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRecipient {
    /// RU: Наименование получателя (до 160 символов). EN: Recipient name (up to 160 chars).
    #[validate(length(min = 1, max = 160))]
    pub name: String,
    /// RU: ИНН получателя (10–12 цифр). EN: Recipient INN (10–12 digits).
    #[validate(length(min = 10, max = 12))]
    pub inn: String,
    /// RU: КПП получателя (только для юрлиц). EN: Recipient KPP (legal entities only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpp: Option<String>,
    /// RU: Расчётный счёт получателя (20 цифр). EN: Recipient account number (20 digits).
    #[validate(length(equal = 20))]
    pub account: String,
    /// RU: БИК банка получателя (9 цифр). EN: Recipient bank BIK (9 digits).
    #[validate(length(equal = 9))]
    pub bik: String,
    /// RU: Наименование банка получателя. EN: Recipient bank name.
    pub bank_name: String,
    /// RU: Корреспондентский счёт банка (20 цифр). EN: Bank correspondent account (20 digits).
    #[validate(length(equal = 20))]
    pub bank_account: String,
}

impl PaymentRecipient {
    /// RU: Создать реквизиты получателя. EN: Build recipient bank details.
    pub fn new(
        name: impl Into<String>,
        inn: impl Into<String>,
        account: impl Into<String>,
        bik: impl Into<String>,
        bank_name: impl Into<String>,
        bank_account: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            inn: inn.into(),
            kpp: None,
            account: account.into(),
            bik: bik.into(),
            bank_name: bank_name.into(),
            bank_account: bank_account.into(),
        }
    }

    /// RU: Установить КПП. EN: Set KPP.
    pub fn kpp(mut self, kpp: impl Into<String>) -> Self {
        self.kpp = Some(kpp.into());
        self
    }
}

/// RU: Платёжное поручение (для подписи). EN: Bank payment order (for sign).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentForSign {
    /// RU: Идентификатор запроса. EN: Request ID.
    pub request_id: Uuid,
    /// RU: Уникальный код клиента. EN: Customer code.
    pub customer_code: String,
    /// RU: Идентификатор счёта списания. EN: Source account ID.
    pub account_id: String,
    /// RU: Сумма в копейках. EN: Amount in kopeks.
    pub amount: u64,
    /// RU: Назначение платежа. EN: Payment purpose.
    pub payment_purpose: Option<String>,
    /// RU: Статус. EN: Status.
    pub status: PaymentForSignStatus,
    /// RU: Реквизиты получателя. EN: Recipient bank details.
    pub recipient: Option<PaymentRecipient>,
    /// RU: Дата создания. EN: Creation timestamp.
    pub created_at: Option<DateTime<Utc>>,
    /// RU: Дата проведения. EN: Scheduled payment date.
    pub payment_date: Option<NaiveDate>,
}

/// RU: Запрос на создание платёжного поручения. EN: Create payment-for-sign payload.
#[derive(Debug, Clone, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentForSignPayload {
    /// RU: Уникальный код клиента (9 символов). EN: Customer code (9 chars).
    #[validate(length(equal = 9))]
    pub customer_code: String,
    /// RU: Идентификатор счёта списания. EN: Source account ID.
    pub account_id: String,
    /// RU: Сумма в копейках. EN: Amount in kopeks.
    #[validate(range(min = 1))]
    pub amount: u64,
    /// RU: Назначение платежа (до 210 символов). EN: Payment purpose (up to 210 chars).
    #[validate(length(min = 1, max = 210))]
    pub payment_purpose: String,
    /// RU: Реквизиты получателя. EN: Recipient bank details.
    pub recipient: PaymentRecipient,
    /// RU: Дата проведения (по умолчанию — сегодня). EN: Payment date (defaults to today).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<NaiveDate>,
    /// RU: Очерёдность платежа (5 — стандартная). EN: Payment priority (5 is standard).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
}

impl CreatePaymentForSignPayload {
    /// RU: Создать запрос платёжного поручения. EN: Build a create-payment-for-sign request.
    pub fn new(
        customer_code: impl Into<String>,
        account_id: impl Into<String>,
        amount: u64,
        payment_purpose: impl Into<String>,
        recipient: PaymentRecipient,
    ) -> Self {
        Self {
            customer_code: customer_code.into(),
            account_id: account_id.into(),
            amount,
            payment_purpose: payment_purpose.into(),
            recipient,
            payment_date: None,
            priority: None,
        }
    }

    /// RU: Установить дату проведения. EN: Set payment execution date.
    pub fn payment_date(mut self, date: NaiveDate) -> Self {
        self.payment_date = Some(date);
        self
    }

    /// RU: Установить очерёдность платежа. EN: Set payment priority.
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority);
        self
    }
}

/// RU: Параметры запроса списка платёжных поручений. EN: Query params for payment-for-sign list.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentForSignQuery {
    /// RU: Код клиента. EN: Customer code filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_code: Option<String>,
    /// RU: Фильтр по статусу. EN: Status filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentForSignStatus>,
    /// RU: Начало периода (YYYY-MM-DD). EN: Period start date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<NaiveDate>,
    /// RU: Конец периода (YYYY-MM-DD). EN: Period end date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<NaiveDate>,
    /// RU: Номер страницы. EN: Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// RU: Записей на странице. EN: Items per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

impl PaymentForSignQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn customer_code(mut self, code: impl Into<String>) -> Self {
        self.customer_code = Some(code.into());
        self
    }

    pub fn status(mut self, status: PaymentForSignStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn from_date(mut self, date: NaiveDate) -> Self {
        self.from_date = Some(date);
        self
    }

    pub fn to_date(mut self, date: NaiveDate) -> Self {
        self.to_date = Some(date);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
}

/// RU: Страница платёжных поручений. EN: Payment-for-sign list page.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentForSignPageData {
    /// RU: Список платёжных поручений. EN: Payment orders collection.
    pub payment: Vec<PaymentForSign>,
}
