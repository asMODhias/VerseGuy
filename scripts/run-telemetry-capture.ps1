# Start capture server and run telemetry-e2e test against it
param()

$ErrorActionPreference = 'Stop'

Write-Host "Starting capture server (background) on port 4319..."
$env:CAPTURE_PORT='4319'
$proc = Start-Process -NoNewWindow -FilePath cargo -ArgumentList 'run -p telemetry-e2e --bin capture_server' -PassThru

# Wait for port 4319 to become available
Write-Host "Waiting for capture server to be ready on 127.0.0.1:4319..."
$max = 30
for ($i=0; $i -lt $max; $i++) {
    try {
        $tcp = New-Object System.Net.Sockets.TcpClient
        $async = $tcp.BeginConnect('127.0.0.1', 4319, $null, $null)
        $ok = $async.AsyncWaitHandle.WaitOne(1000)
        if ($ok -and $tcp.Connected) { $tcp.EndConnect($async); $tcp.Close(); break }
    } catch { }
    Start-Sleep -Seconds 1
}
Write-Host "Starting telemetry e2e test (HTTP exporter)"
$env:TELEMETRY_DEBUG='1'
$env:OTLP_USE_HTTP='1'
$env:OTEL_EXPORTER_OTLP_INSECURE='1'
$env:OTLP_ENDPOINT='http://127.0.0.1:4319'
$env:RUST_LOG='opentelemetry_otlp=trace,reqwest=trace,opentelemetry_http=trace'

cargo test -p telemetry-e2e --test e2e -- --nocapture | Tee-Object -FilePath target/telemetry_e2e_run.log

Write-Host "Fetching captured requests (GET /dump)"
$port = $env:CAPTURE_PORT
if ([string]::IsNullOrEmpty($port)) { $port = '4318' }
Write-Host "Fetching dump from port $port"
Invoke-RestMethod -Uri "http://127.0.0.1:$port/dump" -Method GET -OutFile target/otlp_capture_dump.json
Write-Host "Saved capture dump to target/otlp_capture_dump.json (from port $port)"

# Kill capture server
Write-Host "Stopping capture server (PID $($proc.Id))"
Stop-Process -Id $proc.Id -ErrorAction SilentlyContinue
Write-Host "Done."