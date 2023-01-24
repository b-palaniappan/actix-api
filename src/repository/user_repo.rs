use actix_web::web::Data;
use futures::TryStreamExt;
use mongodb::{
    bson::doc,
    error::Error,
    results::{DeleteResult, UpdateResult},
    Client, Collection,
};
use nanoid::nanoid;

use crate::{constants, models::user_model::User};

// Add a new user to Mongo DB.
pub async fn create_user(client: &Data<Client>, new_user: User) -> Result<Option<User>, Error> {
    let new_doc = User {
        id: Some(nanoid!()),
        name: new_user.name,
        location: new_user.location,
        title: new_user.title,
    };
    let collection = client
        .database(constants::MONGO_DATABASE)
        .collection(constants::MONGO_USER_COLLECTION);
    let added_user = collection.insert_one(new_doc, None).await;
    // On successful add.. Retrieve the added record as response.
    match added_user {
        Ok(u) => collection.find_one(doc! {"_id": u.inserted_id}, None).await,
        Err(err) => Err(err),
    }
}

// Get a user by given id from MongoDB database
pub async fn get_user(client: &Data<Client>, id: &String) -> Result<Option<User>, Error> {
    let obj_id = String::from(id);
    let filter = doc! {"_id": obj_id};
    let collection = client
        .database(constants::MONGO_DATABASE)
        .collection(constants::MONGO_USER_COLLECTION);
    collection.find_one(filter, None).await
}

// Update a user for give unique user id.
pub async fn update_user(
    client: &Data<Client>,
    id: &String,
    new_user: User,
) -> Result<UpdateResult, Error> {
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
    let collection: Collection<User> = client
        .database(constants::MONGO_DATABASE)
        .collection(constants::MONGO_USER_COLLECTION);
    collection.update_one(filter, new_doc, None).await
}

// Delete a user for given unique user id.
pub async fn delete_user(client: &Data<Client>, id: &String) -> Result<DeleteResult, Error> {
    let obj_id = String::from(id);
    let filter = doc! {"_id": obj_id};
    let collection: Collection<User> = client
        .database(constants::MONGO_DATABASE)
        .collection(constants::MONGO_USER_COLLECTION);
    collection.delete_one(filter, None).await
}

// Fetch all users from the database
pub async fn get_all_users(client: &Data<Client>) -> Result<Vec<User>, Error> {
    let collection: Collection<User> = client
        .database(constants::MONGO_DATABASE)
        .collection(constants::MONGO_USER_COLLECTION);
    let mut cursors = collection.find(None, None).await?;
    let mut users: Vec<User> = Vec::new();
    while let Some(user) = cursors.try_next().await? {
        users.push(user)
    }
    Ok(users)
}
