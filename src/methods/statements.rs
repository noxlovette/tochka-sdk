use crate::{Client, Data, Error, PayloadWrapper, Service, StatementPageData, StatementPayload};
use log::debug;

impl Client {
    /// Метод для получения конкретной выписки
    ///
    ///  После вызова метода `Init Statement` с помощью `statementId` можно отследить,на каком этапе создание определённой выписки.
    ///
    /// Есть три статуса: *Created* — создан запрос на выписку; *Processing* — запрос в обработке; *Ready* — выписка готова.
    ///
    /// **Особенности:** Метод *Init Statement* отрабатывает асинхронно.Отражаются только операции, находящиеся в финальном статусе — *Ready*.
    pub async fn get_statement(
        &self,
        accound_id: &str,
        statement_id: &str,
    ) -> Result<Data<StatementPageData>, Error> {
        debug!("Fetching statement {statement_id} for account {accound_id}");
        self.send::<Data<StatementPageData>>(self.client.get(self.url(
            Service::OpenBanking,
            crate::ApiVersion::V1_0,
            format!("accounts/{accound_id}/statements/{statement_id}").as_str(),
        )))
        .await
    }

    /// Метод для создания выписки по конкретному счёту
    pub async fn init_statement(
        &self,
        payload: StatementPayload,
    ) -> Result<Data<StatementPageData>, Error> {
        debug!("Initializing statement with payload: {:?}", payload);
        self.send::<Data<StatementPageData>>(
            self.client
                .post(self.url(Service::OpenBanking, crate::ApiVersion::V1_0, "statements"))
                .json(&PayloadWrapper::wrap(payload)),
        )
        .await
    }
    /// Метод для получения списка доступных выписок
    ///  
    /// После вызова метода `Init Statement` можно отследить, в каком статусе готовящаяся выписка:
    /// *Created* — только создан запрос на выписку;
    /// *Processing* — запрос в обработке;
    /// *Ready* — выписка готова.
    ///  
    /// **Особенности:**
    /// Отражаются только операции, находящиеся в финальном статусе — *Ready*.
    pub async fn get_statements_list(&self) -> Result<Data<StatementPageData>, Error> {
        debug!("Fetching statements list");
        self.send::<Data<StatementPageData>>(self.client.get(self.url(
            Service::OpenBanking,
            crate::ApiVersion::V1_0,
            "statements",
        )))
        .await
    }
}
