use crate::{ApiVersion, Client, Data, Error, ResultBody, Service, Webhook, WebhookType};
use log::debug;

impl Client {
    /// Метод для создания вебхуков
    ///
    /// Требует предварительной настройки `client_id` через `with_client_id()`.
    /// Метод использует PUT (не POST) — это особенность API Точки.
    pub async fn create_webhook(&self, payload: Webhook) -> Result<Data<Webhook>, Error> {
        let id = self.client_id.as_deref().ok_or_else(|| {
            Error::Config("client_id must be set via with_client_id() before create_webhook()".into())
        })?;
        debug!("Creating webhook for client_id {id}");
        self.send::<Data<Webhook>>(
            self.client
                .put(self.url(Service::Webhook, ApiVersion::V1_0, id))
                .json(&payload),
        )
        .await
    }

    /// Метод для изменения URL и типа вебхука
    ///
    /// Требует предварительной настройки `client_id` через `with_client_id()`.
    /// Метод использует POST (не PUT) — это особенность API Точки.
    pub async fn edit_webhook(&self, payload: Webhook) -> Result<Data<Webhook>, Error> {
        let id = self.client_id.as_deref().ok_or_else(|| {
            Error::Config("client_id must be set via with_client_id() before edit_webhook()".into())
        })?;
        debug!("Editing webhook for client_id {id}");
        self.send::<Data<Webhook>>(
            self.client
                .post(self.url(Service::Webhook, ApiVersion::V1_0, id))
                .json(&payload),
        )
        .await
    }

    /// Метод для получения списка вебхуков приложения
    pub async fn get_webhooks(&self) -> Result<Data<Webhook>, Error> {
        let id = self.client_id.as_deref().ok_or_else(|| {
            Error::Config("client_id must be set via with_client_id() before get_webhooks()".into())
        })?;
        debug!("Fetching webhooks for client_id {id}");
        self.send::<Data<Webhook>>(
            self.client
                .get(self.url(Service::Webhook, ApiVersion::V1_0, id)),
        )
        .await
    }

    /// Метод для удаления вебхука
    pub async fn delete_webhook(&self) -> Result<Data<ResultBody>, Error> {
        let id = self.client_id.as_deref().ok_or_else(|| {
            Error::Config("client_id must be set via with_client_id() before delete_webhook()".into())
        })?;
        debug!("Deleting webhook for client_id {id}");
        self.send::<Data<ResultBody>>(
            self.client
                .delete(self.url(Service::Webhook, ApiVersion::V1_0, id)),
        )
        .await
    }

    /// Метод для проверки отправки вебхука
    pub async fn send_webhook(&self, payload: WebhookType) -> Result<Data<ResultBody>, Error> {
        let id = self.client_id.as_deref().ok_or_else(|| {
            Error::Config("client_id must be set via with_client_id() before send_webhook()".into())
        })?;
        debug!("Triggering test webhook send for client_id {id}");
        self.send::<Data<ResultBody>>(
            self.client
                .post(self.url(
                    Service::Webhook,
                    ApiVersion::V1_0,
                    format!("{id}/test_send").as_str(),
                ))
                .json(&payload),
        )
        .await
    }
}
