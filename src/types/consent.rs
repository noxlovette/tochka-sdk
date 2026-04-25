use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// RU: Тип разрешения (scope) для согласия. EN: Permission scope for a consent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalConsentTypeEnum {
    #[serde(rename = "ReadAccountsBasic")]
    ReadAccountsBasic,
    #[serde(rename = "ReadAccountsDetail")]
    ReadAccountsDetail,
    #[serde(rename = "ReadBalances")]
    ReadBalances,
    #[serde(rename = "ReadStatements")]
    ReadStatements,
    #[serde(rename = "ReadTransactionsBasic")]
    ReadTransactionsBasic,
    #[serde(rename = "ReadTransactionsCredits")]
    ReadTransactionsCredits,
    #[serde(rename = "ReadTransactionsDebits")]
    ReadTransactionsDebits,
    #[serde(rename = "ReadTransactionsDetail")]
    ReadTransactionsDetail,
    #[serde(rename = "ReadCustomerData")]
    ReadCustomerData,
    #[serde(rename = "ReadSBPData")]
    ReadSbpData,
    #[serde(rename = "EditSBPData")]
    EditSbpData,
    #[serde(rename = "CreatePaymentForSign")]
    CreatePaymentForSign,
    #[serde(rename = "CreatePaymentOrder")]
    CreatePaymentOrder,
    #[serde(rename = "ReadAcquiringData")]
    ReadAcquiringData,
    #[serde(rename = "MakeAcquiringOperation")]
    MakeAcquiringOperation,
    #[serde(rename = "ManageInvoiceData")]
    ManageInvoiceData,
    #[serde(rename = "ManageWebhookData")]
    ManageWebhookData,
    #[serde(rename = "MakeCustomer")]
    MakeCustomer,
    #[serde(rename = "ManageGuarantee")]
    ManageGuarantee,
}

/// RU: Статус согласия. EN: Consent status.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ConsentStatus {
    /// RU: Авторизовано. EN: Authorised.
    Authorised,
    /// RU: Ожидает авторизации. EN: Awaiting authorisation.
    AwaitingAuthorisation,
    /// RU: Отклонено. EN: Rejected.
    Rejected,
    /// RU: Отозвано. EN: Revoked.
    Revoked,
}

/// RU: Согласие (разрешение) для доступа к данным. EN: Consent for data access.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consent {
    /// RU: Идентификатор согласия. EN: Consent ID.
    pub consent_id: String,
    /// RU: Статус. EN: Status.
    pub status: ConsentStatus,
    /// RU: Время создания. EN: Creation timestamp.
    pub creation_date_time: DateTime<Utc>,
    /// RU: Время истечения. EN: Expiration timestamp.
    pub expiration_date_time: Option<DateTime<Utc>>,
    /// RU: Список разрешений. EN: Granted permissions.
    pub permissions: Vec<ExternalConsentTypeEnum>,
    /// RU: URL для авторизации (только при создании). EN: Authorization URL (creation response only).
    pub authorize_url: Option<String>,
}

/// RU: Запрос создания согласия. EN: Create consent request payload.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConsentPayload {
    /// RU: Список запрашиваемых разрешений. EN: Requested permission scopes.
    pub permissions: Vec<ExternalConsentTypeEnum>,
    /// RU: Время истечения согласия (ISO 8601). EN: Consent expiration time (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_time: Option<DateTime<Utc>>,
}

impl CreateConsentPayload {
    /// RU: Создать запрос согласия с заданным набором разрешений. EN: Build a consent request for the given permissions.
    pub fn new(permissions: Vec<ExternalConsentTypeEnum>) -> Self {
        Self {
            permissions,
            expiration_date_time: None,
        }
    }

    /// RU: Установить дату истечения. EN: Set expiration time.
    pub fn expires_at(mut self, dt: DateTime<Utc>) -> Self {
        self.expiration_date_time = Some(dt);
        self
    }
}

/// RU: Страница согласий. EN: Consent list page.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsentPageData {
    /// RU: Список согласий. EN: Consent collection.
    pub consent: Vec<Consent>,
}
