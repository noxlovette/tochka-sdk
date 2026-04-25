use std::collections::HashSet;

use crate::{Account, AccountPageData, ApiVersion, Client, Data, Error, ExternalType, Service};
use log::debug;

const CUSTOMER_CODE_ENV: &str = "CUSTOMER_CODE";

impl Client {
    /// Метод для получения списка доступных счетов
    pub async fn get_accounts_list(&self) -> Result<Data<AccountPageData>, Error> {
        debug!("Fetching accounts list");
        self.send(
            self.client
                .get(self.url(Service::OpenBanking, ApiVersion::V1_0, "accounts")),
        )
        .await
    }

    /// Метод для получения информации по конкретному счёту
    pub async fn get_account_into(&self, account_id: &str) -> Result<Data<Account>, Error> {
        debug!("Fetching account info for {account_id}");
        self.send(self.client.get(self.url(
            Service::OpenBanking,
            ApiVersion::V1_0,
            format!("accounts/{account_id}").as_str(),
        )))
        .await
    }

    /// Попытаться выбрать customer_code из бизнес-счетов.
    ///
    /// Логика:
    /// 1) Если указана переменная окружения CUSTOMER_CODE, возвращаем её.
    /// 2) Иначе получаем список счетов, фильтруем только Business и берём уникальные customer_code.
    /// 3) Если найден один — используем его; если ноль или больше одного — возвращаем ошибку конфигурации
    ///    с подсказкой установить CUSTOMER_CODE вручную.
    pub async fn resolve_business_customer_code(&self) -> Result<String, Error> {
        if let Ok(code) = std::env::var(CUSTOMER_CODE_ENV) {
            debug!("Using customer_code from {CUSTOMER_CODE_ENV} env var");
            return Ok(code);
        }

        debug!("CUSTOMER_CODE not set, resolving via Business accounts");
        let accounts = self.get_accounts_list().await?.data.account;
        select_business_customer_code(&accounts)
    }
}

// Вспомогательная функция выделена отдельно для возможности unit-тестирования.
fn select_business_customer_code(accounts: &[Account]) -> Result<String, Error> {
    let mut business_codes: HashSet<String> = accounts
        .iter()
        .filter(|acc| acc.account_type == ExternalType::Business)
        .map(|acc| acc.customer_code.clone())
        .collect();
    debug!(
        "Found {} business accounts before deduplication",
        business_codes.len()
    );
    // Стабилизируем порядок, чтобы предсказуемо доставать единственный элемент.
    let mut unique_codes: Vec<String> = business_codes.drain().collect();
    unique_codes.sort();
    debug!(
        "Unique business customer codes resolved: {}",
        unique_codes.len()
    );

    match unique_codes.len() {
        0 => Err(Error::Config(
            "no Business accounts returned; set CUSTOMER_CODE to pick one".into(),
        )),
        1 => Ok(unique_codes.swap_remove(0)),
        _ => Err(Error::Config(
            "multiple Business accounts returned; set CUSTOMER_CODE to choose which one to use"
                .into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, Utc};
    use codes_iso_4217::CurrencyCode;

    use super::*;
    use crate::{AccountDetail, AccountStatus, AccountSubType};

    fn stub_account(customer_code: &str, account_type: ExternalType) -> Account {
        Account {
            customer_code: customer_code.to_string(),
            account_id: "id".into(),
            transit_account: None,
            status: AccountStatus::Enabled,
            status_update_date_time: Utc::now(),
            currency: CurrencyCode::RUB,
            account_type,
            account_sub_type: AccountSubType::CurrentAccount,
            registration_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            account_details: Some(vec![AccountDetail {
                identification: "ident".into(),
                name: "name".into(),
                scheme_name: "scheme".into(),
            }]),
        }
    }

    #[test]
    fn picks_single_business_code() {
        let accounts = vec![
            stub_account("BIZ1", ExternalType::Business),
            stub_account("PERSON1", ExternalType::Personal),
        ];

        let code = select_business_customer_code(&accounts).unwrap();
        assert_eq!(code, "BIZ1");
    }

    #[test]
    fn errors_when_no_business_accounts() {
        let accounts = vec![stub_account("PERSON1", ExternalType::Personal)];

        match select_business_customer_code(&accounts) {
            Err(Error::Config(msg)) => {
                assert!(msg.contains("no Business accounts"));
            }
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn errors_when_multiple_business_accounts() {
        let accounts = vec![
            stub_account("BIZ1", ExternalType::Business),
            stub_account("BIZ2", ExternalType::Business),
        ];

        match select_business_customer_code(&accounts) {
            Err(Error::Config(msg)) => {
                assert!(msg.contains("multiple Business accounts"));
            }
            other => panic!("unexpected result: {other:?}"),
        }
    }
}
