#!/bin/bash

# === Base config ===
FIN_DAG_BINARY="cargo run --bin findag"   # Adjust to your compiled binary path
NETWORK_NAME="finDAGLocalNet"

# === Node configs ===
NODE_COUNT=3

BASE_P2P_PORT=7000
BASE_API_PORT=3000

# === Create config folders ===
echo "🔨 Setting up $NODE_COUNT nodes..."

for i in $(seq 1 $NODE_COUNT); do
  NODE_DIR="nodes/node$i"
  mkdir -p $NODE_DIR

  # === Generate unique ports ===
  P2P_PORT=$((BASE_P2P_PORT + i))
  API_PORT=$((BASE_API_PORT + i))

  echo "✅ Configuring Node $i: P2P=$P2P_PORT API=$API_PORT"

  # === Generate dummy config ===
  cat > $NODE_DIR/config.toml <<EOF
[network]
network_name = "$NETWORK_NAME"
p2p_port = $P2P_PORT
api_port = $API_PORT
node_key_file = "node_key.pem"

[peers]
known_peers = [
  "127.0.0.1:7001",
  "127.0.0.1:7002", 
  "127.0.0.1:7003"
]
EOF

  # === Generate unique node key if missing ===
  if [ ! -f $NODE_DIR/node_key.pem ]; then
    echo "🔑 Generating node key for Node $i"
    openssl genrsa -out $NODE_DIR/node_key.pem 2048
  fi

  # === Create empty db folder ===
  mkdir -p $NODE_DIR/db
done

echo "🚀 Launching nodes..."
sleep 1

# === Run all nodes in separate terminal tabs ===
for i in $(seq 1 $NODE_COUNT); do
  NODE_DIR="nodes/node$i"
  echo "▶️  Starting Node $i..."
  $FIN_DAG_BINARY --config $NODE_DIR/config.toml --db $NODE_DIR/db &
done

echo "✅ All nodes launched. Check logs above!" 