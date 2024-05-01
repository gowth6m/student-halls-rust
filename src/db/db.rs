use bson::Document;
use mongodb::{ Client, Database, Collection, options::ClientOptions };
use once_cell::sync::Lazy;
use std::env;

use crate::models::university_models::University;
use crate::models::hall_models::Hall;
use crate::models::user_models::User;

static MONGO_URI: Lazy<String> = Lazy::new(|| {
    env::var("MONGO_URI").expect("Expected MONGO_URI to be set in the environment")
});
static MONGO_DB_NAME: Lazy<String> = Lazy::new(|| {
    env::var("MONGO_DB_NAME").expect("Expected MONGO_DB_NAME to be set in the environment")
});

pub struct MongoConnection {
    db: Database,
    pub collections: Collections,
}

pub struct Collections {
    pub university: Collection<University>,
    pub hall: Collection<Hall>,
    pub user: Collection<User>,
}

impl MongoConnection {
    pub async fn new() -> mongodb::error::Result<Self> {
        let client_options = ClientOptions::parse(&*MONGO_URI).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(&*MONGO_DB_NAME);

        let university_collection = db.collection::<University>("university");
        let hall_collection = db.collection::<Hall>("hall");
        let user_collection = db.collection::<User>("user");

        Ok(Self {
            db,
            collections: Collections {
                university: university_collection,
                hall: hall_collection,
                user: user_collection,
            },
        })
    }

    // To get a collection from the database
    pub fn get_collection(&self, coll_name: &str) -> Collection<Document> {
        self.db.collection::<Document>(coll_name)
    }
}
