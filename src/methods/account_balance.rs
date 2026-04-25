use crate::{
    Balance, BalancePageData, Client, Data, Error, PaginatedResponse, TransactionPageData,
};
use log::debug;

impl Client {
    /// Метод для получения авторизованных карточных транзакций конкретного счёта
    pub async fn get_authorized_card_transactions(
        &self,
        account_id: &str,
    ) -> Result<Data<TransactionPageData>, Error> {
        debug!("Requesting authorized card transactions for account {account_id}");
        self.send::<Data<TransactionPageData>>(self.client.get(self.url(
            crate::Service::OpenBanking,
            crate::ApiVersion::V1_0,
            format!("accounts/{account_id}/authorized-card-transactions").as_str(),
        )))
        .await
    }

    /// Метод получения информации о балансе конкретного счета
    pub async fn get_balance_info(&self, account_id: &str) -> Result<Data<Balance>, Error> {
        debug!("Requesting balance info for account {account_id}");
        self.send::<Data<Balance>>(self.client.get(self.url(
            crate::Service::OpenBanking,
            crate::ApiVersion::V1_0,
            format!("accounts/{account_id}/balances").as_str(),
        )))
        .await
    }

    /// Метод для получения баланса по нескольким счетам
    pub async fn get_balances_list(&self) -> Result<PaginatedResponse<BalancePageData>, Error> {
        debug!("Requesting balances list for all accounts");
        self.send::<PaginatedResponse<BalancePageData>>(self.client.get(self.url(
            crate::Service::OpenBanking,
            crate::ApiVersion::V1_0,
            "accounts/balances",
        )))
        .await
    }
}
