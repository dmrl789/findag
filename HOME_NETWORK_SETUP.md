# FinDAG Home Network Setup Guide

## 🏠 Two-Laptop Configuration
- **Laptop 1 (NY)**: 192.168.1.44
- **Laptop 2 (London)**: 192.168.1.20
- **HTTP Port**: 8080
- **P2P Port**: 9000

## 📋 Prerequisites

### On Both Laptops:
1. **Rust** installed (https://rustup.rs/)
2. **Git** for cloning the repository
3. **Windows PowerShell** (for Windows) or **Bash** (for Linux/Mac)
4. **Network connectivity** between laptops

## 🚀 Step-by-Step Setup

### Step 1: Clone and Build (Both Laptops)

```powershell
# Clone the repository (if not already done)
git clone <repository-url>
cd FinDAG

# Build the project
cargo build --release
```

### Step 2: Test Connectivity

First, test that both laptops can communicate:

```powershell
# Run the connectivity test
.\test-connectivity.ps1
```

This will:
- ✅ Test ping connectivity between laptops
- ✅ Check if required ports are available
- ✅ Verify if FinDAG is already running
- ✅ Identify which laptop you're on

### Step 3: Start FinDAG Nodes

#### On Laptop 1 (192.168.1.44):
```powershell
# Start the FinDAG node
.\start-findag-windows.ps1
```

#### On Laptop 2 (192.168.1.20):
```powershell
# Start the FinDAG node
.\start-findag-windows.ps1
```

You should see output like:
```
🚀 Starting FinDAG Node...
📍 Current IP: 192.168.1.44
🏙️  Detected: Laptop 1 (NY)
📡 HTTP API: http://0.0.0.0:8080
🌐 P2P Bind: 0.0.0.0:9000
👥 Peers: 192.168.1.20:9000
🏷️  Node Name: NY-Laptop1
```

### Step 4: Start Transaction Bots

In **new terminal windows** on each laptop:

#### On Laptop 1:
```powershell
.\transaction-bot-windows.ps1 -NodeUrl "http://192.168.1.44:8080" -NodeName "NY-Laptop1"
```

#### On Laptop 2:
```powershell
.\transaction-bot-windows.ps1 -NodeUrl "http://192.168.1.20:8080" -NodeName "London-Laptop2"
```

## 🔍 Testing the Setup

### 1. Health Checks
Test that both nodes are running:

```powershell
# Test Laptop 1
Invoke-WebRequest -Uri "http://192.168.1.44:8080/health"

# Test Laptop 2
Invoke-WebRequest -Uri "http://192.168.1.20:8080/health"
```

### 2. Node Information
Check node status and peer connections:

```powershell
# Laptop 1 info
Invoke-WebRequest -Uri "http://192.168.1.44:8080/node/info" | ConvertFrom-Json

# Laptop 2 info
Invoke-WebRequest -Uri "http://192.168.1.20:8080/node/info" | ConvertFrom-Json
```

### 3. View Blocks
See the DAG growing:

```powershell
# View blocks on Laptop 1
Invoke-WebRequest -Uri "http://192.168.1.44:8080/blocks" | ConvertFrom-Json

# View blocks on Laptop 2
Invoke-WebRequest -Uri "http://192.168.1.20:8080/blocks" | ConvertFrom-Json
```

### 4. DAG Statistics
Monitor DAG growth:

```powershell
# DAG stats on Laptop 1
Invoke-WebRequest -Uri "http://192.168.1.44:8080/dag" | ConvertFrom-Json

# DAG stats on Laptop 2
Invoke-WebRequest -Uri "http://192.168.1.20:8080/dag" | ConvertFrom-Json
```

## 🌐 Web Interface

You can also access the web interface in your browser:

- **Laptop 1**: http://192.168.1.44:8080
- **Laptop 2**: http://192.168.1.20:8080

## 📊 What to Expect

### Initial Setup:
1. **Node Discovery**: Both nodes should discover each other via P2P
2. **Block Production**: Each node produces blocks every ~1 second
3. **Transaction Propagation**: Transactions sent to one node appear on the other
4. **DAG Growth**: You'll see the DAG structure growing with multiple blocks

### Normal Operation:
- **Block Count**: Should increase over time
- **Peer Count**: Should show 1 peer (the other laptop)
- **Transaction Count**: Should increase as bots send transactions
- **DAG Depth**: Should grow as more blocks are added

### Expected Output:
```
[NY-Laptop1] ✅ Transaction #1 sent: 456 tokens
[NY-Laptop1] ✅ Transaction #2 sent: 123 tokens
[NY-Laptop1] 📊 Node Info: 15 blocks, 1 peers
[NY-Laptop1] ✅ Transaction #3 sent: 789 tokens
```

## 🛠️ Troubleshooting

### Connection Issues:
1. **Ping Test**: Ensure both laptops can ping each other
2. **Firewall**: Allow UDP traffic on port 9000 between laptops
3. **Port Conflicts**: Make sure ports 8080 and 9000 are not in use

### Node Won't Start:
1. **Check Binary**: Ensure `cargo build --release` completed successfully
2. **Check Ports**: Verify ports 8080 and 9000 are available
3. **Check Logs**: Look for error messages in the terminal output

### No Peer Connection:
1. **Network**: Ensure both laptops are on the same network
2. **IP Addresses**: Verify IP addresses are correct
3. **P2P Port**: Check that UDP port 9000 is not blocked

### Transactions Not Propagating:
1. **Node Health**: Check that both nodes are healthy
2. **Peer Count**: Verify peer count shows 1 on both nodes
3. **Network**: Test basic connectivity between laptops

## 🔧 Advanced Configuration

### Custom Ports:
If you need to use different ports, modify the scripts:
- `start-findag-windows.ps1`: Change `$HTTP_PORT` and `$P2P_PORT`
- `transaction-bot-windows.ps1`: Update the `$NodeUrl` parameter

### Multiple Peers:
To add more laptops, update the peer configuration in `start-findag-windows.ps1`:
```powershell
$PEERS = "192.168.1.20:9000,192.168.1.45:9000,192.168.1.46:9000"
```

### Monitoring:
For continuous monitoring, you can create a simple monitoring script:
```powershell
while ($true) {
    Clear-Host
    Write-Host "FinDAG Network Status - $(Get-Date)"
    Write-Host "Laptop 1: $(Invoke-WebRequest -Uri 'http://192.168.1.44:8080/health' -UseBasicParsing).StatusCode"
    Write-Host "Laptop 2: $(Invoke-WebRequest -Uri 'http://192.168.1.20:8080/health' -UseBasicParsing).StatusCode"
    Start-Sleep -Seconds 5
}
```

## 🎯 Success Indicators

You'll know the setup is working when:
- ✅ Both nodes show "Healthy" status
- ✅ Peer count shows 1 on both nodes
- ✅ Block count increases on both nodes
- ✅ Transactions sent to one node appear on the other
- ✅ DAG statistics show growing depth and width
- ✅ No error messages in the terminal output

## 📞 Support

If you encounter issues:
1. Check the troubleshooting section above
2. Verify network connectivity between laptops
3. Ensure all prerequisites are installed
4. Check that ports are not blocked by firewall
5. Review terminal output for error messages

Happy DAG testing! 🚀 