use crate::models::{post_model::Post, task_model::Task};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Team {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_tasks: Option<Vec<Task>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub posts: Option<Vec<Post>>,

    pub color: String,
}
