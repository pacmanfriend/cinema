use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketSaleResponse {
    pub sale_id: i32,
    pub session_id: i32,
    pub customer_id: i32,
    pub employee_id: i32,
    pub ticket_count: i32,
    pub sale_time: String,
    pub total_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SalesStatsResponse {
    pub total_sales: i64,
    pub total_revenue: f64,
    pub avg_tickets_per_sale: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateTicketSaleRequest {
    pub session_id: i32,
    pub customer_id: i32,
    pub employee_id: i32,
    pub ticket_count: i32,
}

pub async fn create_ticket_sale(
    pool: web::Data<PgPool>,
    new_sale: web::Json<CreateTicketSaleRequest>,
) -> Result<HttpResponse, AppError> {
    let sale = sqlx::query_as!(
        TicketSaleResponse,
        r#"
        WITH inserted AS (
            INSERT INTO ticket_sale (session_id, customer_id, employee_id, ticket_count)
            VALUES ($1, $2, $3, $4)
            RETURNING
                sale_id,
                session_id,
                customer_id,
                employee_id,
                ticket_count,
                sale_time::text
        )
        SELECT
            i.sale_id,
            i.session_id,
            i.customer_id,
            i.employee_id,
            i.ticket_count,
            i.sale_time,
            (i.ticket_count * s.ticket_price) as total_price
        FROM inserted i
        JOIN session s ON i.session_id = s.session_id
        "#,
        new_sale.session_id,
        new_sale.customer_id,
        new_sale.employee_id,
        new_sale.ticket_count
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(sale))
}

pub async fn get_sales_stats(pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let stats = sqlx::query_as!(
        SalesStatsResponse,
        r#"
        SELECT
            COUNT(*) as total_sales,
            COALESCE(SUM(ticket_count * s.ticket_price), 0) as total_revenue,
            COALESCE(AVG(ticket_count), 0) as avg_tickets_per_sale
        FROM ticket_sale ts
        JOIN session s ON ts.session_id = s.session_id
        "#
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(stats))
}

pub async fn get_ticket_sale(
    pool: web::Data<PgPool>,
    sale_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let sale = sqlx::query_as!(
        TicketSaleResponse,
        r#"
        SELECT
            ts.sale_id,
            ts.session_id,
            ts.customer_id,
            ts.employee_id,
            ts.ticket_count,
            ts.sale_time::text,
            (ts.ticket_count * s.ticket_price) as total_price
        FROM ticket_sale ts
        JOIN session s ON ts.session_id = s.session_id
        WHERE ts.sale_id = $1
        "#,
        sale_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await?;

    match sale {
        Some(s) => Ok(HttpResponse::Ok().json(s)),
        None => Err(AppError::NotFound("Ticket sale not found".into())),
    }
}
