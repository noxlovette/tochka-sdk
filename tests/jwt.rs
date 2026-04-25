use tochka_sdk::{AcquiringClaims, Client, PaymentMode, PaymentStatus, WebhookType};
use uuid::uuid;

#[tokio::test]
async fn decode_acquiring_payment_webhook_jwt() {
    let client = Client::new().await.unwrap();

    let decoded = client.decode_token::<AcquiringClaims>(TOKEN).unwrap();

    assert_eq!(
        decoded.claims,
        AcquiringClaims {
            customer_code: "300123123".into(),
            amount: "0.33".into(),
            payment_type: PaymentMode::Card,
            operation_id: uuid!("beeac8a4-6047-3f38-8922-a664e6b5c43b"),
            purpose: "Оплата по счету № 1 от 01.01.2021. Без НДС".into(),
            webhook_type: WebhookType::AcquiringInternetPayment,
            merchant_id: "200000000001234".into(),
            consumer_id: Some(uuid!("917ed389-a120-4291-8e73-38c6ef7d6770")),
            status: PaymentStatus::Approved,
            transaction_id: None,
            payer_name: None,
            qrc_id: None
        }
    );
}

const TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJjdXN0b21lckNvZGUiOiAiMzAwMTIzMTIzIiwgImFtb3VudCI6ICIwLjMzIiwgInBheW1lbnRUeXBlIjogImNhcmQiLCAib3BlcmF0aW9uSWQiOiAiYmVlYWM4YTQtNjA0Ny0zZjM4LTg5MjItYTY2NGU2YjVjNDNiIiwgInB1cnBvc2UiOiAiXHUwNDFlXHUwNDNmXHUwNDNiXHUwNDMwXHUwNDQyXHUwNDMwIFx1MDQzZlx1MDQzZSBcdTA0NDFcdTA0NDdcdTA0MzVcdTA0NDJcdTA0NDMgXHUyMTE2IDEgXHUwNDNlXHUwNDQyIDAxLjAxLjIwMjEuIFx1MDQxMVx1MDQzNVx1MDQzNyBcdTA0MWRcdTA0MTRcdTA0MjEiLCAid2ViaG9va1R5cGUiOiAiYWNxdWlyaW5nSW50ZXJuZXRQYXltZW50IiwgIm1lcmNoYW50SWQiOiAiMjAwMDAwMDAwMDAxMjM0IiwgImNvbnN1bWVySWQiOiAiOTE3ZWQzODktYTEyMC00MjkxLThlNzMtMzhjNmVmN2Q2NzcwIiwgInN0YXR1cyI6ICJBUFBST1ZFRCJ9.cFFVd_rpmBFSrefGm2jRM7NhahAY2Jhb_7vwQXs952q18SiIGfyJZdFqGh2uicwBNKOnv_Bto0gDoTZcToEfqPXbHCKCYsA7uu8JUR_DHJtxjYuiilXRgkyDfUxNb0jCqWEfMJz6sGM2AA7kKyQ-Ds9fnGipX9GzYBtksO38PWPmIAJmjYSXsnXYM6bqE8cl-EhbbBbHkCg7vDMV2kwiA0AoW7_SniT4LR0H_NosyaVe0WHq9d7pTipW8rxKl69tFebCl8iz2ZulU0qWLVWPifcF1SbnIAw5WMhEbWBJVNuKYMiGrU_YX958iIOTZ0CyajmEwQutnDIJoMpHi1xOc747ba14H1l1rfDVRXiuNOmzbXYnGBuZ6Bzy2vlJ-54PB4ToQt9nthpi65uWB4YUck3yuoFJzntqkVrmp3ZztoyvVPkpR8ub7nzAhDxLWVLdbM7Q4HQACMcnJXRfkkHlGFR9P8Ygji3OATvNcP0wnxqNxp9P629vG5uICke-Ex0I";
