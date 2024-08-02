use askama::Template;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path="index.html")]
pub struct HomepageTemplate {
    pub error: Option<String>
}

#[derive(Template)]
#[template(path="login.html")]
pub struct LoginTemplate{
    pub(crate) error: Option<String>,
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




#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    pub  username: String,
    pub  password: String,
}


