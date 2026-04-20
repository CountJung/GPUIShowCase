param(
    [string]$BindHost = "127.0.0.1",
    [int]$Port = 4173,
    [string]$TargetDir = "target-agent"
)

$ErrorActionPreference = "Stop"
$env:RUST_BACKTRACE = "1"

& cargo run --bin web_mode --target-dir $TargetDir -- --host $BindHost --port $Port