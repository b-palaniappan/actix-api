use actix_web::web::Data;
use bson::doc;
use mongodb::results::InsertOneResult;
use mongodb::{error::Error, Client, Collection};

use crate::{constants, models::auth_model::Auth};

// Add a user to auth table with hash password.
pub async fn auth_register(
    client: &Data<Client>,
    register_user: Auth,
) -> Result<InsertOneResult, Error> {
    let collection = client
        .database(constants::MONGO_DATABASE)
        .collection(constants::MONGO_AUTH_COLLECTION);
    collection.insert_one(register_user, None).await
}

// Check if user with email already esitst in auth table or not.
pub async fn check_email(client: &Data<Client>, email: &String) -> bool {
    let collection: Collection<Auth> = client
        .database(constants::MONGO_DATABASE)
        .collection(constants::MONGO_AUTH_COLLECTION);
    let count = collection
        .count_documents(doc! {"email": email}, None)
        .await;
    match count {
        Ok(c) => c == 0,
        Err(_) => true,
    }
}

// Fetch user from auth table based on email id for authentication with credentials.
pub async fn fetch_by_email(client: &Data<Client>, email: &String) -> Option<Auth> {
    let collection: Collection<Auth> = client
        .database(constants::MONGO_DATABASE)
        .collection(constants::MONGO_AUTH_COLLECTION);
    let auth = collection.find_one(doc! {"email": email}, None).await;
    match auth {
        Ok(a) => a,
        Err(_) => None,
    }
}
