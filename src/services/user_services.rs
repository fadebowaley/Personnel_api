use crate::db::repositories::user_repo::UserRepo;
use crate::models::user::User;
use mongodb::error::Error;

pub async fn create_user(user: User, repo: &UserRepo) -> Result<(), Error> {
    repo.create(user).await
}

pub async fn get_all_users(repo: &UserRepo) -> Result<Vec<User>, Error> {
    repo.find_all().await
}

pub async fn update_user(user_id: String, user: User, repo: &UserRepo) -> Result<(), Error> {
    repo.update(user_id, user).await
}

pub async fn delete_user(user_id: String, repo: &UserRepo) -> Result<(), Error> {
    repo.delete(user_id).await
}
