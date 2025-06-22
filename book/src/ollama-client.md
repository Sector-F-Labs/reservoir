# Ollama Client Integration

You can use reservoir as a memory system for the [Ollama](https://ollama.com/) command line client by integrating it with a simple bash script.

You can place the following function in your `~/.bashrc` or `~/.zshrc` file and it will use reservoir to
 - Fetch context from the model
 - Prepend the context to your query
 - Send the request to the model
 - Save the output

```sh
function contextual_ollama_with_ingest() {
    local user_query="$1"

    # Validate input
    if [ -z "$user_query" ]; then
        echo "Usage: contextual_ollama_with_ingest 'Your question goes here'" >&2
        return 1
    fi

    # Ingest the user's query into Reservoir
    echo "$user_query" | reservoir ingest

    # Generate dynamic system prompt with context
    local system_prompt_content=$(
        echo "the following is info from semantic search based on your query:"
        reservoir search "$user_query" --semantic --link
        echo "the following is recent history:"
        reservoir view 10
    )

    local full_prompt_content=$(
        echo "You are a helpful assistant. Use the following context to answer the user's question."
        echo "$system_prompt_content"
        echo "User's question: ${user_query}"
    )

    # Call cgip with enriched context
    local assistant_response=$(ollama run gemma3 "$full_prompt_content")
    
    # Store the assistant's response
    echo "$assistant_response" | reservoir ingest --role assistant

    # Display the response
    echo "$assistant_response"
}

# Create a convenient alias
alias olm='contextual_ollama_with_ingest'
```


By adhering to POSIX standards, reservoir become the semantic memory for any shell interaction with a language model.
