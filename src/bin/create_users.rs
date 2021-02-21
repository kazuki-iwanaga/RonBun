use diesel::prelude::*;
use ronbun::models::NewUser;
use ronbun::schema::users as users_schema;
use ronbun::utils::establish_connection;

fn main() {
    let connection = establish_connection();

    let new_user = vec![
        NewUser {
            user_name: String::from("new_user_1"),
            user_email: String::from("new_user_1@sample.com"),
        },
        NewUser {
            user_name: String::from("new_user_2"),
            user_email: String::from("new_user_2@sample.com"),
        },
    ];

    diesel::insert_into(users_schema::dsl::users)
        .values(new_user)
        .execute(&connection)
        .expect("Error saving new user");
}
