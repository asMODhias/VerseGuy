param(
  [Parameter(Mandatory = $true)] [string]$WebDistPath,
  [Parameter(Mandatory = $true)] [string]$UiProjectPath
)

Write-Host "Copying web assets from $WebDistPath to $UiProjectPath/www"
if (-not (Test-Path $WebDistPath)) { Write-Error "Web dist not found: $WebDistPath"; exit 1 }
if (-not (Test-Path $UiProjectPath)) { Write-Error "UI project path not found: $UiProjectPath"; exit 1 }

$target = Join-Path $UiProjectPath 'www'
if (Test-Path $target) { Remove-Item -Recurse -Force $target }
New-Item -ItemType Directory -Path $target | Out-Null

Get-ChildItem -Path $WebDistPath -Recurse | ForEach-Object {
    $dest = $_.FullName.Replace($WebDistPath, $target)
    if ($_.PSIsContainer) { New-Item -ItemType Directory -Path $dest -Force | Out-Null } else { Copy-Item -Path $_.FullName -Destination $dest -Force }
}
Write-Host "Copied web assets successfully."