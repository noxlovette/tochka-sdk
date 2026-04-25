use tochka_sdk::{Customer, CustomerPageData, Data, ExternalType, PaginatedResponse};

#[test]
fn deserialize_customer_info() {
    let json = r#"
    {
  "Data": {
    "customerCode": "300000092",
    "customerType": "Personal",
    "isResident": true,
    "taxCode": "660000000000",
    "fullName": "Индивидуальный Предприниматель Тест",
    "shortName": "ИП Тест",
    "kpp": "668501001",
    "customerOgrn": "319665800211661"
  },
  "Links": {
    "self": "https://enter.tochka.com/uapi"
  },
  "Meta": {
    "totalPages": 1
  }
}
    "#;

    let parsed: Data<Customer> = serde_json::from_str(json).unwrap();

    assert_eq!(parsed.data.customer_code, "300000092");
    assert_eq!(parsed.data.customer_type, ExternalType::Personal);
}

#[test]
fn deserialize_customer_list() {
    let json = r#"
    
 {
  "Data": {
    "Customer": [
      {
        "customerCode": "300000092",
        "customerType": "Personal",
        "isResident": true,
        "taxCode": "660000000000",
        "fullName": "Индивидуальный Предприниматель Тест",
        "shortName": "ИП Тест",
        "kpp": "668501001",
        "customerOgrn": "319665800211661"
      }
    ]
  },
  "Links": {
    "self": "https://enter.tochka.com/uapi"
  },
  "Meta": {
    "totalPages": 1
  }
}
    "#;

    let parsed: PaginatedResponse<CustomerPageData> = serde_json::from_str(json).unwrap();

    assert_eq!(
        parsed.data.customer[0].tax_code,
        Some("660000000000".to_string())
    );
    assert_eq!(
        parsed.data.customer[0].customer_type,
        ExternalType::Personal
    );
}
