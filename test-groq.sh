#!/bin/bash
BASE_URL="http://localhost:8787"

curl -X POST $BASE_URL/groq \
  -H "Content-Type: application/json" \
  -H "x-api-key: your-secret-api-key-12345" \
  -d '{
    "prompt": "What is the capital of France?"
  }'

# curl -X POST $BASE_URL/groq \
#   -H "Content-Type: application/json" \
#   -d '{
#     "prompt": "Explain quantum computing",
#     "systemMsg": "You are an expert physics professor",
#     "model": "llama-3.1-8b-instant"
#   }'