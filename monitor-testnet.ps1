# FinDAG Testnet Monitor
Write-Host "📊 FinDAG Testnet Monitor" -ForegroundColor Green
Write-Host "=========================" -ForegroundColor Green

# Function to check node status
function Check-NodeStatus {
    param(
        [string]$NodeName,
        [string]$NodeUrl
    )
    
    Write-Host "`n🔍 Checking $NodeName ($NodeUrl)..." -ForegroundColor Cyan
    
    try {
        # Check health
        $health = Invoke-WebRequest -Uri "$NodeUrl/health" -UseBasicParsing -TimeoutSec 5
        Write-Host "  ✅ Health: OK" -ForegroundColor Green
        
        # Check blocks
        $blocks = Invoke-WebRequest -Uri "$NodeUrl/blocks" -UseBasicParsing -TimeoutSec 5
        $blocksData = $blocks.Content | ConvertFrom-Json
        Write-Host "  📦 Blocks: $($blocksData.Count)" -ForegroundColor White
        
        # Check mempool
        $mempool = Invoke-WebRequest -Uri "$NodeUrl/mempool-transactions" -UseBasicParsing -TimeoutSec 5
        $mempoolData = $mempool.Content | ConvertFrom-Json
        Write-Host "  📋 Mempool: $($mempoolData.Count) transactions" -ForegroundColor White
        
        # Check mempool status
        $mempoolStatus = Invoke-WebRequest -Uri "$NodeUrl/mempool-status" -UseBasicParsing -TimeoutSec 5
        $statusData = $mempoolStatus.Content | ConvertFrom-Json
        Write-Host "  📊 Pool Size: $($statusData.pool_size)" -ForegroundColor White
        
        return $true
    }
    catch {
        Write-Host "  ❌ Error: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
}

# Check all nodes
$node1Ok = Check-NodeStatus "Node 1" "http://localhost:3001"
$node2Ok = Check-NodeStatus "Node 2" "http://localhost:3002" 
$node3Ok = Check-NodeStatus "Node 3" "http://localhost:3003"

# Check processes
Write-Host "`n🔍 Checking processes..." -ForegroundColor Cyan
$findagProcesses = Get-Process | Where-Object {$_.ProcessName -eq "findag"}
$botProcesses = Get-Process | Where-Object {$_.ProcessName -eq "transaction_bot"}

Write-Host "  🤖 FindAG nodes: $($findagProcesses.Count)" -ForegroundColor White
Write-Host "  🤖 Transaction bots: $($botProcesses.Count)" -ForegroundColor White

# Summary
Write-Host "`n📈 Summary:" -ForegroundColor Magenta
if ($node1Ok -and $node2Ok -and $node3Ok) {
    Write-Host "  ✅ All nodes are healthy" -ForegroundColor Green
} else {
    Write-Host "  ❌ Some nodes are not responding" -ForegroundColor Red
}

if ($findagProcesses.Count -eq 3) {
    Write-Host "  ✅ All 3 nodes are running" -ForegroundColor Green
} else {
    Write-Host "  ❌ Expected 3 nodes, found $($findagProcesses.Count)" -ForegroundColor Red
}

if ($botProcesses.Count -gt 0) {
    Write-Host "  ✅ Transaction bots are running" -ForegroundColor Green
} else {
    Write-Host "  ❌ No transaction bots found" -ForegroundColor Red
}

Write-Host "`n🎯 Next steps:" -ForegroundColor Yellow
Write-Host "  1. Check logs for 'TxPool size: >0'" -ForegroundColor White
Write-Host "  2. Look for 'Transaction added to pool' messages" -ForegroundColor White
Write-Host "  3. Monitor block production at http://localhost:3001/blocks" -ForegroundColor White
Write-Host "  4. Check mempool at http://localhost:3001/mempool-transactions" -ForegroundColor White 