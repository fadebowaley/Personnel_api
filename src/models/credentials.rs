use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, doc};


#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub id: Option<ObjectId>,                  // Unique identifier for the document
    pub employee_id: ObjectId,                 // Reference to the Employee
    pub document_type: String,                 // Type of document (e.g., certificate, ID)
    pub issue_date: String,                    // Date the document was issued
    pub expiration_date: Option<String>,       // Date the document expires (if applicable)
    pub status: String,                        // Status of the document (e.g., valid, expired)
    pub file_path: String,                     // Path to the document file (e.g., URL or local path)
}
