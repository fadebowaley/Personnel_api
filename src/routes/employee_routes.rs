use actix_web::web;
use crate::handlers::employee_handler;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/employees")
            .route(web::post().to(employee_handler::create_employee))
            .route(web::get().to(employee_handler::read_employees)),
    )
    .service(
        web::resource("/employees/{id}")
            .route(web::put().to(employee_handler::update_employee))
            .route(web::delete().to(employee_handler::delete_employee)),
    );
}
