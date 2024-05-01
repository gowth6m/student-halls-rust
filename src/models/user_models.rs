use serde::{ Serialize, Deserialize };
use mongodb::bson::{ oid::ObjectId, doc };

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    email: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    password: String,
    year_of_study: i32,
    university: Option<ObjectId>, // Reference to a University object
    reviews: Vec<ObjectId>, // References to Review objects
    profile_pic_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub year_of_study: i32,
    pub university: Option<ObjectId>,
    pub profile_pic_url: Option<String>,
}

impl User {
    pub fn to_new_user(&self) -> NewUser {
        NewUser {
            name: self.name.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
            year_of_study: self.year_of_study,
            university: self.university.clone().into(),
            profile_pic_url: self.profile_pic_url.clone().into(),
        }
    }
}

impl NewUser {
    pub fn to_user(&self) -> User {
        User {
            id: None, // New users typically won't have an ID until saved to DB
            name: self.name.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
            year_of_study: self.year_of_study,
            university: self.university.clone(),
            reviews: Vec::new(), // Assuming no reviews at the point of creation
            profile_pic_url: self.profile_pic_url.clone().unwrap_or_else(|| String::new()),
        }
    }
}
