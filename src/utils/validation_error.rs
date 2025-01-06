use serde_json::Value;
use std::borrow::Cow;
use validator::ValidationErrors;

pub fn validation_error(errors: ValidationErrors) -> Vec<Value> {
    errors
        .field_errors()
        .iter()
        .map(|(field, errors)| {
            let messages: Vec<String> = errors
                .iter()
                .map(|err| {
                    err.message
                        .clone()
                        .unwrap_or_else(|| Cow::Borrowed("Invalid value"))
                        .to_string()
                })
                .collect();
            serde_json::json!({ "field": field, "errors": messages })
        })
        .collect()
}
