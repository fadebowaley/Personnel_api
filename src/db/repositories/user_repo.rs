use mongodb::Collection;
use crate::models::user::User;
use futures::TryStreamExt;
use mongodb::error::Error;
use bson::{doc, oid::ObjectId}; 
//use bson::doc;
pub struct UserRepo {
    collection: Collection<User>,
}

impl UserRepo {

    pub async fn create(&self, user: User) -> Result<(), Error> {
        self.collection.insert_one(user, None).await?;
        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<User>, Error> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut users = Vec::new();

        while let Some(user) = cursor.try_next().await? {
            users.push(user);
        }

        Ok(users)
    }

  pub async fn update(&self, user_id: String, user: User) -> Result<(), Error> {
    // Convert user_id from string to ObjectId
    let object_id = ObjectId::parse_str(&user_id).map_err(|_| {
        Error::from(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid user ID format"))
    })?;

    // Create a filter to find the user by ID
    let filter = doc! { "_id": object_id };

    // Create an update document with the fields you want to update
    let update = doc! { "$set": bson::to_bson(&user).unwrap() };

    // Perform the update operation
    self.collection.update_one(filter, update, None).await?;
    Ok(())
}

    pub async fn delete(&self, user_id: String) -> Result<(), Error> {
        let filter = doc! { "_id": user_id };
        self.collection.delete_one(filter, None).await?;
        Ok(())
    }
}
