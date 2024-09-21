use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, doc};


// Promotional Exam Attendance
#[derive(Debug, Serialize, Deserialize)]
pub struct PromotionalExam {
    pub id: Option<ObjectId>,                  // Unique identifier for the exam record
    pub employee_id: ObjectId,                 // Reference to the Employee
    pub exam_date: String,                     // Date of the exam
    pub status: String,                        // Status of the exam (e.g., completed, pending)
}
