use serde::{ Serialize, Deserialize };
use mongodb::bson::{ oid::ObjectId, doc };
#[derive(Debug, Serialize, Deserialize)]

pub struct Hall {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    university_id: ObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    reviews: Option<Vec<ObjectId>>,
    location: String,
    postcode: String,
    cover_img_url: String,
    img_urls: Vec<String>,
}
