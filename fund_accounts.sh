#!/bin/bash

# === Fund test accounts script ===
echo "💰 Funding test accounts..."

# === Test accounts to fund ===
TEST_ACCOUNTS=(
  "fdg1qalice1234567890"
  "fdg1qbob1234567890" 
  "fdg1qcharlie1234567890"
  "fdg1qdiana1234567890"
  "fdg1qedward1234567890"
  "fdg1qbot301129"
  "fdg1qbot519950"
  "fdg1qbot260960"
  "fdg1qbot730226"
  "fdg1qbot821642"
)

# === Initial balance for each account ===
INITIAL_BALANCE=10000

# === Node API endpoint ===
NODE_URL="http://127.0.0.1:3000"

echo "🎯 Funding ${#TEST_ACCOUNTS[@]} accounts with $INITIAL_BALANCE USD each..."

for account in "${TEST_ACCOUNTS[@]}"; do
  echo "💰 Funding account: $account"
  
  # Create funding transaction
  curl -X POST "$NODE_URL/tx" \
    -H "Content-Type: application/json" \
    -d "{
      \"from\": \"system\",
      \"to\": \"$account\",
      \"amount\": $INITIAL_BALANCE,
      \"currency\": \"USD\",
      \"shard_id\": 0
    }" \
    --silent --show-error
  
  if [ $? -eq 0 ]; then
    echo "✅ Funded $account"
  else
    echo "❌ Failed to fund $account"
  fi
  
  # Small delay between requests
  sleep 0.1
done

echo "🎉 Account funding complete!"
echo "📊 Check balances with: curl $NODE_URL/balance/{account}" 