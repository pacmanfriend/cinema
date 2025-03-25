use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingResponse {
    pub booking_id: i32,
    pub session_id: i32,
    pub customer_id: i32,
    pub ticket_count: i32,
    pub booking_time: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookingWithDetailsResponse {
    pub booking_id: i32,
    pub film_title: String,
    pub cinema_name: String,
    pub start_time: String,
    pub customer_name: String,
    pub ticket_count: i32,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookingRequest {
    pub session_id: i32,
    pub customer_id: i32,
    pub ticket_count: i32,
}

pub async fn get_bookings(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let bookings = sqlx::query_as!(
        BookingResponse,
        r#"
        SELECT
            booking_id,
            session_id,
            customer_id,
            ticket_count,
            booking_time::text,
            status
        FROM booking
        "#
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(bookings))
}

pub async fn get_active_bookings(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let bookings = sqlx::query_as!(
        BookingWithDetailsResponse,
        r#"
        SELECT
            b.booking_id,
            f.title as film_title,
            c.name as cinema_name,
            s.start_time::text,
            (cust.first_name || ' ' || cust.last_name) as customer_name,
            b.ticket_count,
            b.status
        FROM booking b
        JOIN session s ON b.session_id = s.session_id
        JOIN film f ON s.film_id = f.film_id
        JOIN cinema c ON s.cinema_id = c.cinema_id
        JOIN customer cust ON b.customer_id = cust.customer_id
        WHERE b.status = 'active'
        AND s.start_time > NOW()
        ORDER BY s.start_time
        "#
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(bookings))
}

pub async fn get_booking(
    pool: web::Data<PgPool>,
    booking_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let booking = sqlx::query_as!(
        BookingWithDetailsResponse,
        r#"
        SELECT
            b.booking_id,
            f.title as film_title,
            c.name as cinema_name,
            s.start_time::text,
            (cust.first_name || ' ' || cust.last_name) as customer_name,
            b.ticket_count,
            b.status
        FROM booking b
        JOIN session s ON b.session_id = s.session_id
        JOIN film f ON s.film_id = f.film_id
        JOIN cinema c ON s.cinema_id = c.cinema_id
        JOIN customer cust ON b.customer_id = cust.customer_id
        WHERE b.booking_id = $1
        "#,
        booking_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await?;

    match booking {
        Some(b) => Ok(HttpResponse::Ok().json(b)),
        None => Err(AppError::NotFound("Booking not found".into())),
    }
}

pub async fn create_booking(
    pool: web::Data<PgPool>,
    new_booking: web::Json<CreateBookingRequest>,
) -> Result<HttpResponse, AppError> {
    let booking = sqlx::query_as!(
        BookingResponse,
        r#"
        INSERT INTO booking (session_id, customer_id, ticket_count, status)
        VALUES ($1, $2, $3, 'active')
        RETURNING
            booking_id,
            session_id,
            customer_id,
            ticket_count,
            booking_time::text,
            status
        "#,
        new_booking.session_id,
        new_booking.customer_id,
        new_booking.ticket_count
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(booking))
}

pub async fn confirm_booking(
    pool: web::Data<PgPool>,
    booking_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let booking = sqlx::query_as!(
        BookingResponse,
        r#"
        UPDATE booking
        SET status = 'completed'
        WHERE booking_id = $1
        RETURNING
            booking_id,
            session_id,
            customer_id,
            ticket_count,
            booking_time::text,
            status
        "#,
        booking_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await?;

    match booking {
        Some(b) => Ok(HttpResponse::Ok().json(b)),
        None => Err(AppError::NotFound("Booking not found".into())),
    }
}

pub async fn cancel_booking(
    pool: web::Data<PgPool>,
    booking_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let booking = sqlx::query_as!(
        BookingResponse,
        r#"
        UPDATE booking
        SET status = 'cancelled'
        WHERE booking_id = $1
        RETURNING
            booking_id,
            session_id,
            customer_id,
            ticket_count,
            booking_time::text,
            status
        "#,
        booking_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await?;

    match booking {
        Some(b) => Ok(HttpResponse::Ok().json(b)),
        None => Err(AppError::NotFound("Booking not found".into())),
    }
}
