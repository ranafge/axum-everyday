
use tokio::net::TcpListener;
use axum::{ routing::get, Router};

mod routes;

use routes::create_routes;

pub async fn run() {
    let app = create_routes();
    let tcp_listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening from port : 8080");
    axum::serve(tcp_listener, app.into_make_service()).await.unwrap();
 
}