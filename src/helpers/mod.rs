use validator::ValidationError;

/// RU: Проверка телефонного номера: 11–15 символов, допускается ведущий '+'.  
/// EN: Validate phone number length (11–15) with optional leading '+'.
pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    // Length: 11–15
    if phone.len() < 11 || phone.len() > 15 {
        return Err(ValidationError::new("phone_length"));
    }

    // Pattern: optional '+' then digits
    let valid = match phone.strip_prefix('+') {
        Some(rest) => rest.chars().all(|c| c.is_ascii_digit()),
        None => phone.chars().all(|c| c.is_ascii_digit()),
    };

    if !valid {
        return Err(ValidationError::new("phone_pattern"));
    }

    Ok(())
}

/// RU: Проверка ИНН/налогового номера: длина 10–12, только цифры.  
/// EN: Validate tax code length (10–12) and digits-only.
pub fn validate_tax_code(tax: &str) -> Result<(), ValidationError> {
    // Length: 10–12
    if tax.len() < 10 || tax.len() > 12 {
        return Err(ValidationError::new("tax_length"));
    }

    // Pattern: digits only
    if !tax.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new("tax_pattern"));
    }

    Ok(())
}
