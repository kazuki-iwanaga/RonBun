use crate::schema::users;

use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub user_email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub user_name: String,
    pub user_email: String,
}
