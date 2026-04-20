#!/usr/bin/env bash
set -euo pipefail

host="127.0.0.1"
port="4173"
target_dir="target-agent"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --host)
      host="$2"
      shift 2
      ;;
    --port)
      port="$2"
      shift 2
      ;;
    --target-dir)
      target_dir="$2"
      shift 2
      ;;
    *)
      shift
      ;;
  esac
done

export RUST_BACKTRACE=1

cargo run --bin web_mode --target-dir "$target_dir" -- --host "$host" --port "$port"