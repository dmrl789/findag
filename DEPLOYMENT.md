# FinDAG Two-Node Deployment Guide

This guide explains how to deploy two FinDAG nodes (New York and London) with transaction bots on separate servers.

## Architecture

- **New York Node**: Ports 8080 (API), 9898 (Metrics), 9000 (P2P)
- **London Node**: Ports 8081 (API), 9899 (Metrics), 9001 (P2P)
- **Transaction Bots**: Each node runs its own bot generating transactions
- **Peer Communication**: Nodes communicate via HTTP API

## Prerequisites

- Docker and Docker Compose installed on both servers
- Network connectivity between servers
- Ports 8080, 9898, 9000 (NY) and 8081, 9899, 9001 (London) open

## Deployment Steps

### 1. New York Server

```bash
# Clone the repository
git clone <your-repo-url>
cd FinDAG

# Make deployment script executable
chmod +x deploy-ny.sh

# Edit the script to set London server IP
nano deploy-ny.sh
# Replace YOUR_LONDON_SERVER_IP with actual London server IP

# Run deployment
./deploy-ny.sh
```

### 2. London Server

```bash
# Clone the repository
git clone <your-repo-url>
cd FinDAG

# Make deployment script executable
chmod +x deploy-london.sh

# Edit the script to set NY server IP
nano deploy-london.sh
# Replace YOUR_NY_SERVER_IP with actual NY server IP

# Run deployment
./deploy-london.sh
```

## Configuration

### Environment Variables

- `FINDAG_BIND_ADDR`: Node binding address (default: 0.0.0.0:8080)
- `FINDAG_PEERS`: Comma-separated list of peer addresses
- `NODE_ID`: Unique node identifier
- `NODE_REGION`: Node region (ny/london)

### Port Mapping

| Service | NY Node | London Node |
|---------|---------|-------------|
| HTTP API | 8080 | 8081 |
| Metrics | 9898 | 9899 |
| P2P | 9000 | 9001 |

## Monitoring

### Health Checks

- NY Node: `http://ny-server:8080/health`
- London Node: `http://london-server:8081/health`

### Node Information

- NY Node: `http://ny-server:8080/node/info`
- London Node: `http://london-server:8081/node/info`

### Transaction Monitoring

- NY Transactions: `http://ny-server:8080/transactions`
- London Transactions: `http://london-server:8081/transactions`

### Block Monitoring

- NY Blocks: `http://ny-server:8080/blocks`
- London Blocks: `http://london-server:8081/blocks`

## Transaction Bot

Each node runs a transaction bot that:
- Generates transactions every 1 second
- Uses 4 test addresses (alice, bob, charlie, diana)
- Sends random amounts between 1-1000
- Automatically submits transactions to the local node

## Block Production

- Blocks are produced every 5 seconds
- Maximum 10 transactions per block
- Skip-when-empty: No blocks produced if no transactions

## Troubleshooting

### Check Container Status

```bash
# On NY server
docker ps | grep findag-ny

# On London server
docker ps | grep findag-london
```

### View Logs

```bash
# NY node logs
docker logs findag-ny

# London node logs
docker logs findag-london
```

### Restart Services

```bash
# Restart NY node
docker-compose restart findag-ny

# Restart London node
docker-compose restart findag-london
```

### Network Issues

If nodes can't communicate:
1. Check firewall settings
2. Verify peer IP addresses in deployment scripts
3. Test connectivity: `curl http://peer-server:port/health`

## Performance

- **Block Production**: 1 block every 5 seconds
- **Transaction Generation**: 1 transaction per second per node
- **Expected Throughput**: ~2 TPS (1 per node)
- **Block Size**: Up to 10 transactions per block

## Scaling

To add more nodes:
1. Create new deployment script
2. Update peer lists in existing nodes
3. Use unique ports for each node
4. Update docker-compose.yml with new service 