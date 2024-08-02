mod controllers;
mod db_operations;
mod models;
mod schema;

use std::env;
use std::sync::Mutex;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};

use actix_web::{cookie::{SameSite, Key},web, App, HttpServer, Responder, HttpResponse};
use askama::filters::format;
use dotenvy::dotenv;
use actix_files as fs;


use models::app_state::AppState;
use controllers::users::{ login_page, register_page, login_user, register_user, home_page };
use db_operations::db::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").expect("HOST not found");
    let port = env::var("PORT").expect("PORT not found");
    let host_port = format!("{}:{}",host, port);


    HttpServer::new(
        move || {
            let app_state = web::Data::new(AppState{
                db_connection:Mutex::new(establish_connection())
            });

            App::new().app_data(app_state.clone())
                .service(fs::Files::new("/static", "./static").show_files_listing())
                .route("/", web::get().to(home_page))
                .route("/login", web::get().to(login_page))
                .route("/login", web::post().to(login_user))
                .route("/register", web::get().to(register_page))
                .route("/register", web::post().to(register_user))
        }).bind(host_port)?
        .run()
        .await


}