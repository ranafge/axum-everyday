use axum::Extension;

use super::ShareData;


pub async fn hello_world(Extension(share_data): Extension<ShareData>) -> String {
    println!("Hello world!");
    format!("{}", "Hello world! from hello world handler!") ;
    share_data.message
}