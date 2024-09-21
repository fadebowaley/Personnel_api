use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, doc};
use crate::models::user::UserRole;

// Employee status enumeration
#[derive(Debug, Serialize, Deserialize)]
pub enum EmployeeStatus {
    Active,
    Resigned,
    Demised,
}

// Employee struct representing employee records
#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub id: Option<ObjectId>,                  // Unique identifier for the employee
    pub name: String,                          // Employee's full name
    pub age: u32,                              // Employee's age
    pub department_id: ObjectId,               // Reference to the Department
    pub salary: f64,                           // Employee's salary
    pub role: UserRole,                        // Role of the employee in the organization
    pub email: String,                         // Employee's email address
    pub phone: String,                         // Employee's phone number
    pub hire_date: String,                     // Date the employee was hired
    pub address: String,                       // Home address of the employee
    pub supervisor_id: Option<ObjectId>,       // ID of the employee's supervisor (another Employee)
    pub employment_type: String,               // Type of employment (e.g., full-time, part-time, contract)
    pub skills: Vec<String>,                   // List of skills the employee possesses
    pub projects: Vec<ObjectId>,               // List of project IDs the employee is involved in
    pub status: EmployeeStatus,                 // Current status of the employee
}
