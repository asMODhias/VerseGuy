param([string[]]$Args)

# Run build-rust job locally
$cmd = "act -j build-rust -W .github/workflows/ci.yml -P ubuntu-latest=verseguy/act-runner:node18 --action-offline-mode" + ($Args -join ' ')
Write-Host "Running: $cmd"
Invoke-Expression $cmd
