use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, doc};

// Department struct representing organizational departments
#[derive(Debug, Serialize, Deserialize)]
pub struct Department {
    pub id: Option<ObjectId>,                  // Unique identifier for the department
    pub name: String,                          // Department name
    pub location: String,                       // Location of the department
}
