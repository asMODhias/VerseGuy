param(
    [Parameter(ValueFromRemainingArguments=$true)]
    [string[]]
    $DockerArgs
)

if (-not $DockerArgs -or $DockerArgs.Count -eq 0) {
    Write-Host "Usage: .\scripts\docker-build-wrapper.ps1 -- [docker build args...]"
    exit 1
}

# Defaults for estimation (can be overridden via env vars)
$BW_MB = if ($env:DOCKER_BUILD_EST_BW_MB) { [int]$env:DOCKER_BUILD_EST_BW_MB } else { 10 }
$BUILD_COEFF_SEC_PER_MB = if ($env:DOCKER_BUILD_SEC_PER_MB) { [double]$env:DOCKER_BUILD_SEC_PER_MB } else { 0.5 }

function Get-DockerfilePathFromArgs {
    param($Args)
    for ($i=0; $i -lt $Args.Count; $i++) {
        if ($Args[$i] -eq '-f' -or $Args[$i] -eq '--file') {
            if ($i+1 -lt $Args.Count) { return $Args[$i+1] }
        }
    }
    return 'Dockerfile'
}

function Parse-Dockerfile {
    param($Path)
    $froms = @()
    $copyTargets = @()
    if (-not (Test-Path $Path)) { return @{Froms=$froms; CopyTargets=$copyTargets} }
    $lines = Get-Content $Path
    foreach ($l in $lines) {
        if ($l -match '^FROM\s+([^\s]+)') { $froms += $matches[1] }
        if ($l -match 'COPY\s+--from=[^\s]+\s+([^\s]+)\s+([^\s]+)') { $copyTargets += $matches[1] }
        if ($l -match 'COPY\s+--from=[^\s]+\s+([^\s]+)\s+([^\s]+)') { $copyTargets += $matches[1] }
    }
    return @{Froms = $froms | Select-Object -Unique; CopyTargets = $copyTargets | Select-Object -Unique}
}

function Get-ImageSizeBytes {
    param($image)
    $size = 0
    try {
        $out = docker image inspect --format '{{.Size}}' $image 2>&1
        if ($LASTEXITCODE -eq 0 -and $out.Trim()) { $size = [int64]$out.Trim() }
        else { throw "no local image" }
    } catch {
        Write-Host "Image $image not found locally; pulling for size estimation..." -ForegroundColor Yellow
        docker pull $image | ForEach-Object { Write-Host $_ }
        $out = docker image inspect --format '{{.Size}}' $image 2>&1
        if ($LASTEXITCODE -eq 0 -and $out.Trim()) { $size = [int64]$out.Trim() }
    }
    return $size
}

function Get-LocalPathSizeBytes {
    param($path)
    if (-not (Test-Path $path)) { return 0 }
    $fi = Get-ChildItem $path -Recurse -File -ErrorAction SilentlyContinue
    if (-not $fi) { return 0 }
    return ($fi | Measure-Object -Property Length -Sum).Sum
}

# Determine Dockerfile to use
$dockerfile = Get-DockerfilePathFromArgs -Args $DockerArgs
$dockerfilePath = if ((Test-Path $dockerfile)) { $dockerfile } else { Join-Path (Get-Location) $dockerfile }
$info = Parse-Dockerfile -Path $dockerfilePath
$froms = $info.Froms
$copyTargets = $info.CopyTargets

# Compute base image sizes (bytes)
$baseSizes = @{}
$totalBaseSize = 0
foreach ($img in $froms) {
    $s = 0
    try { $s = Get-ImageSizeBytes -image $img } catch { Write-Host "Failed to get size for $img" -ForegroundColor Yellow }
    $baseSizes[$img] = $s
    $totalBaseSize += $s
}

# Try to infer the final-stage additional size: prefer specific copy-from-builder paths
$additionalSize = 0
if ($copyTargets.Count -gt 0) {
    foreach ($t in $copyTargets) {
        # If the path is absolute inside builder, try to map to local repo path
        $candidate = $t -replace '^/usr/src/verseguy/', ''
        if (Test-Path $candidate) { $additionalSize += Get-LocalPathSizeBytes -path $candidate }
        elseif (Test-Path ".\$candidate") { $additionalSize += Get-LocalPathSizeBytes -path ".\$candidate" }
    }
}

