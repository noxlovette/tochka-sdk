use crate::{
    ApiVersion, Client, ClosingDocument, CreateClosingDocumentPayload, CreateInvoicePayload, Data,
    Error, Invoice, PayloadWrapper, ResultBody, SendDocumentEmailPayload, Service,
};
use log::debug;

impl Client {
    // ─── Invoices ────────────────────────────────────────────────────────────

    /// Метод для создания счёта на оплату
    pub async fn create_invoice(
        &self,
        payload: CreateInvoicePayload,
    ) -> Result<Data<Invoice>, Error> {
        debug!(
            "Creating invoice: amount={} customer={}",
            payload.amount, payload.customer_code
        );
        self.send::<Data<Invoice>>(
            self.client
                .post(self.url(Service::Invoice, ApiVersion::V1_0, "bills"))
                .json(&PayloadWrapper::wrap(payload)),
        )
        .await
    }

    /// Метод для удаления счёта на оплату
    pub async fn delete_invoice(
        &self,
        customer_code: &str,
        document_id: &str,
    ) -> Result<Data<ResultBody>, Error> {
        debug!("Deleting invoice {document_id} for customer {customer_code}");
        self.send::<Data<ResultBody>>(self.client.delete(self.url(
            Service::Invoice,
            ApiVersion::V1_0,
            format!("bills/{customer_code}/{document_id}").as_str(),
        )))
        .await
    }

    /// Метод для отправки счёта на почту
    pub async fn send_invoice_email(
        &self,
        customer_code: &str,
        document_id: &str,
        payload: SendDocumentEmailPayload,
    ) -> Result<Data<ResultBody>, Error> {
        debug!(
            "Sending invoice {document_id} for customer {customer_code} to {}",
            payload.email
        );
        self.send::<Data<ResultBody>>(
            self.client
                .post(self.url(
                    Service::Invoice,
                    ApiVersion::V1_0,
                    format!("bills/{customer_code}/{document_id}/email").as_str(),
                ))
                .json(&payload),
        )
        .await
    }

    /// Метод для получения файла выставленного счёта (бинарные данные)
    pub async fn get_invoice_file(
        &self,
        customer_code: &str,
        document_id: &str,
    ) -> Result<Vec<u8>, Error> {
        debug!("Downloading invoice file {document_id} for customer {customer_code}");
        self.download(self.client.get(self.url(
            Service::Invoice,
            ApiVersion::V1_0,
            format!("bills/{customer_code}/{document_id}/file").as_str(),
        )))
        .await
    }

    /// Метод для получения статуса оплаты счёта
    pub async fn get_invoice_payment_status(
        &self,
        customer_code: &str,
        document_id: &str,
    ) -> Result<Data<Invoice>, Error> {
        debug!("Fetching invoice payment status for {document_id}, customer {customer_code}");
        self.send::<Data<Invoice>>(self.client.get(self.url(
            Service::Invoice,
            ApiVersion::V1_0,
            format!("bills/{customer_code}/{document_id}/payment-status").as_str(),
        )))
        .await
    }

    // ─── Closing Documents ────────────────────────────────────────────────────

    /// Метод для создания закрывающего документа
    pub async fn create_closing_document(
        &self,
        payload: CreateClosingDocumentPayload,
    ) -> Result<Data<ClosingDocument>, Error> {
        debug!(
            "Creating closing document: amount={} customer={}",
            payload.amount, payload.customer_code
        );
        self.send::<Data<ClosingDocument>>(
            self.client
                .post(self.url(
                    Service::Invoice,
                    ApiVersion::V1_0,
                    "closing-documents",
                ))
                .json(&PayloadWrapper::wrap(payload)),
        )
        .await
    }

    /// Метод для удаления закрывающего документа
    pub async fn delete_closing_document(
        &self,
        customer_code: &str,
        document_id: &str,
    ) -> Result<Data<ResultBody>, Error> {
        debug!("Deleting closing document {document_id} for customer {customer_code}");
        self.send::<Data<ResultBody>>(self.client.delete(self.url(
            Service::Invoice,
            ApiVersion::V1_0,
            format!("closing-documents/{customer_code}/{document_id}").as_str(),
        )))
        .await
    }

    /// Метод для отправки закрывающего документа на почту
    pub async fn send_closing_document_email(
        &self,
        customer_code: &str,
        document_id: &str,
        payload: SendDocumentEmailPayload,
    ) -> Result<Data<ResultBody>, Error> {
        debug!(
            "Sending closing document {document_id} for customer {customer_code} to {}",
            payload.email
        );
        self.send::<Data<ResultBody>>(
            self.client
                .post(self.url(
                    Service::Invoice,
                    ApiVersion::V1_0,
                    format!("closing-documents/{customer_code}/{document_id}/email").as_str(),
                ))
                .json(&payload),
        )
        .await
    }

    /// Метод для получения файла закрывающего документа (бинарные данные)
    pub async fn get_closing_document_file(
        &self,
        customer_code: &str,
        document_id: &str,
    ) -> Result<Vec<u8>, Error> {
        debug!(
            "Downloading closing document file {document_id} for customer {customer_code}"
        );
        self.download(self.client.get(self.url(
            Service::Invoice,
            ApiVersion::V1_0,
            format!("closing-documents/{customer_code}/{document_id}/file").as_str(),
        )))
        .await
    }
}
