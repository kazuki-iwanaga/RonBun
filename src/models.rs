use crate::schema::users;
use crate::schema::papers;

use chrono::NaiveDateTime;


#[derive(Debug, Queryable)]
pub struct Paper {
    pub paper_id: i32,
    pub paper_title: String,
    pub paper_author: String,
    pub paper_year: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "papers"]
pub struct NewPaper {
    pub paper_title: String,
    pub paper_author: String,
    pub paper_year: i32,
    pub user_id: i32,
}


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
