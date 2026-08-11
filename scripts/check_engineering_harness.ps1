$ErrorActionPreference = "Stop"
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path

python (Join-Path $PSScriptRoot "validate_engineering_artifacts.py") --root $repoRoot
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
python (Join-Path $PSScriptRoot "generate_harness_dashboard.py") --root $repoRoot
exit $LASTEXITCODE
