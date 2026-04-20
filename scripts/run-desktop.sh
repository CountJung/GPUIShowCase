#!/usr/bin/env bash
set -euo pipefail

route=""
category=""
component=""
playground_script=""
target_dir="target-agent"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --route)
      route="$2"
      shift 2
      ;;
    --category)
      category="$2"
      shift 2
      ;;
    --component)
      component="$2"
      shift 2
      ;;
    --playground-script)
      playground_script="$2"
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

cargo_args=(run --bin gpuishowcase --target-dir "$target_dir")
app_args=()

if [[ -n "$route" ]]; then app_args+=(--route "$route"); fi
if [[ -n "$category" ]]; then app_args+=(--category "$category"); fi
if [[ -n "$component" ]]; then app_args+=(--component "$component"); fi
if [[ -n "$playground_script" ]]; then app_args+=(--playground-script "$playground_script"); fi

if [[ ${#app_args[@]} -gt 0 ]]; then
  cargo_args+=(--)
  cargo_args+=("${app_args[@]}")
fi

"${cargo_args[@]}"