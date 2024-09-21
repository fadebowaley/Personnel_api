use actix_web::{web, HttpResponse};
use crate::models::employee::Employee;
use crate::services::employee_services;
use crate::db::repositories::employee_repo::EmployeeRepo; // Import EmployeeRepo

pub async fn create_employee(repo: web::Data<EmployeeRepo>, employee: web::Json<Employee>) -> HttpResponse {
    match employee_services::create_employee(employee.into_inner(), &repo).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn read_employees(repo: web::Data<EmployeeRepo>) -> HttpResponse {
    match employee_services::get_all_employees(&repo).await {
        Ok(employees) => HttpResponse::Ok().json(employees),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn update_employee(repo: web::Data<EmployeeRepo>, employee_id: web::Path<String>, employee: web::Json<Employee>) -> HttpResponse {
    match employee_services::update_employee(employee_id.into_inner(), employee.into_inner(), &repo).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn delete_employee(repo: web::Data<EmployeeRepo>, employee_id: web::Path<String>) -> HttpResponse {
    match employee_services::delete_employee(employee_id.into_inner(), &repo).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
