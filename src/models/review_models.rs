use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, doc};

use super::user_models::User;


#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    user: User,
    room_rating: f64,
    building_rating: f64,
    social_rating: f64,
    location_rating: f64,
    bathroom_rating: f64,
    year_of_study: i32,
    content: String,
    image_urls: Vec<String>,
    verified: bool,
}