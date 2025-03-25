use actix_web::{HttpResponse, web};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilmResponse {
    pub film_id: i32,
    pub title: String,
    pub age_restriction: String,
    pub is_booking_available: bool,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateFilmRequest {
    pub title: String,
    pub age_restriction: String,
    pub is_booking_available: bool,
    pub start_date: String,
    pub end_date: String,
}

pub async fn get_films(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let films = sqlx::query_as!(
        FilmResponse,
        r#"
        SELECT
            film_id,
            title,
            age_restriction,
            is_booking_available,
            start_date::text,
            end_date::text
        FROM film
        "#
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(films))
}

pub async fn get_active_films(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let films = sqlx::query_as!(
        FilmResponse,
        r#"
        SELECT
            film_id,
            title,
            age_restriction,
            is_booking_available,
            start_date::text,
            end_date::text
        FROM film
        WHERE end_date >= CURRENT_DATE
        "#
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(films))
}

pub async fn get_film(
    pool: web::Data<PgPool>,
    film_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let film = sqlx::query_as!(
        FilmResponse,
        r#"
        SELECT
            film_id,
            title,
            age_restriction,
            is_booking_available,
            start_date::text,
            end_date::text
        FROM film
        WHERE film_id = $1
        "#,
        film_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await?;

    match film {
        Some(f) => Ok(HttpResponse::Ok().json(f)),
        None => Err(AppError::NotFound("Film not found".into())),
    }
}

pub async fn create_film(
    pool: web::Data<PgPool>,
    new_film: web::Json<CreateFilmRequest>,
) -> Result<HttpResponse, AppError> {
    let start_date = NaiveDate::parse_from_str(&new_film.start_date, "%Y-%m-%d")?;
    let end_date = NaiveDate::parse_from_str(&new_film.end_date, "%Y-%m-%d")?;

    let film = sqlx::query_as!(
        FilmResponse,
        r#"
        INSERT INTO film (title, age_restriction, is_booking_available, start_date, end_date)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING
            film_id,
            title,
            age_restriction,
            is_booking_available,
            start_date::text,
            end_date::text
        "#,
        new_film.title,
        new_film.age_restriction,
        new_film.is_booking_available,
        start_date,
        end_date
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(film))
}

pub async fn update_film(
    pool: web::Data<PgPool>,
    film_id: web::Path<i32>,
    updated_film: web::Json<CreateFilmRequest>,
) -> Result<HttpResponse, AppError> {
    let start_date = NaiveDate::parse_from_str(&updated_film.start_date, "%Y-%m-%d")?;
    let end_date = NaiveDate::parse_from_str(&updated_film.end_date, "%Y-%m-%d")?;

    let film = sqlx::query_as!(
        FilmResponse,
        r#"
        UPDATE film
        SET
            title = $1,
            age_restriction = $2,
            is_booking_available = $3,
            start_date = $4,
            end_date = $5
        WHERE film_id = $6
        RETURNING
            film_id,
            title,
            age_restriction,
            is_booking_available,
            start_date::text,
            end_date::text
        "#,
        updated_film.title,
        updated_film.age_restriction,
        updated_film.is_booking_available,
        start_date,
        end_date,
        film_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await?;

    match film {
        Some(f) => Ok(HttpResponse::Ok().json(f)),
        None => Err(AppError::NotFound("Film not found".into())),
    }
}

pub async fn delete_film(
    pool: web::Data<PgPool>,
    film_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let result = sqlx::query!("DELETE FROM film WHERE film_id = $1", film_id.into_inner())
        .execute(pool.get_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Film not found".into()));
    }

    Ok(HttpResponse::NoContent().finish())
}
