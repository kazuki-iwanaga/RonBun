use diesel::prelude::*;
use ronbun::models::User;
use ronbun::schema::users as users_schema;
use ronbun::utils::establish_connection;

fn main() {
    let connection = establish_connection();

    let users = users_schema::dsl::users
        .load::<User>(&connection)
        .expect("Error loading users");

    for user in users {
        println!("{:?}", user);
    }
}
