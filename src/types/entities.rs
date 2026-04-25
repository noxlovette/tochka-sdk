use crate::{validate_phone, validate_tax_code};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use validator::Validate;

/// RU: Данные поставщика (для чеков). EN: Supplier information for receipts.
#[derive(Deserialize, Validate, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Supplier {
    /// RU: Телефон. EN: Phone number.
    #[validate(custom(function = "validate_phone"))]
    pub phone: String,

    /// RU: Наименование. EN: Name.
    #[validate(length(min = 1))]
    pub name: String,

    /// RU: ИНН. EN: Tax code.
    #[validate(custom(function = "validate_tax_code"))]
    pub tax_code: String,
}

/// RU: Информация о контрагенте. EN: Counterparty information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contractor {
    pub inn: Option<String>,
    pub kpp: Option<String>,
    pub name: Option<String>,
}

/// RU: Банк контрагента. EN: Counterparty bank details.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractorBank {
    pub account_identification: Option<String>,
    pub identification: Option<String>,
    pub name: Option<String>,
    pub scheme_name: FinancialInstitutionIdentification,
}

/// RU: Схемы идентификации банков. EN: Bank identification schemes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FinancialInstitutionIdentification {
    #[serde(rename = "RU.CBR.BICFI")]
    RuCbrBicfi,

    #[serde(rename = "RU.CBR.BIK")]
    RuCbrBik,
}

/// RU: Модель клиента из API. EN: Customer model from API.
#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    /// RU: Уникальный код клиента. EN: Unique customer code.
    pub customer_code: String,
    /// RU: Тип клиента (физ/юр). EN: Customer type (personal/business).
    pub customer_type: ExternalType,
    /// RU: Резидент РФ. EN: Resident flag.
    pub is_resident: bool,
    /// RU: ИНН. EN: Tax code (INN).
    #[validate(custom(function = validate_tax_code))]
    pub tax_code: Option<String>,
    /// RU: Полное имя/название. EN: Full name.
    pub full_name: String,
    /// RU: Короткое имя. EN: Short name.
    pub short_name: Option<String>,
    /// RU: КПП. EN: KPP.
    pub kpp: Option<String>,
    /// RU: ОГРН/ОГРНИП. EN: OGRN/OGRNIP.
    pub customer_ogrn: Option<String>,
}

/// RU: Тип клиента. EN: Customer type.
#[derive(Debug, Clone, Deserialize, Serialize, EnumString, Display, PartialEq)]
pub enum ExternalType {
    Business,
    Personal,
}

/// RU: Страница клиентов. EN: Customer page payload.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CustomerPageData {
    pub customer: Vec<Customer>,
}
