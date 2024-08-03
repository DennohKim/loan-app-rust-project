use askama::Template;

use super::users::Users;




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
#[template(path="home.html")]
pub struct DashboardTemplate {
    pub error: Option<String>
}




