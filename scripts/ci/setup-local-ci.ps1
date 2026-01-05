Write-Host "Setting up local CI: pulling/building required images"

# 1) Pull act base runner image
Write-Host "-> Pulling nektos/act-environments-ubuntu:18.04"
docker pull nektos/act-environments-ubuntu:18.04 | Write-Host

# 2) Build local act-runner with Node 18
Write-Host "-> Building verseguy/act-runner:node18 from scripts/dockerfiles/act-runner-node18.Dockerfile"
docker build -t verseguy/act-runner:node18 -f scripts/dockerfiles/act-runner-node18.Dockerfile . | Write-Host

Write-Host "Note: Building 'containers/p2p' can be very large. To build, run: .\scripts\ci\rebuild-images.ps1"
Write-Host "Setup completed. Use .\scripts\ci\run-act-job.ps1 to run CI jobs locally (it uses verseguy/act-runner:node18)."