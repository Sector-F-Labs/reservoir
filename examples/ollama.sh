
# Add this to your ~/.bashrc or ~/.zshrc file
# (then 'source' it or open a new terminal for it to take effect)

function contextual_ollama_with_ingest() {
    local user_query="$1" # The query the user provides

    # --- Step 1: Validate input ---
    if [ -z "$user_query" ]; then
        echo "Usage: contextual_cgip_with_ingest 'Your question goes here'" >&2
        return 1
    fi

    # --- Step 2: Ingest the user's query into Reservoir ---
    echo "$user_query" | reservoir ingest

    # --- Step 3: Generate the dynamic system prompt directly ---
    # We combine the literal text and command outputs all within one command substitution!
    local system_prompt_content=$(
        echo "> You are a fuzzy llama named ollama, the following is info from semantic search based on your query:"
        reservoir search "$user_query" --semantic --link
        echo "> the following is recent history:"
        reservoir view 10
		echo "> the following is the users query: $user_query"
    )

    # --- Step 4: Call ollama with the generated prompt and capture its output ---
    local assistant_response=$(ollama run gemma3 "${system_prompt_content}")
    
    # --- Step 5: Ingest the assistant's response into Reservoir ---
    echo "$assistant_response" | reservoir ingest --role assistant

    # --- Step 6: Print the assistant's response to the user ---
    echo "$assistant_response"
}

alias olma='contextual_ollama_with_ingest'
