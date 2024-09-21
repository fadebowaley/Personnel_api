use actix_web::{web, HttpResponse};
use crate::models::user::User;
use crate::services::user_services;
use crate::db::repositories::user_repo::UserRepo; 

pub async fn create_user(repo: web::Data<UserRepo>, user: web::Json<User>) -> HttpResponse {
    match user_services::create_user(user.into_inner(), &repo).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn read_users(repo: web::Data<UserRepo>) -> HttpResponse {
    match user_services::get_all_users(&repo).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn update_user(repo: web::Data<UserRepo>, user_id: web::Path<String>, user: web::Json<User>) -> HttpResponse {
    match user_services::update_user(user_id.into_inner(), user.into_inner(), &repo).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn delete_user(repo: web::Data<UserRepo>, user_id: web::Path<String>) -> HttpResponse {
    match user_services::delete_user(user_id.into_inner(), &repo).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
