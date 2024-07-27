use actix_web::{HttpResponse, Responder, web};
use askama::Template;
use crate::models::app_state::AppState;
use crate::models::ui::{LoginTemplate, RegisterTemplate, DashboardTemplate, HomepageTemplate,NewUserForm};
use crate::models::ui::{LoginForm};
use bcrypt::{hash, DEFAULT_COST};
use crate::db_operations::users::add_user;
use crate::models::users::NewUser;

async fn handle_register(error: &str) -> HttpResponse {
    let template = LoginTemplate { error: Some(error.to_string())  };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

async fn handle_error(error: &str) -> HttpResponse {
    let template = LoginTemplate { error: Some(error.to_string()) };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

pub async fn home_page(error: Option<String>) -> impl Responder {

    let template = HomepageTemplate { error };
    HttpResponse::Ok().content_type(
        "text/html"
    ).body(template.render().unwrap())

}
pub async fn login_page(error: Option<String>) -> impl Responder {

    let template = LoginTemplate { error };

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

    let template = LoginTemplate { error : Some("To be implemented".to_string()) };

    HttpResponse::Ok().content_type(
        "text/html"
    ).body(template.render().unwrap())
}

pub async fn register_user(item: web::Form<NewUserForm>, state: web::Data<AppState>) -> impl Responder {
    if item.name.is_empty() || item.email.is_empty()  || item.password.is_empty() {
        return HttpResponse::BadRequest().body("All fields are required");
    }
    // Hash and salt the password
    let hashed_password = match hash(&item.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => return HttpResponse::InternalServerError().body("Error hashing password"),
    };


    let new_user = NewUser {
        first_name: item.name.clone(),
        last_name: item.name.clone(),
        email: item.email.clone(),


    };

    // access db connection
    let mut connection = state.db_connection.lock().unwrap();

    let user = add_user(new_user, &mut *connection);

    match user {
        Ok(user) => {
            return handle_register("Account has been created").await;
        }
        Err(err) => {
            return handle_error("Error creating an account").await;
        }
    }


}


