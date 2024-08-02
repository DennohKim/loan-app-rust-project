use diesel::PgConnection;
use diesel::prelude::*;
use crate::models::users::NewUser;
use crate::models::users::Users;
use crate::schema::users::dsl::*;

pub fn get_user_by_id(connection: &mut PgConnection, user_id: i32) -> Option<Users> {
    users.filter(id.eq(user_id)).first::<Users>(connection).optional().unwrap_or_else(|err| {
        println!("Error occurred: {:?}", err);
        None
    })
}
pub fn get_user_by_email(connection: &mut PgConnection, user_email: String) -> Option<Users> {
    users.filter(email.eq(user_email)).first::<Users>(connection).optional().unwrap_or_else(|err| {
        println!("Error occured: {:?}", err);
        None
    })
}

pub fn get_user_by_username(connection: &mut PgConnection, user_name: String) -> Option<Users> {
    users.filter(username.eq(user_name)).first::<Users>(connection).optional().unwrap_or_else(|err| {
        println!("Error occured: {:?}", err);
        None
    })
}


pub fn add_user(new_user: NewUser, connection: &mut PgConnection) -> Result<Users, diesel::result::Error> {
    diesel::insert_into(crate::schema::users::table)
       .values(&new_user)
       // .returning(Post::as_returning())
       .get_result::<Users>(connection)

}