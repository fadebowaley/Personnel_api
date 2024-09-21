// src/db/connection.rs

use mongodb::{Client, Database as MongoDatabase};

pub struct Database {
    pub client: Client,
}

impl Database {
    pub async fn connect(uri: &str) -> Result<Self, mongodb::error::Error> {
        let client = Client::with_uri_str(uri).await?;
        Ok(Self { client })
    }

    pub fn get_database(&self, name: &str) -> MongoDatabase {
        self.client.database(name)
    }
}
