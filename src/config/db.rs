use log::error;
use mongodb::Client;
use std::borrow::ToOwned;
use std::env;

// MongoDB initialize function.
// Get DB connection url from environment file and connect.
pub async fn init() -> Client {
    let uri = match env::var("MONGO.URI") {
        Ok(uri) => uri,
        Err(_) => {
            error!("Error loading env info for MongoDB connection");
            "Error loading env variables to connect to MongoDB".to_owned()
        }
    };

    // panic if not able to connect to DB.
    let client = Client::with_uri_str(uri)
        .await
        .expect("Error connecting to backend database");
    client
}
