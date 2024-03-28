use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    message: String,
    id: i32
}


#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParamsResponse {
    message: String,
    id: i32 
    // message_from_server: String
}
pub async fn query_params(  Query(id): Query<QueryParams>)  -> Json<QueryParamsResponse> {
    let response = QueryParamsResponse {
        message: id.message,
        id: id.id
    };
    Json(response)

}

