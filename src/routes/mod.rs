use axum::{
    routing::{get, patch, post}, Extension, Router
};


mod hello;
use hello::hello_world;
mod query_params;
mod mirror_body_json;
mod mirror_body_string;
mod path_variable;
mod user_agent;
mod middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use query_params::query_params;
use path_variable::path_variable;
use user_agent::user_agent;
use middleware_message::middleware_message;

#[derive(Clone, Debug)]
pub struct ShareData {
    pub message: String,
    
}

pub fn create_routes() ->Router{
    let share_data  = ShareData {
        message: "Hello from Axum".to_owned()
    };
    // It is retrun a Router in axum 0.7
    Router::new().route("/", get(hello_world))
    .route("/mirror_body_json", get(mirror_body_json))
    .route("/mirror_body_string", post(mirror_body_string))
    .route("/query_params", get(query_params))
    .route("/path_variable/:id", post(path_variable))
    .route("/user_agent", get(user_agent))
    .route("/middleware_message", get(middleware_message))
    .layer(Extension(share_data))
    .layer(tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init())
}
