from __future__ import annotations

from dataclasses import dataclass
import os
import shutil
import subprocess


@dataclass(frozen=True)
class BridgeConfig:
    reservoir_bin: str = "reservoir"
    partition: str | None = None
    instance: str | None = None


class ReservoirBridge:
    def __init__(self, config: BridgeConfig) -> None:
        self._config = config
        self._validate_binary(config.reservoir_bin)

    @staticmethod
    def _validate_binary(reservoir_bin: str) -> None:
        if os.path.sep in reservoir_bin:
            if os.path.exists(reservoir_bin):
                return
            raise FileNotFoundError(f"reservoir binary not found: {reservoir_bin}")
        if shutil.which(reservoir_bin) is None:
            raise FileNotFoundError(f"reservoir binary not found in PATH: {reservoir_bin}")

    # Subcommands that accept --partition / --instance flags.
    _PARTITIONED_COMMANDS = {"search", "view", "thread", "ingest"}

    def run(self, subcommand: str, *args: str, stdin: str | None = None) -> str:
        cmd: list[str] = [self._config.reservoir_bin, subcommand]
        # Inject default partition/instance for commands that support them,
        # unless the caller already passed explicit values.
        if subcommand in self._PARTITIONED_COMMANDS:
            if self._config.partition and "--partition" not in args and "-p" not in args:
                cmd.extend(["--partition", self._config.partition])
            if self._config.instance and "--instance" not in args and "-i" not in args:
                cmd.extend(["--instance", self._config.instance])
        cmd.extend(args)

        completed = subprocess.run(
            cmd,
            check=False,
            capture_output=True,
            text=True,
            input=stdin,
        )
        if completed.returncode != 0:
            stderr = completed.stderr.strip()
            stdout = completed.stdout.strip()
            details = stderr or stdout or "unknown error"
            raise RuntimeError(
                f"reservoir command failed ({completed.returncode}): {' '.join(cmd)}\n{details}"
            )
        return completed.stdout
