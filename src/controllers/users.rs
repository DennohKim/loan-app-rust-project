use actix_web::{HttpResponse, Responder, web};
use askama::Template;
use crate::models::app_state::AppState;
use crate::models::ui::{LoginTemplate, RegisterTemplate, DashboardTemplate, HomepageTemplate,};
use crate::models::ui::{LoginForm};
use bcrypt::{hash, DEFAULT_COST};
use crate::models::users::{NewUser, NewUserForm, Users};

async fn handle_register_error(error: &str) -> HttpResponse {
    let template = RegisterTemplate { error: Some(error.to_string()) };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

async fn handle_login_information(error: &str) -> HttpResponse {
    let template = LoginTemplate { error: None, message: Some(error.to_string()) };
    HttpResponse::Ok().content_type("text/html").append_header((actix_web::http::header::LOCATION, "/dashboard")).body(template.render().unwrap())
}

pub async fn home_page(error: Option<String>) -> impl Responder {

    let template = HomepageTemplate { error };
    HttpResponse::Ok().content_type(
        "text/html"
    ).body(template.render().unwrap())

}
pub async fn login_page(error: Option<String>, message: Option<String>) -> impl Responder {

    let template = LoginTemplate { error, message };

    HttpResponse::Ok().content_type(
        "text/html"
    ).body(template.render().unwrap())

}


pub async fn register_page(error: Option<String>) -> impl Responder {

    let template = RegisterTemplate { error };

    HttpResponse::Ok().content_type(
        "text/html"
    ).body(template.render().unwrap())



}

pub async fn login_user(form: web::Form<LoginForm>, state : web::Data<AppState>) -> impl Responder {

    let template = LoginTemplate { error : Some("To be implemented".to_string()), message: None };

    HttpResponse::Ok().content_type(
        "text/html"
    ).body(template.render().unwrap())
}

pub async fn register_user(item: web::Form<NewUserForm>, state: web::Data<AppState>) -> impl Responder {
    if item.username.is_empty() ||  item.first_name.is_empty() ||  item.last_name.is_empty() ||  item.email.is_empty() ||  item.address.is_empty()  || item.confirm_password.is_empty() || item.password.is_empty() {
        println!("Empty fields detected");
        return HttpResponse::BadRequest().body("All fields are required");
    }
    println!("All fields have content");

    let hashed_password = match hash(&item.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(e) => {
            println!("Error");
            return handle_register_error("error hashing password").await;
        }
    };

    let new_user = NewUser {
        username: item.username.clone(),
        first_name: item.first_name.clone(),
        last_name: item.last_name.clone(),
        email: item.email.clone(),
        address: item.address.clone(),
        date_of_birth: item.date_of_birth.clone(),
        password: hashed_password,
        confirm_password: hashed_password,
  

    };

    let mut connection = state.db_connection.lock().unwrap();
    let res= diesel::insert_into(crate::schema::users::table).values(&new_user).get_result::<Users>(&mut *connection);
    match res {
        Ok(_) => {
            return handle_login_information("Account created, please login to continue").await;
        }
        Err(err) => {
            println!("db error {:#?}", err);
            return handle_register_error("error creating account").await;
        }
    }


}


