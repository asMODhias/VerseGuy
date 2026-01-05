#!/usr/bin/env bash
set -euo pipefail

echo "This will remove all stopped containers, images, volumes and prune builder cache."
read -p "Proceed? (y/N) " ans
if [[ "$ans" != "y" && "$ans" != "Y" ]]; then
  echo "Aborted."; exit 1
fi

docker ps -aq | xargs -r docker rm -f
docker images -aq | xargs -r docker rmi -f
docker volume ls -q | xargs -r docker volume rm
# aggressive pruning
docker builder prune -af
docker system prune -a --volumes -f
docker network prune -f

echo "Cleanup complete."