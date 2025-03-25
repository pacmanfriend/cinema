use actix_web::{HttpResponse, web};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CinemaResponse {
    pub cinema_id: i32,
    pub name: String,
    pub address: String,
    pub employee_count: i32,
    pub hall_count: i32,
    pub opening_time: String,
    pub closing_time: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCinemaRequest {
    pub name: String,
    pub address: String,
    pub employee_count: i32,
    pub hall_count: i32,
    pub opening_time: String,
    pub closing_time: String,
}

pub async fn get_cinemas(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let cinemas = sqlx::query_as!(
        CinemaResponse,
        r#"
        SELECT
            cinema_id,
            name,
            address,
            employee_count,
            hall_count,
            opening_time::text,
            closing_time::text
        FROM cinema
        "#
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(cinemas))
}

pub async fn get_cinema(
    pool: web::Data<PgPool>,
    cinema_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let cinema = sqlx::query_as!(
        CinemaResponse,
        r#"
        SELECT
            cinema_id,
            name,
            address,
            employee_count,
            hall_count,
            opening_time::text,
            closing_time::text
        FROM cinema
        WHERE cinema_id = $1
        "#,
        cinema_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await?;

    match cinema {
        Some(c) => Ok(HttpResponse::Ok().json(c)),
        None => Err(AppError::NotFound("Cinema not found".into())),
    }
}

pub async fn create_cinema(
    pool: web::Data<PgPool>,
    new_cinema: web::Json<CreateCinemaRequest>,
) -> Result<HttpResponse, AppError> {
    let opening_time = NaiveTime::parse_from_str(&new_cinema.opening_time, "%H:%M:%S")?;
    let closing_time = NaiveTime::parse_from_str(&new_cinema.closing_time, "%H:%M:%S")?;

    let cinema = sqlx::query_as!(
        CinemaResponse,
        r#"
        INSERT INTO cinema (name, address, employee_count, hall_count, opening_time, closing_time)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING
            cinema_id,
            name,
            address,
            employee_count,
            hall_count,
            opening_time::text,
            closing_time::text
        "#,
        new_cinema.name,
        new_cinema.address,
        new_cinema.employee_count,
        new_cinema.hall_count,
        opening_time,
        closing_time
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(cinema))
}

pub async fn update_cinema(
    pool: web::Data<PgPool>,
    cinema_id: web::Path<i32>,
    updated_cinema: web::Json<CreateCinemaRequest>,
) -> Result<HttpResponse, AppError> {
    let opening_time = NaiveTime::parse_from_str(&updated_cinema.opening_time, "%H:%M:%S")?;
    let closing_time = NaiveTime::parse_from_str(&updated_cinema.closing_time, "%H:%M:%S")?;

    let cinema = sqlx::query_as!(
        CinemaResponse,
        r#"
        UPDATE cinema
        SET
            name = $1,
            address = $2,
            employee_count = $3,
            hall_count = $4,
            opening_time = $5,
            closing_time = $6
        WHERE cinema_id = $7
        RETURNING
            cinema_id,
            name,
            address,
            employee_count,
            hall_count,
            opening_time::text,
            closing_time::text
        "#,
        updated_cinema.name,
        updated_cinema.address,
        updated_cinema.employee_count,
        updated_cinema.hall_count,
        opening_time,
        closing_time,
        cinema_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await?;

    match cinema {
        Some(c) => Ok(HttpResponse::Ok().json(c)),
        None => Err(AppError::NotFound("Cinema not found".into())),
    }
}

pub async fn delete_cinema(
    pool: web::Data<PgPool>,
    cinema_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let result = sqlx::query!(
        "DELETE FROM cinema WHERE cinema_id = $1",
        cinema_id.into_inner()
    )
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Cinema not found".into()));
    }

    Ok(HttpResponse::NoContent().finish())
}
