mod api;
mod models;
mod repository;

//#//pswd Mongo db 5q9M41bexXrKwuct
use actix_cors::Cors;
use actix_web::Responder;
use actix_web::{web, HttpResponse};
use actix_web::{web::Data, App, HttpServer};
use api::team_api::{create_team, delete_team, get_all_teams};
use api::user_api::{
    create_user, delete_user, get_all_users, get_user, get_user_login, update_user, users_taken,
}; //import the handler here
use repository::mongodb_repo::MongoRepo;
use std::sync::Mutex;

use std::env;
use std::path::PathBuf;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn test(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard
                   // <- response with count
    println!("Got Request");

    format!("Request number: {counter}")
}

async fn login(data: web::Data<AppStateWithCounter>) -> Option<HttpResponse> {
    let count = data.counter.lock().unwrap();

    Some(HttpResponse::Ok().body(format!("{}", count)))
}
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!(
            r"/home/julian/Desktop/code/die_gefarten_server/server/src/public/build/index.html"
        ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //basic web needs
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    //db conections
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    //For html shit
    let client_dir_login =
        PathBuf::from(r"/home/julian/Desktop/code/die_gefarten_server/server/src/public/build");
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    println!("Server is running at: '127.0.0.1:8080'");
    HttpServer::new(move || {
        let current_satic_dir_login = client_dir_login.clone();

        // move counter into the closure
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            //.wrap(Logger::default())
            .app_data(counter.clone()) // <- register the created data
            .app_data(db_data.clone()) // <- database conection
            //API call USER
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)
            .service(get_user_login)
            .service(users_taken)
            // API call team
            .service(delete_team)
            .service(create_team)
            .service(get_all_teams)
            //test routes
            .route("/test", web::get().to(test))
            .route("/login", web::get().to(login))
            //WEB routes
            //--> login page
            .service(
                actix_files::Files::new("/", &current_satic_dir_login).index_file("index.html"),
            )
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
