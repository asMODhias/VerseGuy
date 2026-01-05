param([switch]$Force)

if (-not $Force) {
    $r = Read-Host "This will remove all containers, images, volumes and prune caches. Proceed? (Y/N)"
    if ($r -notin @('Y','y')) { Write-Host "Aborted."; exit 1 }
}

docker ps -aq | ForEach-Object { docker rm -f $_ } 2>$null
docker images -aq | ForEach-Object { docker rmi -f $_ } 2>$null
docker volume ls -q | ForEach-Object { docker volume rm $_ } 2>$null
docker builder prune -af
docker system prune -a --volumes -f
docker network prune -f
Write-Host "Cleanup complete."