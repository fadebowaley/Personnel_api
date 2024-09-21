use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, doc};


// Reference Letter Request
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceLetterRequest {
    pub id: Option<ObjectId>,                  // Unique identifier for the request
    pub employee_id: ObjectId,                 // Reference to the Employee
    pub date_requested: String,                // Date of request
    pub status: String,                        // Status of the request (e.g., approved, pending)
}