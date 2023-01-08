use futures::stream::TryStreamExt;
use nanoid::nanoid;
use std::env;

use crate::models::user_model::User;
use mongodb::{
    bson::{doc},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    // MongoDB initialize function.
    // Get DB connection url from environment file and connect.
    pub async fn init() -> Self {
        let uri = match env::var("MONGO.URI") {
            Ok(v) => v,
            Err(_) => "Error loading env variable".to_string(),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    // Add a new user to Mongo DB.
    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: Some(nanoid!()),
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        self
            .col
            .insert_one(new_doc, None)
            .await
    }

    // Get a user by given id from MongoDB database
    pub async fn get_user(&self, id: &String) -> Result<Option<User>, Error> {
        let obj_id = String::from(id);
        let filter = doc! {"_id": obj_id};
        self
            .col
            .find_one(filter, None)
            .await
    }
    
    // Update a user for give unique user id.
    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = String::from(id);
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "_id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };
        self
            .col
            .update_one(filter, new_doc, None)
            .await
    }
    
    // Delete a user for given unique user id.
    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult,Error> {
        let obj_id = String::from(id);
        let filter = doc! {"_id": obj_id};
        self
            .col
            .delete_one(filter, None)
            .await
    }

    // Fetch all users from the database
    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await?;
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await?
        {
            users.push(user)
        }
        Ok(users)
    }
}
