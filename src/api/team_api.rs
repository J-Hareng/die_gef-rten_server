use crate::{
    models::{
        team_model::Team,
        user_model::{User, Userlogin},
    },
    repository::mongodb_repo::MongoRepo,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
// use env_logger::fmt;
use mongodb::bson::oid::ObjectId;

//basics

#[post("/add_team")]
pub async fn create_team(db: Data<MongoRepo>, new_team: Json<Team>) -> HttpResponse {
    let data = Team {
        id: None,
        name: new_team.name.to_owned(),
        members: new_team.members.to_owned(),
        tasks: None,
        done_tasks: None,
        posts: None,
        color: new_team.color.to_owned(),
    };
    println!("{:?}", &data);
    let team_detail = db.create_team(data).await;
    match team_detail {
        Ok(team_detail) => HttpResponse::Ok().json(team_detail),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// #[get("/user/{id}")]
// pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
// let id = path.into_inner();
// println!("id_got: {}", &id);
// if id.is_empty() {
// return HttpResponse::BadRequest().body("invalid ID");
// }
// let user_detail = db.get_user(&id).await;
// match user_detail {
// Ok(user) => HttpResponse::Ok().json(user),
// Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
// }
// }

// #[put("/user/{id}")]
// pub async fn update_user(
//     db: Data<MongoRepo>,
//     path: Path<String>,
//     new_user: Json<User>,
// ) -> HttpResponse {
//     let id = path.into_inner();
//     if id.is_empty() {
//         return HttpResponse::BadRequest().body("invalid ID");
//     };
//     let data = User {
//         id: Some(ObjectId::parse_str(&id).unwrap()),
//         name: new_user.name.to_owned(),
//         teams: new_user.teams.to_owned(),

//         is_active: new_user.is_active.to_owned(),
//         is_admin: new_user.is_admin.to_owned(),
//         password: new_user.password.to_owned(),
//     };
//     let update_result = db.update_user(&id, data).await;
//     match update_result {
//         Ok(update) => {
//             if update.matched_count == 1 {
//                 let updated_user_info = db.get_user(&id).await;
//                 return match updated_user_info {
//                     Ok(user) => HttpResponse::Ok().json(user),
//                     Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//                 };
//             } else {
//                 return HttpResponse::NotFound().body("No user found with specified ID");
//             }
//         }
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
// }
// }
#[delete("/team/{id}")]
pub async fn delete_team(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_team(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Team successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("User with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[get("/teams")]
pub async fn get_all_teams(db: Data<MongoRepo>) -> HttpResponse {
    let team = db.get_all_teams().await;
    match team {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
    // }
    // #[get("/users_taken/{name}")]
    // pub async fn users_taken(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    //     let name = path.into_inner();
    //     let users = db.get_user_logindb(name).await;
    //     match users {
    //         Ok(users) => {
    //             print!("Usersfund: {:?}", users);
    //             return HttpResponse::Ok().body("1");
    //         }
    //         Err(err) => {
    //             if err == "User not found" {
    //                 return HttpResponse::Ok().body("0");
    //             } else {
    //                 println!("{}", err);
    //                 return HttpResponse::InternalServerError().body(err.to_string());
    //             }
    //         }
    //     }
}
// #[post("/user_login")]
// pub async fn get_user_login(db: Data<MongoRepo>, new_user: Json<Userlogin>) -> HttpResponse {
//     println!("IIID login");
//     let user_data = Userlogin {
//         name: new_user.name.to_owned(),
//         password: new_user.password.to_owned(),
//     };
//     println!("{:#?}", user_data);
//     let user_detail = db.get_user_logindb(user_data.name.clone()).await;
//     match user_detail {
//         Ok(user) => {
//             if user_data.password == user.password {
//                 println!("{:?}", user);
//                 HttpResponse::Ok().json(user)
//             } else {
//                 HttpResponse::NotFound().json(user_data)
//             }
//         }
//         Err(err) => HttpResponse::InternalServerError().body(err),
//     }
// }
