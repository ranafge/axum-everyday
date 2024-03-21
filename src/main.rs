use axum::{
    routing::get, Router
    
};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
   let app = Router::new().route("/", get(hello_world));
   let tcp_listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
   axum::serve(tcp_listener, app.into_make_service()).await.unwrap()

}



async fn hello_world() -> String {
    println!("Hello world!");
    format!("{}", "Hello world! from hello world handler!") 
}