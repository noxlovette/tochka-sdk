use crate::{CashAccount, Contractor, ContractorBank, CreditDebitIndicator, TaxFields};
use chrono::{DateTime, NaiveDate, Utc};
use codes_iso_4217::CurrencyCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// RU: Авторизованная транзакция. EN: Authorized transaction model.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// RU: Идентификатор счёта. EN: Account identifier.
    pub account_id: String,
    /// RU: Маскированный PAN. EN: Masked PAN.
    pub pan: String,
    /// RU: Дата и время операции (ISO8601). EN: Operation timestamp (ISO8601).
    pub date_time: DateTime<Utc>,
    #[serde(rename = "Amount")]
    /// RU: Оригинальная сумма и валюта. EN: Original amount and currency.
    pub amount: Amount,
    #[serde(rename = "AccountAmount")]
    /// RU: Сумма в валюте счёта. EN: Amount in account currency.
    pub account_amount: AccountAmount,
    #[serde(rename = "TerminalData")]
    /// RU: Данные терминала. EN: Terminal data.
    pub terminal_data: TerminalData,
}

/// RU: Сумма транзакции. EN: Transaction amount.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    /// RU: Сумма. EN: Amount.
    pub amount: f64,
    /// RU: Сумма в копейках (если передана). EN: Amount in minor units if provided.
    pub amount_nat: Option<u32>,
    /// RU: Валюта ISO 4217. EN: Currency ISO 4217.
    pub currency: CurrencyCode,
}

/// RU: Сумма в валюте счёта. EN: Amount in account currency.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountAmount {
    pub amount: f64,
    pub currency: CurrencyCode,
}

/// RU: Информация о терминале. EN: Terminal information.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TerminalData {
    pub city: Option<String>,
    pub location: Option<String>,
    pub owner: Option<String>,
}

/// RU: Страница транзакций. EN: Transaction page payload.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionPageData {
    /// RU: Список транзакций. EN: Transaction collection.
    pub transactions: Vec<Transaction>,
}

/// RU: Статус проводки. EN: Transaction status.
#[derive(Debug, Deserialize, Serialize)]
pub enum TransactionStatus {
    Booked,
    Pending,
}

/// RU: Транзакция в выписке. EN: Statement transaction entry.
#[derive(Validate, Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionStatement {
    /// RU: Идентификатор транзакции. EN: Transaction ID.
    pub transaction_id: Option<String>,
    /// RU: Идентификатор платежа. EN: Payment ID.
    pub payment_id: Option<String>,
    /// RU: Признак кредит/дебет. EN: Credit/debit indicator.
    pub credit_debit_indicator: CreditDebitIndicator,
    /// RU: Статус. EN: Status.
    pub status: TransactionStatus,
    /// RU: Номер документа. EN: Document number.
    pub document_number: Option<String>,
    /// RU: Код вида операции. EN: Transaction type code.
    pub transaction_type_code: Option<TransationTypeCode>,
    /// RU: Дата проводки. EN: Posting date.
    pub document_process_date: Option<NaiveDate>,
    /// RU: Назначение платежа. EN: Payment description.
    pub description: Option<String>,
    #[serde(flatten)]
    /// RU: Дополнительные поля контрагентов/суммы. EN: Counterparty/amount subfields.
    pub subfields: TransactionSubfields,
}

/// RU: Детализация сумм и контрагентов. EN: Amount and counterparty details.
#[derive(Validate, Serialize, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionSubfields {
    pub amount: Amount,
    pub debtor_party: Contractor,
    pub debtor_account: CashAccount,
    pub debtor_agent: ContractorBank,

    pub creditor_party: Contractor,
    pub creditor_account: CashAccount,
    pub creditor_agent: ContractorBank,

    pub tax_fields: TaxFields,
}

/// RU: Тип платёжного документа. EN: Payment document type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransationTypeCode {
    #[serde(rename = "Неопределенное значение")]
    Undefined,

    #[serde(rename = "Платежное поручение")]
    PaymentOrder,

    #[serde(rename = "Платежное требование")]
    PaymentRequest,

    #[serde(rename = "Денежный чек, РКО")]
    CashCheckRko,

    #[serde(rename = "Объявление на взнос наличными, ПКО")]
    CashDepositPko,

    #[serde(rename = "Требование-поручение")]
    DemandOrder,

    #[serde(rename = "Инкассовое поручение")]
    CollectionOrder,

    #[serde(rename = "Расчетный чек")]
    SettlementCheck,

    #[serde(rename = "Аккредитив")]
    LetterOfCredit,

    #[serde(rename = "Мемориальный ордер")]
    MemorialOrder,

    #[serde(rename = "Погашение кредита")]
    LoanRepayment,

    #[serde(rename = "Выдача кредита")]
    LoanIssuance,

    #[serde(rename = "Авизо")]
    Aviso,

    #[serde(rename = "Банковские карты")]
    BankCards,

    #[serde(rename = "Платежный ордер")]
    PaymentInstruction,

    #[serde(rename = "Банковский ордер")]
    BankOrder,

    #[serde(rename = "Ордер по передаче ценностей")]
    AssetTransferOrder,

    #[serde(rename = "Программный ордер")]
    ProgramOrder,

    #[serde(rename = "Импортированная запись")]
    ImportedRecord,
}
