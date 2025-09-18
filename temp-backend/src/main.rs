use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String,
}

#[tokio::main]
async fn main() {
    // GET / -> JSON response
    let hello = warp::path::end()
        .and(warp::get())
        .and_then(hello_handler);

    let routes = hello
        .with(warp::cors().allow_any_origin());

    println!("Server starting on http://localhost:3000");
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}

async fn hello_handler() -> Result<impl warp::Reply, Infallible> {
    let response = HelloResponse {
        message: "hello world".to_string(),
    };
    
    Ok(warp::reply::json(&response))
}
