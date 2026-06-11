mod handlers;
mod models;
mod router;

use worker::*;

#[event(fetch)]
async fn fetch(request: Request, _env: Env, _ctx: Context) -> Result<Response> {
    router::dispatch(request).await
}