# Fallback: if no copyTargets detected, try common locations (target/release/examples)
if ($additionalSize -eq 0) {
    $fallback = 'target\release\examples'
    if (Test-Path $fallback) { $additionalSize = Get-LocalPathSizeBytes -path $fallback }
}

if ($additionalSize -eq 0) { $additionalSize = 5MB }

# Choose final base image as the last FROM
$finalBase = if ($froms.Count -gt 0) { $froms[-1] } else { '' }
$finalBaseSize = if ($finalBase -and $baseSizes.ContainsKey($finalBase)) { $baseSizes[$finalBase] } else { 0 }

$totalEstimatedImageSize = $finalBaseSize + $additionalSize

# Estimate duration (simple heuristic)
$downloadSize = 0
# Sum sizes of images that are not present locally (we can estimate by checking presence: if size==0 in baseSizes it's not available)
foreach ($kv in $baseSizes.GetEnumerator()) { if ($kv.Value -gt 0) { } else { $downloadSize += $kv.Value } }
# For simplicity, assume we may need to pull all base images if not present
$estimatedDownloadSec = [math]::Ceiling(($totalBaseSize / 1MB) / $BW_MB)
$estimatedBuildSec = [math]::Ceiling(($additionalSize / 1MB) * $BUILD_COEFF_SEC_PER_MB) + 30
$estimatedTotalSec = $estimatedDownloadSec + $estimatedBuildSec

function Format-Bytes([int64]$b) {
    if ($b -ge 1GB) { return "{0:N2} GB" -f ($b/1GB) }
    if ($b -ge 1MB) { return "{0:N2} MB" -f ($b/1MB) }
    return "{0:N0} B" -f $b
}

function Format-Time([int]$s) {
    [timespan]$t = [timespan]::FromSeconds($s)
    return $t.ToString()
}

Write-Host "Erwartete Image-Größe: $(Format-Bytes $totalEstimatedImageSize) (Basis: $(Format-Bytes $finalBaseSize), zusätzliche Dateien: $(Format-Bytes $additionalSize))"
Write-Host "Geschätzte Dauer: $(Format-Time $estimatedTotalSec) (Netzwerkannahme: $BW_MB MB/s)"

# Now run build and provide live progress with time estimate
$startTime = Get-Date
$stepTimes = @()
$currentStep = 0
$totalSteps = 0
$lastStepChangeTime = $null

docker build --progress=plain @DockerArgs 2>&1 | ForEach-Object {
    $line = $_
    if ($line -match 'Step\s+([0-9]+)/([0-9]+)') {
        $n = [int]$matches[1]
        $total = [int]$matches[2]
        if ($totalSteps -eq 0) { $totalSteps = $total }
        # step change
        if ($currentStep -ne $n) {
            $now = Get-Date
            if ($lastStepChangeTime) {
                $duration = ($now - $lastStepChangeTime).TotalSeconds
                $stepTimes += $duration
            }
            $lastStepChangeTime = $now
            $currentStep = $n
        }
        $avg = if ($stepTimes.Count -gt 0) { [math]::Ceiling(($stepTimes | Measure-Object -Average).Average) } else { 0 }
        $elapsed = (Get-Date) - $startTime
        $remainingSteps = [math]::Max(0, $total - $currentStep)
        $estRem = ($avg * $remainingSteps) + ($estimatedBuildSec - ($stepTimes | Measure-Object -Sum).Sum)
        $estTotal = [math]::Ceiling($elapsed.TotalSeconds + $estRem)
        $pct = if ($total -gt 0) { [int](($currentStep * 100) / $total) } else { 0 }
        # Print progress with carriage return
        Write-Host -NoNewline "`rProgress: $currentStep/$total ($pct%) - Zeit: $([timespan]::FromSeconds([math]::Round($elapsed.TotalSeconds))) / $([timespan]::FromSeconds($estTotal)) (verbleibend ~$([timespan]::FromSeconds([math]::Round($estRem)))) `n"
    } else {
        Write-Host $line
    }
}

# final newline
Write-Host ""
