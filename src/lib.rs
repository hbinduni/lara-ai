use worker::*;

mod handlers;
use handlers::{handle_cors_preflight, handle_groq, handle_info};

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let router = Router::new();

    router
        .post_async(
            "/groq",
            |req, ctx| async move { handle_groq(req, ctx).await },
        )
        .options("/groq", |_req, _ctx| handle_cors_preflight())
        .get("/", handle_info)
        .run(req, env)
        .await
}
