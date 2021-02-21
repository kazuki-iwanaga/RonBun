use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[get("/{name}")]
async fn hello(web::Path(name): web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(hello))
        .bind("0.0.0.0:8989")?
        .run()
        .await
}
