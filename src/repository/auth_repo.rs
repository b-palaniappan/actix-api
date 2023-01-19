use actix_web::web::Data;
use bson::doc;
use mongodb::results::InsertOneResult;
use mongodb::{error::Error, Client, Collection};

use crate::constants;
use crate::models::auth_model::Auth;

pub async fn auth_register(
    client: &Data<Client>,
    register_user: Auth,
) -> Result<InsertOneResult, Error> {
    let collection = client
        .database(constants::MONGO_DATABASE)
        .collection(constants::MONGO_AUTH_COLLECTION);
    collection.insert_one(register_user, None).await
}

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
