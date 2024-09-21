use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, doc};

// Department Transfer Request
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentTransfer {
    pub id: Option<ObjectId>,                  // Unique identifier for the transfer request
    pub employee_id: ObjectId,                 // Reference to the Employee
    pub from_department_id: ObjectId,         // ID of the department being transferred from
    pub to_department_id: ObjectId,           // ID of the department being transferred to
    pub date_requested: String,                // Date of request
    pub status: String,                        // Status of the request (e.g., approved, pending)
}
