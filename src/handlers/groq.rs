use serde_json::{json, Value};
use worker::*;

// Constants to avoid repeated allocations
const DEFAULT_SYSTEM_MSG: &str = "You are a helpful, knowledgeable, and professional AI assistant. Provide accurate, clear, and concise responses to assist users with their questions and tasks.";
const DEFAULT_MODEL: &str = "qwen-qwq-32b";
const GROQ_API_URL: &str = "https://api.groq.com/openai/v1/chat/completions";

// CORS and SSE header constants
const CONTENT_TYPE_SSE: &str = "text/event-stream";
const CACHE_CONTROL: &str = "no-cache";
const CONNECTION: &str = "keep-alive";
const CORS_ORIGIN: &str = "*";
const CORS_METHODS: &str = "GET, POST, OPTIONS";
const CORS_HEADERS: &str = "Content-Type, Authorization, X-API-Key";

pub async fn handle_groq(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Early authentication check - fail fast before parsing body
    let provided_api_key = req.headers().get("X-API-Key")?;

    let expected_api_key = match ctx.env.secret("X_API_KEY") {
        Ok(secret) => secret.to_string(),
        Err(_) => return Response::error("X_API_KEY environment variable not configured", 500),
    };

    let provided_key = match provided_api_key {
        Some(key) => key,
        None => return Response::error("X-API-Key header required", 401),
    };

    if provided_key != expected_api_key {
        return Response::error("Invalid X-API-Key", 401);
    }

    // Parse JSON payload
    let body = req.text().await?;
    let payload: Value =
        serde_json::from_str(&body).map_err(|_| worker::Error::from("Invalid JSON payload"))?;

    // Extract and validate prompt
    let prompt = payload
        .get("prompt")
        .and_then(|v| v.as_str())
        .ok_or_else(|| worker::Error::from("Missing or invalid 'prompt' field"))?;

    if prompt.trim().is_empty() {
        return Response::error("Prompt cannot be empty", 400);
    }

    // Get configuration values with efficient fallbacks
    let env_system_msg = ctx
        .env
        .var("DEFAULT_SYSTEM_MSG")
        .ok()
        .map(|v| v.to_string());
    let system_msg = payload
        .get("systemMsg")
        .and_then(|v| v.as_str())
        .or_else(|| env_system_msg.as_deref())
        .unwrap_or(DEFAULT_SYSTEM_MSG);

    let env_model = ctx.env.var("DEFAULT_MODEL").ok().map(|v| v.to_string());
    let model = payload
        .get("model")
        .and_then(|v| v.as_str())
        .or_else(|| env_model.as_deref())
        .unwrap_or(DEFAULT_MODEL);

    // Get Groq API key
    let groq_api_key = ctx
        .env
        .secret("GROQ_API_KEY")
        .map_err(|_| worker::Error::from("GROQ_API_KEY environment variable not found"))?
        .to_string();

    // Build Groq API request payload
    let groq_request = json!({
        "messages": [
            {"role": "system", "content": system_msg},
            {"role": "user", "content": prompt}
        ],
        "model": model,
        "temperature": 0.6,
        "max_completion_tokens": 4096,
        "top_p": 0.95,
        "stream": true,
        "stop": null
    });

    // Create headers efficiently
    let headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    headers.set("Authorization", &format!("Bearer {}", groq_api_key))?;
    headers.set(
        "User-Agent",
        &format!("lara-ai/{}", env!("CARGO_PKG_VERSION")),
    )?;

    // Create and send request
    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(serde_json::to_string(&groq_request)?.into()));

    let mut api_response = Fetch::Request(Request::new_with_init(GROQ_API_URL, &init)?)
        .send()
        .await?;

    // Check response status
    let status = api_response.status_code();
    if !(200..300).contains(&status) {
        let error_text = api_response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Response::error(&format!("Groq API error: {}", error_text), status);
    }

    // Stream response with optimized headers
    let mut response = Response::from_body(api_response.body().clone())?;
    set_streaming_headers(response.headers_mut())?;
    Ok(response)
}

fn set_streaming_headers(headers: &Headers) -> Result<()> {
    headers.set("Content-Type", CONTENT_TYPE_SSE)?;
    headers.set("Cache-Control", CACHE_CONTROL)?;
    headers.set("Connection", CONNECTION)?;
    headers.set("Access-Control-Allow-Origin", CORS_ORIGIN)?;
    headers.set("Access-Control-Allow-Methods", CORS_METHODS)?;
    headers.set("Access-Control-Allow-Headers", CORS_HEADERS)?;
    Ok(())
}
