# Test transaction script for FinDAG
# Usage: .\test-transaction.ps1 [node_url]

param(
    [string]$NodeUrl = "http://192.168.1.44:8080"
)

Write-Host "Sending test transaction to $NodeUrl"

# Create a test transaction
$transaction = @{
    from = @{0 = "fdg1test1"}
    to = @{0 = "fdg1test2"}
    amount = 100
    payload = @()
    findag_time = [DateTimeOffset]::Now.ToUnixTimeSeconds()
    hashtimer = @(0..31)
    signature = @(0..63)
    public_key = @(0..31)
    shard_id = @{0 = 0}
    source_shard = $null
    dest_shard = $null
    target_chain = $null
    bridge_protocol = $null
}

try {
    $jsonBody = $transaction | ConvertTo-Json -Depth 10
    Write-Host "Transaction JSON: $jsonBody"
    
    $response = Invoke-RestMethod -Uri "$NodeUrl/transactions" -Method POST -Body $jsonBody -ContentType "application/json"
    Write-Host "Transaction sent successfully! Response: $response" -ForegroundColor Green
} catch {
    Write-Host "Error sending transaction: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "Response: $($_.Exception.Response)" -ForegroundColor Red
}

# Also test the health endpoint
try {
    $health = Invoke-RestMethod -Uri "$NodeUrl/health" -Method GET
    Write-Host "Node health: $health" -ForegroundColor Green
} catch {
    Write-Host "Health check failed: $($_.Exception.Message)" -ForegroundColor Red
}

# Test node info
try {
    $nodeInfo = Invoke-RestMethod -Uri "$NodeUrl/node/info" -Method GET
    Write-Host "Node info: $($nodeInfo | ConvertTo-Json)" -ForegroundColor Green
} catch {
    Write-Host "Node info failed: $($_.Exception.Message)" -ForegroundColor Red
} 