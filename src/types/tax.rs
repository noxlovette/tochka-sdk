use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Debug, Clone, Display, EnumString, PartialEq)]
pub enum DateValue {
    Text(String),
    Number(i32),
}

/// TaxFieldsModel
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxFields {
    pub base: Option<String>,
    pub document_date: Option<DateValue>,
    pub document_number: Option<String>,
    pub field107: Option<String>,
    pub kbk: Option<String>,
    pub oktmo: Option<String>,
    pub originator_status: Option<String>,
    pub type_: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TaxSystemCode {
    Osn,
    UsnIncome,
    UsnIncomeOutcome,
    Esn,
    Patent,
    Envd,
}

#[derive(Deserialize, Serialize, EnumString, Display, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum VatType {
    None,
    Vat0,
    Vat5,
    Vat7,
    Vat10,
    Vat20,
    Vat105,
    Vat107,
    Vat110,
    Vat120,
}

#[derive(Deserialize, Serialize, EnumString, Display)]
#[serde(rename_all = "snake_case")]
pub enum NdsKind {
    Nds0,
    Nds5,
    Nds7,
    Nds10,
    Nds20,
    WithoutNds,
}
