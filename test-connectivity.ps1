# Test connectivity to FinDAG nodes
# Usage: .\test-connectivity.ps1

Write-Host "Testing FinDAG node connectivity..." -ForegroundColor Yellow

$nodes = @(
    @{Name = "Laptop 1"; Url = "http://192.168.1.44:8080"},
    @{Name = "Laptop 2"; Url = "http://192.168.1.20:8080"}
)

foreach ($node in $nodes) {
    Write-Host "`nTesting $($node.Name) at $($node.Url)..." -ForegroundColor Cyan
    
    # Test health endpoint
    try {
        $health = Invoke-RestMethod -Uri "$($node.Url)/health" -Method GET -TimeoutSec 5
        Write-Host "✓ Health check passed: $health" -ForegroundColor Green
    } catch {
        Write-Host "✗ Health check failed: $($_.Exception.Message)" -ForegroundColor Red
        continue
    }
    
    # Test node info endpoint
    try {
        $nodeInfo = Invoke-RestMethod -Uri "$($node.Url)/node/info" -Method GET -TimeoutSec 5
        Write-Host "✓ Node info:" -ForegroundColor Green
        Write-Host "  Address: $($nodeInfo.address)" -ForegroundColor White
        Write-Host "  Block count: $($nodeInfo.block_count)" -ForegroundColor White
        Write-Host "  Peers: $($nodeInfo.peers.Count)" -ForegroundColor White
    } catch {
        Write-Host "✗ Node info failed: $($_.Exception.Message)" -ForegroundColor Red
    }
    
    # Test DAG endpoint
    try {
        $dagInfo = Invoke-RestMethod -Uri "$($node.Url)/dag" -Method GET -TimeoutSec 5
        Write-Host "✓ DAG info:" -ForegroundColor Green
        Write-Host "  Total blocks: $($dagInfo.total_blocks)" -ForegroundColor White
        Write-Host "  Tip blocks: $($dagInfo.tip_blocks)" -ForegroundColor White
        Write-Host "  Max depth: $($dagInfo.max_depth)" -ForegroundColor White
    } catch {
        Write-Host "✗ DAG info failed: $($_.Exception.Message)" -ForegroundColor Red
    }
    
    # Test blocks endpoint
    try {
        $blocks = Invoke-RestMethod -Uri "$($node.Url)/blocks" -Method GET -TimeoutSec 5
        Write-Host "✓ Blocks count: $($blocks.Count)" -ForegroundColor Green
    } catch {
        Write-Host "✗ Blocks endpoint failed: $($_.Exception.Message)" -ForegroundColor Red
    }
}

Write-Host "`nConnectivity test complete!" -ForegroundColor Yellow 