use serde::{ Serialize, Deserialize };
use mongodb::bson::{ oid::ObjectId, doc };

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    email: String,
    password: String, // TODO - Hash this password
    year_of_study: i32,
    university: ObjectId, // Reference to a University object
    reviews: Vec<ObjectId>, // References to Review objects
    profile_pic_url: String,
}
