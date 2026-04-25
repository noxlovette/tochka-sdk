use crate::{
    ApiVersion, Client, CreatePaymentForSignPayload, Data, Error, PayloadWrapper,
    PaymentForSign, PaymentForSignPageData, PaymentForSignQuery, PaginatedResponse, Service,
};
use log::debug;

impl Client {
    /// Метод для создания платёжного поручения на подпись
    ///
    /// Создаёт платёжный документ, который должен быть подтверждён владельцем счёта.
    /// После подписи платёж отправляется в банк.
    pub async fn create_payment_for_sign(
        &self,
        payload: CreatePaymentForSignPayload,
    ) -> Result<Data<PaymentForSign>, Error> {
        debug!(
            "Creating payment for sign: amount={} customer={}",
            payload.amount, payload.customer_code
        );
        self.send::<Data<PaymentForSign>>(
            self.client
                .post(self.url(Service::Payment, ApiVersion::V1_0, "for-sign"))
                .json(&PayloadWrapper::wrap(payload)),
        )
        .await
    }

    /// Метод получения списка платёжных поручений на подпись
    pub async fn get_payment_for_sign_list(
        &self,
        query: PaymentForSignQuery,
    ) -> Result<PaginatedResponse<PaymentForSignPageData>, Error> {
        debug!("Fetching payment-for-sign list with query: {:?}", query);
        self.send::<PaginatedResponse<PaymentForSignPageData>>(
            self.client
                .get(self.url(Service::Payment, ApiVersion::V1_0, "for-sign"))
                .query(&query),
        )
        .await
    }

    /// Метод для получения статуса конкретного платёжного поручения
    pub async fn get_payment_for_sign_status(
        &self,
        request_id: impl Into<String>,
    ) -> Result<Data<PaymentForSign>, Error> {
        let request_id = request_id.into();
        debug!("Fetching payment-for-sign status for request_id {request_id}");
        self.send::<Data<PaymentForSign>>(self.client.get(self.url(
            Service::Payment,
            ApiVersion::V1_0,
            format!("status/{request_id}").as_str(),
        )))
        .await
    }
}
