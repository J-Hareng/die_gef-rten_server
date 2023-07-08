use std::env;
extern crate dotenv;
use crate::models::{
    post_model::Post,
    task_model::Task,
    team_model::{self, Team},
    user_model::User,
};
use actix_web::http::header::TE;
use dotenv::dotenv;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId, Bson, Document},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

// impl From<Vec<Task>> for Bson {
//     fn from(tasks: Vec<Task>) -> Self {
//         let mut doc = Document::new();
//         let bson_tasks: Vec<Bson> = tasks
//             .into_iter()
//             .map(|task| {
//                 // Convert each Task object to Bson and add it to the document
//                 // You will need to define the conversion logic based on your Task struct
//                 // For simplicity, let's assume Task has a single field named "name"
//                 Bson::String(task.name) // Convert task.name to Bson::String
//             })
//             .collect();

//         doc.insert("tasks", bson_tasks);
//         Bson::Document(doc)
//     }
// }
pub struct MongoRepo {
    col_user: Collection<User>,
    col_team: Collection<Team>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        let col_user: Collection<User> = db.collection("User");
        let col_team: Collection<Team> = db.collection("Team");
        MongoRepo { col_user, col_team }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name.to_owned(),
            password: new_user.password.to_owned(),
            is_active: new_user.is_active.to_owned(),
            is_admin: new_user.is_admin.to_owned(),
            teams: None,
        };
        let user = self
            .col_user
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }
    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col_user
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }
    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "password": new_user.password,
                    "is_active": new_user.is_active,
                    "is_admin": new_user.is_admin,
                    "teams": new_user.teams,
                },
        };
        let updated_doc = self
            .col_user
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }
    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col_user
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .col_user
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }

    pub async fn get_user_logindb(&self, name: String) -> Result<User, String> {
        let filter = doc! {"name": name};
        let user_detail = self
            .col_user
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");
        return match user_detail {
            Some(user) => Ok(user),
            None => Err(String::from("User not found")),
        };
    }

    // ____________________________
    // < User conection ends here >
    //  -------------------------
    //   \                              _
    //    \                            | \
    //     \                           | |
    //      \                          | |
    //       \    |\                   | |
    //        \  /, ~\                / /
    //          X     `-.....-------./ /
    //           ~-. ~  ~              |
    //              \             /    |
    //               \  /_     ___\   /
    //               | /\ ~~~~~   \ |
    //               | | \        || |
    //               | |\ \       || )
    //              (_/ (_/      ((_/

    pub async fn create_team(&self, team: Team) -> Result<InsertOneResult, Error> {
        let new_doc = Team {
            id: None,
            name: team.name.to_owned(),
            color: team.color.to_owned(),
            members: None,
            tasks: None,
            done_tasks: None,
            posts: None,
        };
        let team = self
            .col_team
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating team");
        Ok(team)
    }

    pub async fn get_all_teams(&self) -> Result<Vec<Team>, Error> {
        let mut cursors = self
            .col_team
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of teams");
        let mut teams: Vec<Team> = Vec::new();
        while let Some(team) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            teams.push(team)
        }
        Ok(teams)
    }
    // pub async fn edit_team(&self, team: Team, id: String) -> Result<DeleteResult, Error> {
    //     let obj_id = ObjectId::parse_str(id).unwrap();
    //     let filter = doc! {"_id": obj_id};
    //     let new_doc = doc! {
    //         "$set":
    //             {
    //                 "id": team.id,
    //                 "name": team.name,
    //                 "members": team.members,
    //                 "tasks": team.tasks,
    //                 "done_tasks": team.done_tasks,
    //             },
    //     };
    //     let updated_doc = self
    //         .col_user
    //         .update_one(filter, new_doc, None)
    //         .await
    //         .ok()
    //         .expect("Error updating user");
    //     Ok(updated_doc)
    // }

    pub async fn delete_team(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let team_detail = self
            .col_team
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(team_detail)
    }
    // ____________________________
    // < Team conection ends here >
    //  -------------------------
    //   \                              _
    //    \                            | \
    //     \                           | |
    //      \                          | |
    //       \    |\                   | |
    //        \  /, ~\                / /
    //          X     `-.....-------./ /
    //           ~-. ~  ~              |
    //              \             /    |
    //               \  /_     ___\   /
    //               | /\ ~~~~~   \ |
    //               | | \        || |
    //               | |\ \       || )
    //              (_/ (_/      ((_/
}
