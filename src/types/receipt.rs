use crate::validate_phone;
use crate::{PaymentMethod, PaymentObject, Supplier, VatType};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate, Serialize, Debug, Clone)]
pub struct ReceiptClient {
    /// Для юрлица — название организации, для ИП и физического лица — ФИО
    #[validate(length(min = 1))]
    pub name: Option<String>,
    #[validate(email)]
    /// Email покупателя, на который будет отправлен чек
    pub email: String,
    /// Телефон пользователя для отправки чека.
    #[validate(custom(function = "validate_phone"))]
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptItem {
    /// Ставка НДС
    pub vat_type: Option<VatType>,

    /// Название товара
    #[validate(length(min = 1, max = 256))]
    pub name: String,

    /// Цена за единицу товара
    pub amount: f64,

    /// Количество товара
    pub quantity: f64,

    /// Тип оплаты
    pub payment_method: Option<PaymentMethod>,

    /// Признак предмета расчёта
    pub payment_object: Option<PaymentObject>,

    /// Елиница измерения
    ///
    /// По умолчанию – штуки
    pub measure: Option<Measure>,

    /// Данные поставщика
    #[serde(rename = "Supplier")]
    pub supplier: Option<Supplier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnitCode {
    #[serde(rename = "шт.")]
    Pieces,

    #[serde(rename = "тыс.шт.")]
    ThousandPieces,

    #[serde(rename = "компл.")]
    Set,

    #[serde(rename = "пар.")]
    Pair,

    #[serde(rename = "усл.ед.")]
    ServiceUnit,

    #[serde(rename = "упак.")]
    Package,

    #[serde(rename = "услуга.")]
    Service,

    #[serde(rename = "пач.")]
    Pack,

    #[serde(rename = "мин.")]
    Minute,

    #[serde(rename = "ч.")]
    Hour,

    #[serde(rename = "сут.")]
    Day,

    #[serde(rename = "г.")]
    Gram,

    #[serde(rename = "кг.")]
    Kilogram,

    #[serde(rename = "л.")]
    Liter,

    #[serde(rename = "м.")]
    Meter,

    #[serde(rename = "м2.")]
    SquareMeter,

    #[serde(rename = "м3.")]
    CubicMeter,

    #[serde(rename = "км.")]
    Kilometer,

    #[serde(rename = "га.")]
    Hectare,

    #[serde(rename = "кВт.")]
    Kilowatt,

    #[serde(rename = "кВт.ч.")]
    KilowattHour,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Measure {
    #[serde(rename = "г.")]
    Gram,

    #[serde(rename = "кг.")]
    Kilogram,

    #[serde(rename = "т.")]
    Ton,

    #[serde(rename = "см.")]
    Centimeter,

    #[serde(rename = "дм.")]
    Decimeter,

    #[serde(rename = "м.")]
    Meter,

    #[serde(rename = "см2.")]
    SquareCentimeter,

    #[serde(rename = "дм2.")]
    SquareDecimeter,

    #[serde(rename = "м2.")]
    SquareMeter,

    #[serde(rename = "мл.")]
    Milliliter,

    #[serde(rename = "л.")]
    Liter,

    /// Косяк Точки
    #[serde(rename = "м3")]
    CubicMeter,

    #[serde(rename = "кВт.ч.")]
    KilowattHour,

    #[serde(rename = "Гкал.")]
    Gigacalorie,

    #[serde(rename = "дн.")]
    Day,

    #[serde(rename = "ч.")]
    Hour,

    #[serde(rename = "мин.")]
    Minute,

    #[serde(rename = "сек.")]
    Second,

    #[serde(rename = "Кб.")]
    Kilobyte,

    #[serde(rename = "Мб.")]
    Megabyte,

    #[serde(rename = "Гб.")]
    Gigabyte,

    #[serde(rename = "Тб.")]
    Terabyte,

    #[serde(rename = "шт.")]
    Piece,
}
