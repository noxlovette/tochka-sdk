use serde::{Deserialize, Serialize};

/// RU: Версия API. EN: API version.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ApiVersion {
    /// RU: Текущая версия v1.0. EN: Current v1.0.
    #[serde(rename = "v1.0")]
    V1_0,
}

impl ApiVersion {
    /// RU: Вернуть строковое представление. EN: Return string value.
    pub fn as_str(&self) -> &'static str {
        match self {
            ApiVersion::V1_0 => "v1.0",
        }
    }
}

impl Default for ApiVersion {
    fn default() -> Self {
        ApiVersion::V1_0
    }
}
