
use axum::Json;
use serde::Serialize;
use serde::Deserialize;


#[derive(Debug,Serialize, Deserialize)]

pub struct MirrorJson {
    message: String
}

#[derive(Debug, Serialize,Deserialize)]
pub struct MirrorJsonResponse{
    message: String,
    message_from_server: String
}
pub async fn mirror_body_json (Json(body): Json<MirrorJson>) -> Json<MirrorJsonResponse>{
    Json(MirrorJsonResponse {
        message: body.message,
        message_from_server: "Hello from Axum".to_owned()
    })
    
    
}