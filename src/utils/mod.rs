use actix_web::{HttpRequest, HttpResponse, error};
use serde::Serialize;

// Api response struct
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub status_code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>
}

impl<T> ApiResponse<T> {
    pub fn new(status_code: u16, message: &str, data: T) -> Self {
        Self {
            success: true,
            status_code,
            message: message.to_string(),
            data: Some(data)
        }
    }
}

impl ApiResponse<()> {
    pub fn new_without_data(status_code: u16, message: &str) -> Self {
        Self {
            success: true,
            status_code,
            message: message.to_string(),
            data: None
        }
    }
}

// Api error struct
#[derive(Serialize)]
pub struct ApiError {
    pub success: bool,
    pub status_code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(status_code: u16, message: &str) -> Self {
        Self {
            success: false,
            status_code,
            message: message.to_string()
        }
    }
}

pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> actix_web::Error {
    let error_message = format!("Invalid request payload: {}", err);

    let api_error = ApiError::new(400, &error_message);

    error::InternalError::from_response(
        err,
        HttpResponse::BadRequest().json(api_error)
    ).into()
}