use crate::{
    ApiVersion, Client, Data, Error, PayloadWrapper, PaginatedResponse, QrCode, QrCodePageData,
    QrCodePaymentStatus, RegisterMerchantPayload, RegisterQrCodePayload, SbpMerchant,
    SbpMerchantPageData, SbpPaymentPageData, SbpPaymentsQuery, SbpRefund, SbpRefundPayload,
    Service, SetMerchantStatusPayload, MerchantStatus,
};
use log::debug;

impl Client {
    // ─── QR Codes ─────────────────────────────────────────────────────────────

    /// Метод для получения списка QR-кодов юрлица
    pub async fn get_qr_codes_list(
        &self,
        legal_id: &str,
    ) -> Result<Data<QrCodePageData>, Error> {
        debug!("Fetching QR codes for legal entity {legal_id}");
        self.send::<Data<QrCodePageData>>(self.client.get(self.url(
            Service::Sbp,
            ApiVersion::V1_0,
            format!("qr-code/legal-entity/{legal_id}").as_str(),
        )))
        .await
    }

    /// Метод для регистрации статического или динамического QR-кода в СБП
    pub async fn register_qr_code(
        &self,
        merchant_id: &str,
        account_id: &str,
        payload: RegisterQrCodePayload,
    ) -> Result<Data<QrCode>, Error> {
        debug!(
            "Registering {:?} QR code for merchant {merchant_id} account {account_id}",
            payload.qrc_type
        );
        self.send::<Data<QrCode>>(
            self.client
                .post(self.url(
                    Service::Sbp,
                    ApiVersion::V1_0,
                    format!("qr-code/merchant/{merchant_id}/{account_id}").as_str(),
                ))
                .json(&PayloadWrapper::wrap(payload)),
        )
        .await
    }

    /// Метод для получения информации о QR-коде
    pub async fn get_qr_code(&self, qrc_id: &str) -> Result<Data<QrCode>, Error> {
        debug!("Fetching QR code {qrc_id}");
        self.send::<Data<QrCode>>(self.client.get(self.url(
            Service::Sbp,
            ApiVersion::V1_0,
            format!("qr-code/{qrc_id}").as_str(),
        )))
        .await
    }

    /// Метод для получения статусов операций по динамическим QR-кодам
    ///
    /// `qrc_ids` — список идентификаторов QR-кодов через запятую.
    pub async fn get_qr_codes_payment_status(
        &self,
        qrc_ids: &str,
    ) -> Result<Data<Vec<QrCodePaymentStatus>>, Error> {
        debug!("Fetching payment status for QR codes: {qrc_ids}");
        self.send::<Data<Vec<QrCodePaymentStatus>>>(self.client.get(self.url(
            Service::Sbp,
            ApiVersion::V1_0,
            format!("qr-codes/{qrc_ids}/payment-status").as_str(),
        )))
        .await
    }

    // ─── Merchants (TSP) ──────────────────────────────────────────────────────

    /// Метод для регистрации ТСП в СБП
    pub async fn register_sbp_merchant(
        &self,
        legal_id: &str,
        payload: RegisterMerchantPayload,
    ) -> Result<Data<SbpMerchant>, Error> {
        debug!("Registering SBP merchant for legal entity {legal_id}");
        self.send::<Data<SbpMerchant>>(
            self.client
                .post(self.url(
                    Service::Sbp,
                    ApiVersion::V1_0,
                    format!("merchant/legal-entity/{legal_id}").as_str(),
                ))
                .json(&PayloadWrapper::wrap(payload)),
        )
        .await
    }

    /// Метод для получения списка ТСП юрлица
    pub async fn get_sbp_merchants_list(
        &self,
        legal_id: &str,
    ) -> Result<Data<SbpMerchantPageData>, Error> {
        debug!("Fetching SBP merchants for legal entity {legal_id}");
        self.send::<Data<SbpMerchantPageData>>(self.client.get(self.url(
            Service::Sbp,
            ApiVersion::V1_0,
            format!("merchant/legal-entity/{legal_id}").as_str(),
        )))
        .await
    }

    /// Метод для установки статуса ТСП
    pub async fn set_sbp_merchant_status(
        &self,
        merchant_id: &str,
        status: MerchantStatus,
    ) -> Result<Data<SbpMerchant>, Error> {
        debug!("Setting SBP merchant {merchant_id} status to {status}");
        let payload = SetMerchantStatusPayload { status };
        self.send::<Data<SbpMerchant>>(
            self.client
                .put(self.url(
                    Service::Sbp,
                    ApiVersion::V1_0,
                    format!("merchant/{merchant_id}").as_str(),
                ))
                .json(&payload),
        )
        .await
    }

    /// Метод для получения информации о ТСП
    pub async fn get_sbp_merchant(&self, merchant_id: &str) -> Result<Data<SbpMerchant>, Error> {
        debug!("Fetching SBP merchant {merchant_id}");
        self.send::<Data<SbpMerchant>>(self.client.get(self.url(
            Service::Sbp,
            ApiVersion::V1_0,
            format!("merchant/{merchant_id}").as_str(),
        )))
        .await
    }

    // ─── SBP Refunds ──────────────────────────────────────────────────────────

    /// Метод для получения списка платежей в СБП
    pub async fn get_sbp_payments(
        &self,
        query: SbpPaymentsQuery,
    ) -> Result<PaginatedResponse<SbpPaymentPageData>, Error> {
        debug!("Fetching SBP payments with query: {:?}", query);
        self.send::<PaginatedResponse<SbpPaymentPageData>>(
            self.client
                .get(self.url(Service::Sbp, ApiVersion::V1_0, "get-sbp-payments"))
                .query(&query),
        )
        .await
    }

    /// Метод запроса возврата платежа через СБП
    pub async fn start_sbp_refund(
        &self,
        payload: SbpRefundPayload,
    ) -> Result<Data<SbpRefund>, Error> {
        debug!("Starting SBP refund: trx_id={} amount={}", payload.trx_id, payload.amount);
        self.send::<Data<SbpRefund>>(
            self.client
                .post(self.url(Service::Sbp, ApiVersion::V1_0, "refund"))
                .json(&PayloadWrapper::wrap(payload)),
        )
        .await
    }

    /// Метод для получения информации о возврате в СБП
    pub async fn get_sbp_refund(&self, request_id: &str) -> Result<Data<SbpRefund>, Error> {
        debug!("Fetching SBP refund {request_id}");
        self.send::<Data<SbpRefund>>(self.client.get(self.url(
            Service::Sbp,
            ApiVersion::V1_0,
            format!("refund/{request_id}").as_str(),
        )))
        .await
    }
}
