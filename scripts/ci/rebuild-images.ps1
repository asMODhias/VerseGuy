Write-Host "Building verseguy/act-runner:node18"
docker build -t verseguy/act-runner:node18 -f scripts/dockerfiles/act-runner-node18.Dockerfile . | Write-Host

Write-Host "About to build verseguy/p2p:local (this may be large). Proceeding..."
docker build -t verseguy/p2p:local -f containers/p2p/Dockerfile . | Write-Host
