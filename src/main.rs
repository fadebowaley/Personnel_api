use actix_web::{web, App, HttpServer};
use mongodb::Collection;
use crate::db::Database;
use crate::models::user::User;
use crate::models::employee::Employee;
use crate::routes::{user_routes, employee_routes};

mod models;
mod db;
mod handlers;
mod routes;
mod services;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = Database::connect("mongodb://localhost:27017").await.unwrap();

    // User and Employee collections
    let user_collection: Collection<User> = database.get_database("rustdb").collection("users");
    let employee_collection: Collection<Employee> = database.get_database("rustdb").collection("employees");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_collection.clone()))
            .app_data(web::Data::new(employee_collection.clone()))
            .configure(user_routes::configure)
            .configure(employee_routes::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
