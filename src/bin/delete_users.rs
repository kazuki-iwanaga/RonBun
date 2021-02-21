use diesel::prelude::*;
use ronbun::schema::users as users_schema;
use ronbun::utils::establish_connection;

fn main() {
    let connection = establish_connection();
    diesel::delete(
        users_schema::dsl::users.filter(users_schema::user_name.like("%user%"))
    )
        .execute(&connection)
        .expect("Error updating users");
}
