use crate::handlers;
use worker::*;

pub async fn dispatch(request: Request, env: Env) -> Result<Response> {
    handlers::chat::handle(request, env).await
}
