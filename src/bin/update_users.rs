use diesel::prelude::*;
use ronbun::schema::users as users_schema;
use ronbun::utils::establish_connection;

fn main() {
    let connection = establish_connection();
    diesel::update(users_schema::dsl::users.filter(users_schema::user_name.eq("new_user_1")))
        .set(users_schema::user_name.eq("updated_user"))
        .execute(&connection)
        .expect("Error updating users");
}
