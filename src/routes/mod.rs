use std::any;

use axum::{
    http::Method, routing::{get, patch, post}, Extension, Router
};
use axum::http::header;

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
use tower_http::cors::{ Any, CorsLayer};
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
    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST])
    .allow_headers([header::CONTENT_TYPE]);
    // It is retrun a Router in axum 0.7
    Router::new().route("/", get(hello_world))
    .route("/mirror_body_json", get(mirror_body_json))
    .route("/mirror_body_string", post(mirror_body_string))
    .route("/query_params", get(query_params))
    .route("/path_variable/:id", post(path_variable))
    .route("/user_agent", get(user_agent))
    .route("/middleware_message", get(middleware_message))
    .layer(Extension(share_data))
    .layer(cors)
    .layer(tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init())
}
