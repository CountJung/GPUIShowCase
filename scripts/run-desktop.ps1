param(
    [string]$Route = "",
    [string]$Category = "",
    [string]$Component = "",
    [string]$PlaygroundScript = "",
    [string]$TargetDir = "target-agent"
)

$ErrorActionPreference = "Stop"
$env:RUST_BACKTRACE = "1"

$cargoArgs = @("run", "--bin", "gpuishowcase", "--target-dir", $TargetDir)
$appArgs = @()

if ($Route) { $appArgs += @("--route", $Route) }
if ($Category) { $appArgs += @("--category", $Category) }
if ($Component) { $appArgs += @("--component", $Component) }
if ($PlaygroundScript) { $appArgs += @("--playground-script", $PlaygroundScript) }

if ($appArgs.Count -gt 0) {
    $cargoArgs += "--"
    $cargoArgs += $appArgs
}

& cargo @cargoArgs