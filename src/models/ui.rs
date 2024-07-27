use askama::Template;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::schema::users::{ password};

#[derive(Template)]
#[template(path="index.html")]
pub struct HomepageTemplate {
    pub error: Option<String>
}

#[derive(Template)]
#[template(path="login.html")]
pub struct LoginTemplate {
    pub error: Option<String>
}


#[derive(Template)]
#[template(path="register.html")]
pub struct RegisterTemplate {
    pub error: Option<String>
}

#[derive(Template)]
#[template(path="dashboard.html")]
pub struct DashboardTemplate {
    pub error: Option<String>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserForm {
    pub name:  String,
    pub email: String,
    pub password: String,


}

#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    pub  email: String,
    pub  password: String,
}


