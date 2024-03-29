use axum::Extension;

use super::ShareData;

pub async fn middleware_message(Extension(share_data): Extension<ShareData>) -> String {
    share_data.message
}