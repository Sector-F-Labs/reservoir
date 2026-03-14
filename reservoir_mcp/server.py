from __future__ import annotations

from typing import Optional

from mcp.server.fastmcp import FastMCP

from .bridge import ReservoirBridge


def build_server(
    bridge: ReservoirBridge,
    host: str = "127.0.0.1",
    port: int = 8000,
    streamable_http_path: str = "/mcp",
) -> FastMCP:
    mcp = FastMCP(
        "reservoir-mcp",
        instructions=(
            "Chat history search and storage. Use `search` to find past messages "
            "by keyword or meaning, `view` to see recent messages, and `thread` "
            "to follow reply chains. Messages come from chat archives such as "
            "Telegram conversations."
        ),
        host=host,
        port=port,
        streamable_http_path=streamable_http_path,
    )

    @mcp.tool(
        description=(
            "Search chat history by keyword, or use semantic mode "
            "for meaning-based similarity search."
        )
    )
    def search(
        term: str,
        semantic: bool = False,
        link: bool = False,
        deduplicate: bool = False,
    ) -> str:
        args = [term]
        if semantic:
            args.append("--semantic")
        if link:
            args.append("--link")
        if deduplicate:
            args.append("--deduplicate")
        return bridge.run("search", *args)

    @mcp.tool(
        description="View the most recent chat messages."
    )
    def view(count: int = 20) -> str:
        return bridge.run("view", str(count))

    @mcp.tool(
        description="Follow the conversation thread — shows messages connected by reply chains."
    )
    def thread(count: int = 50) -> str:
        args: list[str] = []
        if count != 50:
            args.extend(["--count", str(count)])
        return bridge.run("thread", *args)

    @mcp.tool(
        description="Store a new message in the chat history."
    )
    def ingest(
        content: str,
        role: str = "user",
        trace_id: Optional[str] = None,
    ) -> str:
        args: list[str] = []
        if role != "user":
            args.extend(["--role", role])
        if trace_id:
            args.extend(["--trace-id", trace_id])
        return bridge.run("ingest", *args, stdin=content)

    @mcp.tool(description="Export the full chat history as JSON.")
    def export() -> str:
        return bridge.run("export")

    @mcp.tool(description="Get or set reservoir configuration values (e.g. embedding model, database path).")
    def config(
        get: Optional[str] = None,
        set_value: Optional[str] = None,
    ) -> str:
        args: list[str] = []
        if get:
            args.extend(["--get", get])
        if set_value:
            args.extend(["--set", set_value])
        return bridge.run("config", *args)

    return mcp
