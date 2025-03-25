use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
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
    pub opening_time: String,
    pub closing_time: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
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
    pub start_date: String,
    pub end_date: String,
}

// Аналогично для других моделей (Employee, Customer, Session, TicketSale, FilmOrder, Booking)
