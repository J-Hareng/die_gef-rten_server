use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::team_model::Team;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_teams: Option<Vec<Team>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub img: Option<String>,

    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Userlogin {
    pub name: String,
    pub password: String,
}
