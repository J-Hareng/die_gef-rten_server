use crate::models::user_model::User;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Task {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker: Option<Vec<User>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<String>,

    pub timestamp: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_timestamp: Option<i32>,

    pub done: bool,
}
