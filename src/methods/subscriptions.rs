use crate::{
    ApiVersion, ChargeSubscriptionPayload, Client, CreateSubscriptionPayload,
    CreateSubscriptionWithReceiptPayload, Data, Error, PayloadWrapper, Service, SetSubscriptionStatusPayload,
    Subscription, SubscriptionPageData, SubscriptionStatus, SubscriptionStatusResponse,
};
use log::debug;
use uuid::Uuid;

impl Client {
    /// Метод для создания подписки по карте
    pub async fn create_subscription(
        &self,
        payload: CreateSubscriptionPayload,
    ) -> Result<Data<Subscription>, Error> {
        debug!(
            "Creating subscription: amount={} customer={}",
            payload.amount, payload.customer_code
        );
        self.send::<Data<Subscription>>(
            self.client
                .post(self.url(Service::Acquiring, ApiVersion::V1_0, "subscriptions"))
                .json(&PayloadWrapper::wrap(payload)),
        )
        .await
    }

    /// Метод для создания подписки с отправкой чека
    pub async fn create_subscription_with_receipt(
        &self,
        payload: CreateSubscriptionWithReceiptPayload,
    ) -> Result<Data<Subscription>, Error> {
        debug!(
            "Creating subscription with receipt: amount={} customer={}",
            payload.amount, payload.customer_code
        );
        self.send::<Data<Subscription>>(
            self.client
                .post(self.url(
                    Service::Acquiring,
                    ApiVersion::V1_0,
                    "subscriptions/with-receipt",
                ))
                .json(&PayloadWrapper::wrap(payload)),
        )
        .await
    }

    /// Метод для получения всех подписок
    pub async fn get_subscriptions_list(&self) -> Result<Data<SubscriptionPageData>, Error> {
        debug!("Fetching subscriptions list");
        self.send::<Data<SubscriptionPageData>>(
            self.client
                .get(self.url(Service::Acquiring, ApiVersion::V1_0, "subscriptions")),
        )
        .await
    }

    /// Метод для списания средств по рекуррентной подписке
    pub async fn charge_subscription(
        &self,
        operation_id: Uuid,
        payload: ChargeSubscriptionPayload,
    ) -> Result<Data<Subscription>, Error> {
        debug!("Charging subscription {operation_id}: amount={}", payload.amount);
        self.send::<Data<Subscription>>(
            self.client
                .post(self.url(
                    Service::Acquiring,
                    ApiVersion::V1_0,
                    format!("subscriptions/{operation_id}/charge").as_str(),
                ))
                .json(&PayloadWrapper::wrap(payload)),
        )
        .await
    }

    /// Метод для установки статуса подписки
    pub async fn set_subscription_status(
        &self,
        operation_id: Uuid,
        status: SubscriptionStatus,
    ) -> Result<Data<SubscriptionStatusResponse>, Error> {
        debug!("Setting subscription {operation_id} status to {status}");
        let payload = SetSubscriptionStatusPayload { status };
        self.send::<Data<SubscriptionStatusResponse>>(
            self.client
                .post(self.url(
                    Service::Acquiring,
                    ApiVersion::V1_0,
                    format!("subscriptions/{operation_id}/status").as_str(),
                ))
                .json(&payload),
        )
        .await
    }

    /// Метод для получения актуального статуса подписки
    pub async fn get_subscription_status(
        &self,
        operation_id: Uuid,
    ) -> Result<Data<SubscriptionStatusResponse>, Error> {
        debug!("Fetching subscription {operation_id} status");
        self.send::<Data<SubscriptionStatusResponse>>(self.client.get(self.url(
            Service::Acquiring,
            ApiVersion::V1_0,
            format!("subscriptions/{operation_id}/status").as_str(),
        )))
        .await
    }
}
