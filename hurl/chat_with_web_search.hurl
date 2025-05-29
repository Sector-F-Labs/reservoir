POST http://127.0.0.1:3017/partition/{{USER}}/instance/reservoir/v1/chat/completions
Content-Type: application/json

{
  "model": "gemma3",
  "messages": [
    {
      "role": "user",
      "content": "What are the latest developments in AI?"
    }
  ],
  "web_search_options": {
    "enabled": true,
    "max_results": 5
  }
}

HTTP/1.1 200