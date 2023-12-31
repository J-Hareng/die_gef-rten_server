use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<String>>,

    pub is_active: bool,
    pub is_admin: bool,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Userlogin {
    pub name: String,
    pub password: String,
}
