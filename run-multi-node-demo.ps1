# FinDAG Multi-Node Demo: Full Automation
# This script automates a 3-node, 3-bot demo with logging and monitoring

$ErrorActionPreference = 'Stop'

Write-Host "🚀 FinDAG Multi-Node Demo (Automated)" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Green

# 1. Kill old processes
Write-Host "🛑 Killing old findag and bot processes..." -ForegroundColor Yellow
try { taskkill /F /IM findag.exe 2>$null } catch {}
try { taskkill /F /IM transaction_bot.exe 2>$null } catch {}
Start-Sleep -Seconds 2

# 2. Clean up state and logs
Write-Host "🧹 Cleaning up old state and logs..." -ForegroundColor Yellow
Remove-Item -Recurse -Force nodes 2>$null
Remove-Item -Recurse -Force logs 2>$null
New-Item -ItemType Directory -Path "nodes/node1" -Force | Out-Null
New-Item -ItemType Directory -Path "nodes/node2" -Force | Out-Null
New-Item -ItemType Directory -Path "nodes/node3" -Force | Out-Null
New-Item -ItemType Directory -Path "logs" -Force | Out-Null

# 3. Build the project
Write-Host "🔨 Building FinDAG..." -ForegroundColor Yellow
cargo build
if ($LASTEXITCODE -ne 0) { Write-Host "❌ Build failed! Exiting." -ForegroundColor Red; exit 1 }
Write-Host "✅ Build successful!" -ForegroundColor Green

# 4. Start 3 nodes in background, each with unique ports and data dir
$nodes = @(
    @{ name = "Node1"; http = 3001; udp = 9001; data = "nodes/node1"; peers = "127.0.0.1:9002,127.0.0.1:9003"; log = "logs/node1.log" },
    @{ name = "Node2"; http = 3002; udp = 9002; data = "nodes/node2"; peers = "127.0.0.1:9001,127.0.0.1:9003"; log = "logs/node2.log" },
    @{ name = "Node3"; http = 3003; udp = 9003; data = "nodes/node3"; peers = "127.0.0.1:9001,127.0.0.1:9002"; log = "logs/node3.log" }
)

Write-Host "🌐 Starting 3 nodes in background..." -ForegroundColor Yellow
foreach ($n in $nodes) {
    $env:FINDAG_HTTP_PORT = $n.http
    $env:FINDAG_UDP_PORT = $n.udp
    $env:FINDAG_PEERS = $n.peers
    $env:FINDAG_DATA_DIR = $n.data
    Start-Process -FilePath "target\debug\findag.exe" -RedirectStandardOutput $n.log -RedirectStandardError $n.log -WindowStyle Minimized
    Write-Host "  ➡️  $($n.name) on HTTP $($n.http), UDP $($n.udp) [log: $($n.log)]" -ForegroundColor Cyan
}

# 5. Wait for all HTTP ports to be open
Write-Host "⏳ Waiting for all nodes to be ready..." -ForegroundColor Yellow
foreach ($n in $nodes) {
    $ready = $false
    for ($i=0; $i -lt 30; $i++) {
        $test = Test-NetConnection -ComputerName 127.0.0.1 -Port $n.http
        if ($test.TcpTestSucceeded) { $ready = $true; break }
        Start-Sleep -Seconds 1
    }
    if ($ready) {
        Write-Host "  ✅ $($n.name) ready on port $($n.http)" -ForegroundColor Green
    } else {
        Write-Host "  ❌ $($n.name) did not start on port $($n.http)!" -ForegroundColor Red
        exit 1
    }
}

# 6. Fund accounts on all nodes
Write-Host "💰 Funding accounts on all nodes..." -ForegroundColor Yellow
foreach ($n in $nodes) {
    $env:FINDAG_NODE_URL = "http://127.0.0.1:$($n.http)"
    cargo run --bin fund_accounts | Tee-Object -FilePath "logs/fund_$($n.name).log"
    Write-Host "  💸 Funded $($n.name) [log: logs/fund_$($n.name).log]" -ForegroundColor Cyan
}

# 7. Start 3 bots in background, each targeting a different node
$bots = @(
    @{ name = "Bot1"; url = "http://127.0.0.1:3001"; log = "logs/bot1.log" },
    @{ name = "Bot2"; url = "http://127.0.0.1:3002"; log = "logs/bot2.log" },
    @{ name = "Bot3"; url = "http://127.0.0.1:3003"; log = "logs/bot3.log" }
)
Write-Host "🤖 Starting 3 transaction bots in background..." -ForegroundColor Yellow
foreach ($b in $bots) {
    $env:FINDAG_BOT_TARGET_URL = $b.url
    Start-Process -FilePath "target\debug\transaction_bot.exe" -ArgumentList "start" -RedirectStandardOutput $b.log -RedirectStandardError $b.log -WindowStyle Minimized
    Write-Host "  ➡️  $($b.name) targeting $($b.url) [log: $($b.log)]" -ForegroundColor Cyan
}

Write-Host "\n🎉 Multi-Node Demo is Running!" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Green
Write-Host "\n📊 Node logs: logs/node1.log, logs/node2.log, logs/node3.log" -ForegroundColor White
Write-Host "🤖 Bot logs: logs/bot1.log, logs/bot2.log, logs/bot3.log" -ForegroundColor White
Write-Host "\n🔍 Monitor blocks, rounds, and transactions in the log files." -ForegroundColor Cyan
Write-Host "\n⏹️  To stop everything: taskkill /F /IM findag.exe; taskkill /F /IM transaction_bot.exe" -ForegroundColor Red
Write-Host "\n" 