/// RU: Сервисы API Tochka. EN: Tochka API services.
#[derive(Debug, Clone, Copy)]
pub enum Service {
    /// RU: Open Banking. EN: Open Banking.
    OpenBanking,
    /// RU: Платежи. EN: Payments.
    Payment,
    /// RU: Эквайринг. EN: Acquiring.
    Acquiring,
    /// RU: Счета на оплату. EN: Invoice.
    Invoice,
    /// RU: Согласия. EN: Consents.
    Consent,
    /// RU: СБП. EN: SBP.
    Sbp,
    /// RU: Вебхуки. EN: Webhooks.
    Webhook,
}

impl Service {
    /// RU: Путь сервиса в URL. EN: URL path segment for the service.
    pub fn path(&self) -> &'static str {
        match self {
            Service::OpenBanking => "open-banking",
            Service::Payment => "payment",
            Service::Acquiring => "acquiring",
            Service::Invoice => "invoice",
            Service::Consent => "consent",
            Service::Sbp => "sbp",
            Service::Webhook => "webhook",
        }
    }
}
