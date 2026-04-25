use crate::PaymentMode;
use serde::{Deserialize, Serialize};

/// RU: Информация о ретейлере эквайринга. EN: Acquiring retailer info.
#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Retailer {
    /// RU: Статус регистрации. EN: Registration status.
    pub status: RetailerStatus,
    /// RU: Готов к работе. EN: Ready to operate flag.
    pub is_active: bool,
    /// RU: MCC код. EN: MCC code.
    pub mcc: String,
    /// RU: Комиссия. EN: Commission rate.
    pub rate: f64,
    /// RU: Наименование. EN: Name.
    pub name: String,
    /// RU: Сайт. EN: Website URL.
    pub url: String,
    /// RU: Идентификатор мерчанта. EN: Merchant ID.
    pub merchant_id: String,
    /// RU: Идентификатор терминала. EN: Terminal ID.
    pub terminal_id: String,
    /// RU: Доступные способы оплаты. EN: Supported payment modes.
    pub payment_modes: Vec<PaymentMode>,
    /// RU: Подключённая касса. EN: Cashbox name.
    pub cashbox: String,
}

/// RU: Статусы ретейлера. EN: Retailer statuses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RetailerStatus {
    #[serde(rename = "NEW")]
    NEW,
    #[serde(rename = "ADDRESS_DADATA")]
    ADDRESSDADATA,
    #[serde(rename = "OPEN_ACCOUNT")]
    OPENACCOUNT,
    #[serde(rename = "TWPG_SENDED")]
    TWPGSENDED,
    #[serde(rename = "RETAILER_CREATED")]
    RETAILERCREATED,
    #[serde(rename = "TERMINAL_CREATED")]
    TERMINALCREATED,
    #[serde(rename = "FILE_SENT")]
    FILESENT,
    #[serde(rename = "REG")]
    REG,
    #[serde(rename = "CLOSE")]
    CLOSE,
}

/// RU: Страница ретейлеров. EN: Retailer list page.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RetailerPageData {
    pub retailer: Vec<Retailer>,
}

/// RU: Параметры запроса ретейлера. EN: Retailer query parameters.
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct RetailerQuery {
    pub customer_code: String,
}

impl RetailerQuery {
    /// RU: Создать запрос по коду клиента. EN: Build query by customer code.
    pub fn new(customer_code: impl Into<String>) -> Self {
        Self {
            customer_code: customer_code.into(),
        }
    }
}
