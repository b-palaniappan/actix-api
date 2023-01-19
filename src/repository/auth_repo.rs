use actix_web::web::Data;
use mongodb::results::InsertOneResult;
use mongodb::{error::Error, Client};

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
