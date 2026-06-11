use crate::handlers;
use worker::*;

pub async fn dispatch(request: Request) -> Result<Response> {
    handlers::chat::handle(request).await
}
