use actix_web::{web, HttpRequest, HttpResponse, Responder};
use askama::Template;

use crate::models::ui::HomepageTemplate;

pub async fn default_handler(req: HttpRequest) -> impl Responder {

    let template = HomepageTemplate { error: None };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}