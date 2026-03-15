use actix_web::{HttpResponse, Responder, get, post, web};
use serde::{Deserialize};
use sqlx::PgPool;
use validator::{Validate, ValidationError};
use crate::models::Hostel;
use crate::utils::{ApiResponse, ApiError};

// Validations
fn validate_pin_code(value: &str) -> Result<(), ValidationError> {
    if value.chars().all(|c| c.is_ascii_digit()) && value.len() == 6 {
        Ok(())
    } else {
        let mut error = ValidationError::new("numeric_only");
        error.message = Some("Pin code must contain only numbers (0-9) and be 6 digits long".into());
        Err(error)
    }
}

// Request Structs

// Handlers
#[get("/health")]
pub async fn health_check() -> impl Responder {
    let response = ApiResponse::new_without_data(200, "Server is running");
    
    HttpResponse::Ok().json(response)
}

