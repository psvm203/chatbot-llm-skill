mod handlers;
mod router;

use worker::*;

#[event(fetch)]
async fn fetch(request: Request, env: Env, _ctx: Context) -> Result<Response> {
    router::dispatch(request, env).await
}
