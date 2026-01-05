#!/usr/bin/env bash
set -euo pipefail

# Wrapper around `docker build` with pre-build size/duration estimation and
# live progress + time estimates.
# Usage: ./scripts/docker-build-wrapper.sh [docker build args...]

if [ "$#" -eq 0 ]; then
  echo "Usage: $0 [docker build args...]"
  exit 1
fi

BW_MB=${DOCKER_BUILD_EST_BW_MB:-10}
BUILD_SEC_PER_MB=${DOCKER_BUILD_SEC_PER_MB:-0.5}

# Find Dockerfile path from args (-f)
DOCKERFILE="Dockerfile"
ARGS=("$@")
for ((i=0;i<${#ARGS[@]};i++)); do
  if [[ "${ARGS[i]}" == "-f" ]]; then
    DOCKERFILE="${ARGS[i+1]}"
  fi
done

# Parse FROM lines
mapfile -t FROMS < <(grep -E '^FROM ' "$DOCKERFILE" | awk '{print $2}' | uniq)
# Parse COPY --from lines to detect final copied paths
mapfile -t COPY_FROM < <(grep -E 'COPY\s+--from=' "$DOCKERFILE" | awk '{print $3}' || true)

# Get image sizes for bases (bytes)
total_base_size=0
for img in "${FROMS[@]}"; do
  if docker image inspect --format '{{.Size}}' "$img" >/dev/null 2>&1; then
    s=$(docker image inspect --format '{{.Size}}' "$img")
  else
    echo "Image $img not present locally; pulling for size estimation..." >&2
    docker pull "$img" || true
    s=$(docker image inspect --format '{{.Size}}' "$img" 2>/dev/null || echo 0)
  fi
  total_base_size=$((total_base_size + s))
done

# Determine additional size from copy targets if possible
additional_size=0
for p in "${COPY_FROM[@]}"; do
  # normalize possible leading slash
  pp=${p#/usr/src/verseguy/}
  if [ -d "$pp" ]; then
    s=$(du -sb "$pp" 2>/dev/null | awk '{print $1}')
    additional_size=$((additional_size + s))
  elif [ -d "./$pp" ]; then
    s=$(du -sb "./$pp" 2>/dev/null | awk '{print $1}')
    additional_size=$((additional_size + s))
  fi
done

# fallback
if [ $additional_size -eq 0 ]; then
  if [ -d target/release/examples ]; then
    additional_size=$(du -sb target/release/examples | awk '{print $1}') || additional_size=0
  else
    additional_size=$((5 * 1024 * 1024))
  fi
fi

final_base=""
if [ ${#FROMS[@]} -gt 0 ]; then final_base=${FROMS[-1]}; fi

# Try to get final base size
final_base_size=0
if [ -n "$final_base" ]; then
  final_base_size=$(docker image inspect --format '{{.Size}}' "$final_base" 2>/dev/null || echo 0)
fi

total_estimated_image_size=$((final_base_size + additional_size))

estimated_download_sec=$(awk -v s=$total_base_size -v bw=$BW_MB 'BEGIN{print int((s/1024/1024)/bw)}')
estimated_build_sec=$(awk -v s=$additional_size -v coeff=$BUILD_SEC_PER_MB 'BEGIN{print int((s/1024/1024)*coeff)+30}')
estimated_total_sec=$((estimated_download_sec + estimated_build_sec))

format_bytes() { if [ "$1" -ge $((1024*1024*1024)) ]; then printf "%.2f GB" "$(awk -v b=$1 'BEGIN{print b/1024/1024/1024}')"; elif [ "$1" -ge $((1024*1024)) ]; then printf "%.2f MB" "$(awk -v b=$1 'BEGIN{print b/1024/1024}')"; else printf "%d B" "$1"; fi }

format_time() { printf "%s" "$(date -ud "@${1}" +'%H:%M:%S')" }

echo "Erwartete Image-Größe: $(format_bytes $total_estimated_image_size) (Basis: $(format_bytes $final_base_size), zusätzliche Dateien: $(format_bytes $additional_size))"
echo "Geschätzte Dauer: $(format_time $estimated_total_sec) (Netzwerkannahme: $BW_MB MB/s)"

# Run docker build and parse Step N/M, estimate time per step
start_ts=$(date +%s)
step_times=()
current_step=0
total_steps=0
last_step_ts=0

docker build --progress=plain "$@" 2>&1 | while IFS= read -r line; do
  if [[ "$line" =~ Step[[:space:]]([0-9]+)/([0-9]+) ]]; then
    n=${BASH_REMATCH[1]}
    total=${BASH_REMATCH[2]}
    if [ $total_steps -eq 0 ]; then total_steps=$total; fi
    if [ $current_step -ne $n ]; then
      now_ts=$(date +%s)
      if [ $last_step_ts -ne 0 ]; then
        delta=$((now_ts - last_step_ts))
        step_times+=( "$delta" )
      fi
      last_step_ts=$now_ts
      current_step=$n
    fi
    # compute average
    sum=0
    for v in "${step_times[@]}"; do sum=$((sum + v)); done
    avg=0
    if [ ${#step_times[@]} -gt 0 ]; then avg=$((sum / ${#step_times[@]})); fi
    elapsed=$(( $(date +%s) - start_ts ))
    remaining_steps=$(( total - current_step ))
    estRem=$(( avg * remaining_steps + estimated_build_sec - sum ))
    if [ $estRem -lt 0 ]; then estRem=0; fi
    estTotal=$(( elapsed + estRem ))
    pct=0
    if [ $total -gt 0 ]; then pct=$(( current_step * 100 / total )); fi
    printf "\rProgress: %s/%s (%d%%) - Zeit: %s / %s (verbleibend ~%s)\033[K" "$current_step" "$total" "$pct" "$(date -ud @${elapsed} +'%H:%M:%S')" "$(date -ud @${estTotal} +'%H:%M:%S')" "$(date -ud @${estRem} +'%H:%M:%S')"
  else
    printf "\n"; echo "$line";
  fi
done

echo
