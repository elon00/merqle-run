#!/usr/bin/env python3
"""Run real local Rust tests only. Never assert RPC/PQC/deployment readiness."""
from pathlib import Path
import shutil
import subprocess
import sys


def main() -> int:
    cargo = shutil.which("cargo")
    if cargo is None:
        print("BLOCKED: cargo executable is unavailable", file=sys.stderr)
        return 2
    root = Path(__file__).resolve().parents[1]
    result = subprocess.run(
        [cargo, "test", "--offline", "--all-targets"], cwd=root, check=False
    )
    if result.returncode:
        return result.returncode
    print("PASS: local Rust unit tests only; security and production gates remain unverified")
    return 0


if __name__ == "__main__":
    sys.exit(main())
