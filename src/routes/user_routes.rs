use actix_web::web;
use crate::handlers::user_handler;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(user_handler::create_user))
            .route(web::get().to(user_handler::read_users)),
    )
    .service(
        web::resource("/users/{id}")
            .route(web::put().to(user_handler::update_user))
            .route(web::delete().to(user_handler::delete_user)),
    );
}

