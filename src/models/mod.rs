use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Hostel {
    pub id: i32,
    pub name: String,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub pin_code: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Room {
    pub id: i32,
    pub hostel_id: i32,
    pub room_name: String,
    pub fee: f32,
    pub deposit: f32,
    pub is_available: bool,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Student {
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub student_contact: String,

    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub pin_code: String,

    pub father_full_name: Option<String>,
    pub father_contact: Option<String>,
    pub father_email: Option<String>,

    pub mother_full_name: Option<String>,
    pub mother_contact: Option<String>,
    pub mother_email: Option<String>,

    pub guardian_full_name: Option<String>,
    pub guardian_contact: Option<String>,
    pub guardian_email: Option<String>,   
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Allocation {
    pub id: i32,
    pub student_id: i32,
    pub room_id: i32,
    pub check_in_date: NaiveDate,
    pub check_out_date: Option<NaiveDate>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug)]
pub struct Payment {
    pub id: i32,
    pub student_id: i32,
    pub hostel_id: i32,
    pub amount: f32,
    pub payment_type: String,
    pub payment_date: NaiveDateTime,
    pub description: Option<String>,
}