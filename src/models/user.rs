use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, doc};

// User role enumeration
#[derive(Debug, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Staff,
}


// User struct representing application users
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<ObjectId>,
    pub email: String,
    pub username: String,
    pub password: String,
    pub role: UserRole,
}

