use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Cinema {
    pub cinema_id: i32,
    pub name: String,
    pub address: String,
    pub employee_count: i32,
    pub hall_count: i32,
    pub opening_time: NaiveTime,
    pub closing_time: NaiveTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCinema {
    pub name: String,
    pub address: String,
    pub employee_count: i32,
    pub hall_count: i32,
    pub opening_time: String,  // Формат "HH:MM:SS"
    pub closing_time: String,  // Формат "HH:MM:SS"
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Film {
    pub film_id: i32,
    pub title: String,
    pub age_restriction: String,
    pub is_booking_available: bool,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewFilm {
    pub title: String,
    pub age_restriction: String,
    pub is_booking_available: bool,
    pub start_date: String,  // Формат "YYYY-MM-DD"
    pub end_date: String,    // Формат "YYYY-MM-DD"
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Customer {
    pub customer_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewCustomer {
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Employee {
    pub employee_id: i32,
    pub cinema_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub position: String,
    pub age: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEmployee {
    pub cinema_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub position: String,
    pub age: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub session_id: i32,
    pub film_id: i32,
    pub cinema_id: i32,
    pub start_time: NaiveDateTime,
    pub ticket_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewSession {
    pub film_id: i32,
    pub cinema_id: i32,
    pub start_time: String,  // Формат "YYYY-MM-DD HH:MM:SS"
    pub ticket_price: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TicketSale {
    pub sale_id: i32,
    pub session_id: i32,
    pub customer_id: i32,
    pub employee_id: i32,
    pub ticket_count: i32,
    pub sale_time: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTicketSale {
    pub session_id: i32,
    pub customer_id: i32,
    pub employee_id: i32,
    pub ticket_count: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FilmOrder {
    pub order_id: i32,
    pub film_id: i32,
    pub cinema_id: i32,
    pub employee_id: i32,
    pub delivery_date: NaiveDate,
    pub rental_cost: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewFilmOrder {
    pub film_id: i32,
    pub cinema_id: i32,
    pub employee_id: i32,
    pub delivery_date: String,  // Формат "YYYY-MM-DD"
    pub rental_cost: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Booking {
    pub booking_id: i32,
    pub session_id: i32,
    pub customer_id: i32,
    pub ticket_count: i32,
    pub booking_time: NaiveDateTime,
    pub status: String,  // "active", "completed", "cancelled"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewBooking {
    pub session_id: i32,
    pub customer_id: i32,
    pub ticket_count: i32,
}