use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[macro_use]
extern crate serde_derive;

use diesel::prelude::*;
use ronbun::models::{NewUser, User};
use ronbun::schema::users as users_schema;
use ronbun::schema::papers as papers_schema;
use ronbun::utils::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        || App::new()
            .route("/", web::get().to(read_user))
            .route("/search", web::get().to(search))
    )
    .bind("0.0.0.0:8989")?
    .run()
    .await
}

#[derive(Deserialize)]
struct Info {
    id: usize,
}

async fn read_user(id_num: web::Query<Info>) -> impl Responder {
    let connection = establish_connection();

    let user = users_schema::dsl::users
        .filter(users_schema::user_id.eq(id_num.id as i32))
        .first::<User>(&connection);

    // let user = &users[id_num.id];
    match user {
        Ok(u) => HttpResponse::Ok().content_type("text/html").body(format!(
            "<h1>aaaa</h1>\n<p>Name: {}</p>\n<p>Emainl: {}</p>\n<p>Latest update: {}</p>",
            u.user_name, u.user_email, u.updated_at
        )),
        Err(_) => HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("<h1>Not found id = {}</h1>", id_num.id)),
    }
}


#[derive(Deserialize)]
struct Word {
    search: String,
}

async fn search(pattern: web::Query<Word>) -> impl Responder {
    let connection = establish_connection();

    let pattern = format!("%{}%", pattern.search);
    let users = users_schema::dsl::users
        .filter(users_schema::user_name.like(&pattern))
        .load::<User>(&connection);
    
    match users {
        Ok(u) => format!("{:#?}", u),
        Err(e) => format!("Not found.\nError: {:?}", e),
    }
}

