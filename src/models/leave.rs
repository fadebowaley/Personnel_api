use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, doc};

// Leave Request
#[derive(Debug, Serialize, Deserialize)]
pub struct LeaveRequest {
    pub id: Option<ObjectId>,                  // Unique identifier for the leave request
    pub employee_id: ObjectId,                 // Reference to the Employee
    pub leave_type: String,                    // Type of leave (e.g., annual, maternity, sick)
    pub start_date: String,                    // Start date of the leave
    pub end_date: String,                      // End date of the leave
    pub status: String,                        // Status of the request (e.g., approved, pending)
}
