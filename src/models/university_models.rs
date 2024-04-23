use serde::{ Serialize, Deserialize };
use mongodb::bson::{ oid::ObjectId, doc };

#[derive(Debug, Serialize, Deserialize)]
pub struct University {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    halls: Option<Vec<ObjectId>>,
    cover_img_url: String,
    location: String,
}
