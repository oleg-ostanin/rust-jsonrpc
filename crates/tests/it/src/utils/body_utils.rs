use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct ErrorDetail {
    message: Option<String>,
    data: ErrorData,
}

#[derive(Deserialize)]
struct ErrorData {
    detail: Option<String>,
    req_uuid: Option<String>,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: ErrorDetail,
    id: Option<String>,
}

pub(crate) fn get_message(json: Value) -> String {
    let error_response: ErrorResponse = serde_json::from_value(json).unwrap();
    error_response.error.message.unwrap()
}