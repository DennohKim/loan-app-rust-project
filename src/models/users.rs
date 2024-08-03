use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Users {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_of_birth: chrono::NaiveDate,
    pub address: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserForm {
    pub username:  String,
    pub first_name:  String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub date_of_birth: chrono::NaiveDate,
    pub address: String,
    
}
#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct NewUser {

    pub username:  String,
    pub first_name:  String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub date_of_birth: chrono::NaiveDate,
    pub address: String,

}

#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    pub  username: String,
    pub  password: String,
}