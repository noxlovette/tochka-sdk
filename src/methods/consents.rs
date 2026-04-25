use crate::{
    ApiVersion, Client, Consent, ConsentPageData, CreateConsentPayload, Data, Error,
    PayloadWrapper, Service,
};
use log::debug;

impl Client {
    /// Метод для получения списка всех разрешений
    pub async fn get_consents_list(&self) -> Result<Data<ConsentPageData>, Error> {
        debug!("Fetching consents list");
        self.send::<Data<ConsentPageData>>(
            self.client
                .get(self.url(Service::Consent, ApiVersion::V1_0, "consents")),
        )
        .await
    }

    /// Метод для создания разрешения
    ///
    /// Возвращает согласие с URL авторизации (`authorize_url`), по которому пользователь
    /// должен перейти для подтверждения разрешений.
    pub async fn create_consent(
        &self,
        payload: CreateConsentPayload,
    ) -> Result<Data<Consent>, Error> {
        debug!("Creating consent with {} permissions", payload.permissions.len());
        self.send::<Data<Consent>>(
            self.client
                .post(self.url(Service::Consent, ApiVersion::V1_0, "consents"))
                .json(&PayloadWrapper::wrap(payload)),
        )
        .await
    }

    /// Метод для получения информации о конкретном разрешении
    pub async fn get_consent_info(&self, consent_id: &str) -> Result<Data<Consent>, Error> {
        debug!("Fetching consent info for {consent_id}");
        self.send::<Data<Consent>>(self.client.get(self.url(
            Service::Consent,
            ApiVersion::V1_0,
            format!("consents/{consent_id}").as_str(),
        )))
        .await
    }

    /// Метод для получения всех дочерних разрешений
    ///
    /// Дочерние разрешения создаются автоматически при добавлении субъектов
    /// (например, счетов) в родительское разрешение.
    pub async fn get_child_consents(
        &self,
        consent_id: &str,
    ) -> Result<Data<ConsentPageData>, Error> {
        debug!("Fetching child consents for parent {consent_id}");
        self.send::<Data<ConsentPageData>>(self.client.get(self.url(
            Service::Consent,
            ApiVersion::V1_0,
            format!("consents/{consent_id}/children").as_str(),
        )))
        .await
    }
}
