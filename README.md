# Lara AI

A high-performance Cloudflare Worker that provides a clean API interface to Groq AI's large language models. Built with Rust for optimal performance and minimal cold start times.

## 🚀 Features

- **Groq AI Integration**: Direct access to Groq's fast LLM inference
- **Configurable Defaults**: Customizable system messages and models via environment variables
- **CORS Support**: Ready for web applications with proper CORS headers
- **Input Validation**: Robust request validation and error handling
- **Performance Optimized**: Uses native Cloudflare Worker APIs for minimal overhead

## 📡 API Endpoints

### POST `/groq`

Send chat completion requests to Groq AI models.

**Request Body:**

```json
{
  "prompt": "Your question here", // Required
  "systemMsg": "Custom system message", // Optional
  "model": "llama-3.1-8b-instant" // Optional
}
```

**Example:**

```bash
curl -X POST https://your-worker.workers.dev/groq \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "Explain quantum computing in simple terms",
    "systemMsg": "You are a helpful physics teacher",
    "model": "llama-3.1-8b-instant"
  }'
```

### GET `/`

Health check endpoint that returns "Hello World!".

### OPTIONS `/groq`

CORS preflight handler for cross-origin requests.

## 🛠️ Setup

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- Groq API key ([Get one here](https://console.groq.com/keys))

### Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/hbinduni/lara-ai.git
   cd lara-ai
   ```

2. **Install dependencies:**

   ```bash
   cargo build
   ```

3. **Set up environment variables:**

   For production:

   ```bash
   # Set your Groq API key
   wrangler secret put GROQ_API_KEY
   # Enter your Groq API key when prompted

   # Set your API authentication key
   wrangler secret put X_API_KEY
   # Enter a secure API key for client authentication
   ```

   For local development, create `.dev.vars`:

   ```bash
   cat > .dev.vars << EOF
   GROQ_API_KEY=your_actual_groq_api_key_here
   X_API_KEY=your-secret-api-key-12345
   EOF
   ```

4. **Configure defaults (optional):**

   ```bash
   # Set custom default system message
   wrangler secret put DEFAULT_SYSTEM_MSG

   # Set custom default model
   wrangler secret put DEFAULT_MODEL
   ```

## 🔐 Authentication

All requests to the `/groq` endpoint require authentication via the `X-API-Key` header. This protects your worker from unauthorized access.

**Important:** Use a strong, unique API key and keep it secure. Consider using different keys for different environments or applications.

### Local Development

```bash
# Start local development server
wrangler dev

# Test the endpoint
curl -X POST http://localhost:8787/groq \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-secret-api-key-12345" \
  -d '{"prompt": "Hello, how are you?"}'
```

### Deployment

```bash
# Deploy to Cloudflare Workers
wrangler deploy
```

## ⚙️ Configuration

### Environment Variables

| Variable             | Description            | Default                                                              |
| -------------------- | ---------------------- | -------------------------------------------------------------------- |
| `GROQ_API_KEY`       | Your Groq API key      | _Required_                                                           |
| `DEFAULT_SYSTEM_MSG` | Default system message | "You are a helpful, knowledgeable, and professional AI assistant..." |
| `DEFAULT_MODEL`      | Default Groq model     | `qwen-qwq-32b`                                                       |

### Supported Models

- `qwen-qwq-32b` (default)
- `llama-3.1-8b-instant`
- `llama-3.1-70b-instant`
- `mixtral-8x7b-32768`
- And more... (check [Groq's documentation](https://console.groq.com/docs/models))

## 📝 API Response Format

The API streams Server-Sent Events (SSE) directly from Groq AI. Each chunk is a JSON object in the format:

```
data: {"id":"chatcmpl-xxx","object":"chat.completion.chunk","created":1234567890,"model":"qwen-qwq-32b","choices":[{"index":0,"delta":{"content":"Hello"},"finish_reason":null}]}

data: {"id":"chatcmpl-xxx","object":"chat.completion.chunk","created":1234567890,"model":"qwen-qwq-32b","choices":[{"index":0,"delta":{"content":" there!"},"finish_reason":null}]}

data: {"id":"chatcmpl-xxx","object":"chat.completion.chunk","created":1234567890,"model":"qwen-qwq-32b","choices":[{"index":0,"delta":{},"finish_reason":"stop"}]}

data: [DONE]
```

### Response Headers

```
Content-Type: text/event-stream
Cache-Control: no-cache
Connection: keep-alive
Access-Control-Allow-Origin: *
```

### Consuming the Stream

**JavaScript (Fetch API):**

```javascript
const response = await fetch(
  "https://your-worker.your-subdomain.workers.dev/groq",
  {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "X-API-Key": "your-secret-api-key-12345",
    },
    body: JSON.stringify({ prompt: "Hello!" }),
  }
);

const reader = response.body.getReader();
const decoder = new TextDecoder();
let content = "";

while (true) {
  const { done, value } = await reader.read();
  if (done) break;

  const chunk = decoder.decode(value);
  const lines = chunk.split("\n");

  for (const line of lines) {
    if (line.startsWith("data: ")) {
      const data = line.slice(6);
      if (data === "[DONE]") return content;

      try {
        const parsed = JSON.parse(data);
        const delta = parsed.choices?.[0]?.delta?.content;
        if (delta) {
          content += delta;
          console.log(delta); // Stream to UI
        }
      } catch (e) {
        // Skip invalid JSON
      }
    }
  }
}
```

## 🚀 Usage Examples

### Basic Request

```bash
curl -X POST https://your-worker.your-subdomain.workers.dev/groq \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-secret-api-key-12345" \
  -d '{"prompt": "Explain quantum computing in simple terms"}'
```

### Custom System Message

```bash
curl -X POST https://your-worker.your-subdomain.workers.dev/groq \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-secret-api-key-12345" \
  -d '{
    "prompt": "Write a haiku about coding",
    "systemMsg": "You are a poet who specializes in technical subjects"
  }'
```

### Different Model

```bash
curl -X POST https://your-worker.your-subdomain.workers.dev/groq \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-secret-api-key-12345" \
  -d '{
    "prompt": "What is the meaning of life?",
    "model": "llama-3.1-70b-instant"
  }'
```

## 🚨 Error Handling

| Status Code | Description                                        |
| ----------- | -------------------------------------------------- |
| `400`       | Bad Request (empty prompt)                         |
| Status Code | Description                                        |
| ----------- | -------------------------------------------------- |
| `400`       | Bad Request (empty prompt)                         |
| `401`       | Unauthorized (missing or invalid X-API-Key)        |
| `500`       | Server Error (missing API key, invalid JSON, etc.) |

Example error responses:

```json
{
  "error": "Prompt cannot be empty"
}
```

```json
{
  "error": "X-API-Key header required"
}
```

```json
{
  "error": "Invalid X-API-Key"
}
```

## 🏗️ Architecture

- **Runtime**: Cloudflare Workers (V8 isolates)
- **Language**: Rust (compiled to WebAssembly)
- **HTTP Client**: Native Cloudflare Worker Fetch API
- **Performance**: Optimized for minimal cold start times

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Groq](https://groq.com/) for providing fast LLM inference
- [Cloudflare Workers](https://workers.cloudflare.com/) for the serverless platform
- [worker-rs](https://github.com/cloudflare/workers-rs) for the Rust Workers SDK

## 📞 Support

If you have questions or need help:

- Open an [issue](https://github.com/hbinduni/lara-ai/issues)
- Check the [Cloudflare Workers documentation](https://developers.cloudflare.com/workers/)
- Review [Groq's API documentation](https://console.groq.com/docs)
