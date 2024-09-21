use mongodb::Collection;
use crate::models::employee::Employee;
use futures::TryStreamExt;
use mongodb::error::Error;
use bson::doc;

pub struct EmployeeRepo {
    collection: Collection<Employee>,
}

impl EmployeeRepo {

    pub async fn create(&self, employee: Employee) -> Result<(), Error> {
        self.collection.insert_one(employee, None).await?;
        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<Employee>, Error> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut employees = Vec::new();

        while let Some(employee) = cursor.try_next().await? {
            employees.push(employee);
        }

        Ok(employees)
    }

 pub async fn update(&self, employee_id: String, employee: Employee) -> Result<(), Error> {
    let filter = doc! { "_id": employee_id };
    let update = doc! { "$set": bson::to_bson(&employee).unwrap() }; // Wrap in `$set`

    self.collection.update_one(filter, update, None).await?;
    Ok(())
}

    pub async fn delete(&self, employee_id: String) -> Result<(), Error> {
        let filter = doc! { "_id": employee_id };
        self.collection.delete_one(filter, None).await?;
        Ok(())
    }
}
