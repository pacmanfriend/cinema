use super::errors::AppError;
use super::models::{Cinema, Film, NewCinema, NewFilm};
use actix_web::{HttpResponse, web};
use chrono::{NaiveDate, NaiveTime};
use sqlx::PgPool;

pub async fn test(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json("test"))
}

// Получить все кинотеатры
pub async fn get_cinemas(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let cinemas = sqlx::query_as::<_, Cinema>("SELECT * FROM cinema")
        .fetch_all(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(cinemas))
}

// Создать новый кинотеатр
pub async fn create_cinema(
    pool: web::Data<PgPool>,
    new_cinema: web::Json<NewCinema>,
) -> Result<HttpResponse, AppError> {
    let opening_time = NaiveTime::parse_from_str(&new_cinema.opening_time, "%H:%M:%S");
    let closing_time = NaiveTime::parse_from_str(&new_cinema.closing_time, "%H:%M:%S");

    let cinema = sqlx::query_as::<_, Cinema>(
        "INSERT INTO cinema (name, address, employee_count, hall_count, opening_time, closing_time)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING *",
    )
    .bind(&new_cinema.name)
    .bind(&new_cinema.address)
    .bind(new_cinema.employee_count)
    .bind(new_cinema.hall_count)
    .bind(opening_time.unwrap())
    .bind(closing_time.unwrap())
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(cinema))
}

// Аналогично для других обработчиков (фильмы, сеансы, продажи и т.д.)
