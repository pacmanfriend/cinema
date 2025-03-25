use actix_web::{HttpResponse, web};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    pub session_id: i32,
    pub film_id: i32,
    pub cinema_id: i32,
    pub start_time: String,
    pub ticket_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionWithFilmResponse {
    pub session_id: i32,
    pub film_title: String,
    pub cinema_name: String,
    pub start_time: String,
    pub ticket_price: f64,
    pub age_restriction: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub film_id: i32,
    pub cinema_id: i32,
    pub start_time: String,
    pub ticket_price: f64,
}

pub async fn get_sessions(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let sessions = sqlx::query_as!(
        SessionResponse,
        r#"
        SELECT
            session_id,
            film_id,
            cinema_id,
            start_time::text,
            ticket_price
        FROM session
        "#
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(sessions))
}

pub async fn get_upcoming_sessions(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let sessions = sqlx::query_as!(
        SessionWithFilmResponse,
        r#"
        SELECT
            s.session_id,
            f.title as film_title,
            c.name as cinema_name,
            s.start_time::text,
            s.ticket_price,
            f.age_restriction
        FROM session s
        JOIN film f ON s.film_id = f.film_id
        JOIN cinema c ON s.cinema_id = c.cinema_id
        WHERE s.start_time > NOW()
        ORDER BY s.start_time
        "#
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(sessions))
}

pub async fn get_session(
    pool: web::Data<PgPool>,
    session_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let session = sqlx::query_as!(
        SessionWithFilmResponse,
        r#"
        SELECT
            s.session_id,
            f.title as film_title,
            c.name as cinema_name,
            s.start_time::text,
            s.ticket_price,
            f.age_restriction
        FROM session s
        JOIN film f ON s.film_id = f.film_id
        JOIN cinema c ON s.cinema_id = c.cinema_id
        WHERE s.session_id = $1
        "#,
        session_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await?;

    match session {
        Some(s) => Ok(HttpResponse::Ok().json(s)),
        None => Err(AppError::NotFound("Session not found".into())),
    }
}

pub async fn create_session(
    pool: web::Data<PgPool>,
    new_session: web::Json<CreateSessionRequest>,
) -> Result<HttpResponse, AppError> {
    let start_time = NaiveDateTime::parse_from_str(&new_session.start_time, "%Y-%m-%d %H:%M:%S")?;

    let session = sqlx::query_as!(
        SessionResponse,
        r#"
        INSERT INTO session (film_id, cinema_id, start_time, ticket_price)
        VALUES ($1, $2, $3, $4)
        RETURNING
            session_id,
            film_id,
            cinema_id,
            start_time::text,
            ticket_price
        "#,
        new_session.film_id,
        new_session.cinema_id,
        start_time,
        new_session.ticket_price
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(session))
}

pub async fn delete_session(
    pool: web::Data<PgPool>,
    session_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let result = sqlx::query!(
        "DELETE FROM session WHERE session_id = $1",
        session_id.into_inner()
    )
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Session not found".into()));
    }

    Ok(HttpResponse::NoContent().finish())
}
