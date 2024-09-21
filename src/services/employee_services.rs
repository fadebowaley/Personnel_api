use crate::db::repositories::employee_repo::EmployeeRepo; // Import EmployeeRepo
use crate::models::employee::Employee;
use mongodb::error::Error;

pub async fn create_employee(employee: Employee, repo: &EmployeeRepo) -> Result<(), Error> {
    repo.create(employee).await
}

pub async fn get_all_employees(repo: &EmployeeRepo) -> Result<Vec<Employee>, Error> {
    repo.find_all().await
}

pub async fn update_employee(employee_id: String, employee: Employee, repo: &EmployeeRepo) -> Result<(), Error> {
    repo.update(employee_id, employee).await
}

pub async fn delete_employee(employee_id: String, repo: &EmployeeRepo) -> Result<(), Error> {
    repo.delete(employee_id).await
}
