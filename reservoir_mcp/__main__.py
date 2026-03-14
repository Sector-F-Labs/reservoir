from __future__ import annotations

import argparse
from typing import Sequence

from .bridge import BridgeConfig, ReservoirBridge


def _parse_args(argv: Sequence[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        prog="reservoir-mcp",
        description="MCP server that proxies tool calls to the Rust reservoir CLI.",
    )
    parser.add_argument(
        "--transport",
        choices=["stdio", "streamable-http"],
        default="stdio",
        help="MCP transport mode.",
    )
    parser.add_argument(
        "--reservoir-bin",
        default="reservoir",
        help="Path or command name for the Rust reservoir CLI binary.",
    )
    parser.add_argument(
        "--host",
        default="127.0.0.1",
        help="Bind host for HTTP-based transports.",
    )
    parser.add_argument(
        "--port",
        type=int,
        default=8000,
        help="Bind port for HTTP-based transports.",
    )
    parser.add_argument(
        "--streamable-http-path",
        default="/mcp",
        help="HTTP endpoint path for streamable HTTP transport.",
    )
    parser.add_argument(
        "--partition",
        default=None,
        help="Default partition for all commands (avoids mixing with 'default').",
    )
    parser.add_argument(
        "--instance",
        default=None,
        help="Default instance for all commands (defaults to partition if not set).",
    )
    return parser.parse_args(argv)


def main(argv: Sequence[str] | None = None) -> None:
    args = _parse_args(argv)
    try:
        from .server import build_server
    except ModuleNotFoundError as exc:
        if exc.name == "mcp":
            raise SystemExit(
                "Missing dependency 'mcp'. Install reservoir-mcp with pipx/uv to pull dependencies."
            ) from exc
        raise

    bridge = ReservoirBridge(BridgeConfig(
        reservoir_bin=args.reservoir_bin,
        partition=args.partition,
        instance=args.instance,
    ))
    mcp = build_server(
        bridge,
        host=args.host,
        port=args.port,
        streamable_http_path=args.streamable_http_path,
    )
    mcp.run(transport=args.transport)


if __name__ == "__main__":
    main()
