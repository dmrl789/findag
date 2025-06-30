# FinDAG Transaction Bot for Windows
# Sends test transactions to FinDAG nodes

param(
    [Parameter(Mandatory=$true)]
    [string]$NodeUrl,
    
    [Parameter(Mandatory=$true)]
    [string]$NodeName,
    
    [int]$Interval = 3
)

Write-Host "🤖 Starting FinDAG Transaction Bot" -ForegroundColor Green
Write-Host "📡 Node URL: $NodeUrl" -ForegroundColor Cyan
Write-Host "🏷️  Node Name: $NodeName" -ForegroundColor Cyan
Write-Host "⏱️  Interval: $Interval seconds" -ForegroundColor Cyan
Write-Host "🔄 Press Ctrl+C to stop" -ForegroundColor Yellow
Write-Host "-" * 50

$txCount = 0
$session = New-Object Microsoft.PowerShell.Commands.WebRequestSession

function Create-TestTransaction {
    $random = Get-Random
    $fromAddr = "fdg1q$($random.ToString('000000'))"
    $toAddr = "fdg1q$((Get-Random).ToString('000000'))"
    $amount = Get-Random -Minimum 1 -Maximum 1001
    $payload = "Test transaction from $NodeName"
    $findagTime = [long]([DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds() * 1000)  # Microseconds
    $hashtimer = [byte[]](1..32 | ForEach-Object { Get-Random -Minimum 0 -Maximum 256 })
    
    return @{
        from = $fromAddr
        to = $toAddr
        amount = $amount
        payload = [System.Text.Encoding]::UTF8.GetBytes($payload) | ForEach-Object { $_.ToString('x2') } | Join-String
        findag_time = $findagTime
        hashtimer = $hashtimer
        shard_id = @{ "0" = 0 }
    }
}

function Send-Transaction {
    try {
        $txData = Create-TestTransaction
        $jsonBody = $txData | ConvertTo-Json -Depth 10
        
        $response = Invoke-WebRequest -Uri "$NodeUrl/transactions" -Method POST -Body $jsonBody -ContentType "application/json" -TimeoutSec 5 -WebSession $session -UseBasicParsing
        
        $script:txCount++
        
        if ($response.StatusCode -eq 200) {
            Write-Host "[$NodeName] ✅ Transaction #$txCount sent: $($txData.amount) tokens" -ForegroundColor Green
            return $true
        } else {
            Write-Host "[$NodeName] ❌ Failed to send transaction: $($response.StatusCode)" -ForegroundColor Red
            return $false
        }
    } catch {
        Write-Host "[$NodeName] ❌ Error sending transaction: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
}

function Get-NodeInfo {
    try {
        $response = Invoke-WebRequest -Uri "$NodeUrl/node/info" -TimeoutSec 5 -WebSession $session -UseBasicParsing
        if ($response.StatusCode -eq 200) {
            return $response.Content | ConvertFrom-Json
        }
    } catch {
        # Ignore errors for node info
    }
    return $null
}

# Main loop
while ($true) {
    # Get and display node info every 10 transactions
    if ($txCount % 10 -eq 0 -and $txCount -gt 0) {
        $info = Get-NodeInfo
        if ($info) {
            $blockCount = $info.block_count
            $peerCount = if ($info.peers) { $info.peers.Count } else { 0 }
            Write-Host "[$NodeName] 📊 Node Info: $blockCount blocks, $peerCount peers" -ForegroundColor Cyan
        }
    }
    
    Send-Transaction
    Start-Sleep -Seconds $Interval
} 