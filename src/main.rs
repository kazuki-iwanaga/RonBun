// https://github.com/seanmonstar/warp

use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    println!("Server is listening on http://localhost:8989/hello/RonBun");
    
    warp::serve(hello)
        .run(([0, 0, 0, 0], 8989))
        .await;
}
