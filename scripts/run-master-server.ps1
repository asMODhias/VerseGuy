param(
    [string]$dbPath = "./master_server_db",
    [string]$secret = "master-secret",
    [int]$port = 3000
)

$env:MASTER_DB_PATH = $dbPath
$env:MASTER_LICENSE_SECRET = $secret

Write-Host "Starting master-server on port $port (DB: $dbPath)"
# Run cargo with feature
cargo run -p master_server --features run-server -- "--" --port $port
