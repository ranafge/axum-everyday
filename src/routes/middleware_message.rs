use axum::Extension;

use super::ShareData;
use tracing::{info, debug};
pub async fn middleware_message(Extension(share_data): Extension<ShareData>) -> String {
    info!("You are in middleware message handler");
    debug!("share_data: {:?}", share_data.message);
    share_data.message
}