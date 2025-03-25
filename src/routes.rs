use actix_web::web;

mod bookings;
mod cinemas;
mod films;
mod sessions;
mod tickets;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Кинотеатры
            .service(
                web::scope("/cinemas")
                    .route("", web::get().to(cinemas::get_cinemas))
                    .route("", web::post().to(cinemas::create_cinema))
                    .route("/{id}", web::get().to(cinemas::get_cinema))
                    .route("/{id}", web::put().to(cinemas::update_cinema))
                    .route("/{id}", web::delete().to(cinemas::delete_cinema)),
            )
            // Фильмы
            .service(
                web::scope("/films")
                    .route("", web::get().to(films::get_films))
                    .route("", web::post().to(films::create_film))
                    .route("/active", web::get().to(films::get_active_films))
                    .route("/{id}", web::get().to(films::get_film))
                    .route("/{id}", web::put().to(films::update_film))
                    .route("/{id}", web::delete().to(films::delete_film)),
            )
            // Сеансы
            .service(
                web::scope("/sessions")
                    .route("", web::get().to(sessions::get_sessions))
                    .route("", web::post().to(sessions::create_session))
                    .route("/upcoming", web::get().to(sessions::get_upcoming_sessions))
                    .route("/{id}", web::get().to(sessions::get_session))
                    .route("/{id}", web::delete().to(sessions::delete_session)),
            )
            // Продажи билетов
            .service(
                web::scope("/tickets")
                    .route("", web::post().to(tickets::create_ticket_sale))
                    .route("/stats", web::get().to(tickets::get_sales_stats))
                    .route("/{id}", web::get().to(tickets::get_ticket_sale)),
            )
            // Бронирования
            .service(
                web::scope("/bookings")
                    .route("", web::get().to(bookings::get_bookings))
                    .route("", web::post().to(bookings::create_booking))
                    .route("/active", web::get().to(bookings::get_active_bookings))
                    .route("/{id}/confirm", web::put().to(bookings::confirm_booking))
                    .route("/{id}/cancel", web::put().to(bookings::cancel_booking))
                    .route("/{id}", web::get().to(bookings::get_booking)),
            ),
    );
}
