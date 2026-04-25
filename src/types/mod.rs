mod account;
mod balance;
mod consent;
mod entities;
mod invoice;
mod payment;
mod payment_for_sign;
mod receipt;
mod refund;
mod registry;
mod retailer;
mod sbp;
mod service;
mod statements;
mod subscription;
mod tax;
mod transactions;
mod version;
mod webhooks;

pub use account::*;
pub use balance::*;
pub use consent::*;
pub use entities::*;
pub use invoice::*;
pub use payment::*;
pub use payment_for_sign::*;
pub use receipt::*;
pub use refund::*;
pub use registry::*;
pub use retailer::*;
pub use sbp::*;
pub use service::*;
pub use statements::*;
pub use subscription::*;
pub use tax::*;
pub use transactions::*;
pub use version::*;
pub use webhooks::*;

use serde::{Deserialize, Serialize};

/// RU: Обёртка с данными, ссылками и метой. EN: Standard API response wrapper.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data<T> {
    /// RU: Основная информация. EN: Primary payload.
    pub data: T,
    /// RU: Ссылки. EN: Links.
    pub links: Link,
    /// RU: Метаданные. EN: Metadata.
    pub meta: Meta,
}

/// RU: Ссылка self. EN: Self link.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Link {
    #[serde(rename = "self")]
    pub this: String,
}

/// RU: Ссылки пагинации. EN: Pagination links.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedLink {
    /// RU: Ссылка на текущую страницу. EN: Self link.
    #[serde(rename = "self")]
    pub this: String,

    /// RU: Первая страница. EN: First page.
    pub first: Option<String>,
    /// RU: Предыдущая страница. EN: Previous page.
    pub prev: Option<String>,
    /// RU: Следующая страница. EN: Next page.
    pub next: Option<String>,
    /// RU: Последняя страница. EN: Last page.
    pub last: Option<String>,
}

/// RU: Пагинированный ответ. EN: Paginated response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaginatedResponse<T> {
    /// RU: Ссылки пагинации. EN: Pagination links.
    pub links: PaginatedLink,
    /// RU: Метаданные. EN: Metadata.
    pub meta: Meta,
    /// RU: Данные страницы. EN: Page data.
    pub data: T,
}

/// RU: Метаданные ответа. EN: Response metadata.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// RU: Количество страниц. EN: Total pages.
    pub total_pages: u64,
}

/// RU: Обёртка данных для POST. EN: Top-level payload wrapper.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PayloadWrapper<T> {
    /// RU: Основные данные. EN: Wrapped data.
    pub data: T,
}

impl<T> PayloadWrapper<T> {
    /// RU: Обернуть данные. EN: Wrap data.
    pub fn wrap(data: T) -> Self {
        Self { data }
    }
}

/// RU: Простая структура результата (Delete). EN: Simple result payload (Delete).
#[derive(Serialize, Deserialize, Debug)]
pub struct ResultBody {
    /// RU: Успешность операции. EN: Operation success flag.
    pub result: bool,
}
