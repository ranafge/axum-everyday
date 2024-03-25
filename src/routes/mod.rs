use axum::{
    routing::{get, patch, post}, Router
};


mod hello;
use hello::hello_world;

mod mirror_body_json;
use mirror_body_json::mirror_body_json;
pub fn create_routes() ->Router{
    // It is retrun a Router in axum 0.7
    Router::new().route("/", get(hello_world))
    .route("/mirror_body_json", patch(mirror_body_json))
}
