# 🚀 FinDAG Deployment Guide

## Overview

This guide covers deploying FinDAG in various environments from development to production, including Docker, Kubernetes, Helm charts, and CI/CD pipelines.

---

## 📋 Prerequisites

### System Requirements

- **CPU**: 4+ cores (8+ for production)
- **RAM**: 8GB+ (16GB+ for production)
- **Storage**: 100GB+ SSD (500GB+ for production)
- **Network**: 100Mbps+ (1Gbps+ for production)

### Software Requirements

- **Docker**: 20.10+ or **Docker Desktop**: 4.0+
- **Kubernetes**: 1.24+ (for K8s deployment)
- **Helm**: 3.8+ (for K8s deployment)
- **Rust**: 1.70+ (for local development)
- **Node.js**: 18+ (for UI development)

---

## 🐳 Docker Deployment

### Quick Start

```bash
# Clone repository
git clone https://github.com/your-org/findag.git
cd findag

# Build and run with Docker Compose
docker-compose up -d
```

### Production Docker Build

```bash
# Build production image
docker build -t findag:latest -f docker/Dockerfile .

# Run with environment variables
docker run -d \
  --name findag-node \
  -p 8080:8080 \
  -p 9090:9090 \
  -e ADMIN_USERNAME=admin \
  -e ADMIN_PASSWORD_HASH=$(echo -n "your_secure_password" | sha256sum | cut -d' ' -f1) \
  -e JWT_SECRET=your_jwt_secret \
  -e DATABASE_PATH=/data/findag.db \
  -v findag-data:/data \
  findag:latest
```

### Docker Compose Configuration

```yaml
# docker-compose.yml
version: '3.8'

services:
  findag-node:
    build: .
    ports:
      - "8080:8080"  # API
      - "9090:9090"  # Metrics
    environment:
      - ADMIN_USERNAME=admin
      - ADMIN_PASSWORD_HASH=${ADMIN_PASSWORD_HASH}
      - JWT_SECRET=${JWT_SECRET}
      - DATABASE_PATH=/data/findag.db
      - NODE_ENV=production
    volumes:
      - findag-data:/data
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9091:9090"
    volumes:
      - ./docker/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus-data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--web.console.libraries=/etc/prometheus/console_libraries'
      - '--web.console.templates=/etc/prometheus/consoles'
      - '--storage.tsdb.retention.time=200h'
      - '--web.enable-lifecycle'

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=${GRAFANA_PASSWORD}
    volumes:
      - grafana-data:/var/lib/grafana
      - ./docker/grafana/dashboards:/etc/grafana/provisioning/dashboards
      - ./docker/grafana/datasources:/etc/grafana/provisioning/datasources

volumes:
  findag-data:
  prometheus-data:
  grafana-data:
```

---

## ☸️ Kubernetes Deployment

### Prerequisites

```bash
# Install Helm
curl https://get.helm.sh/helm-v3.12.0-linux-amd64.tar.gz | tar xz
sudo mv linux-amd64/helm /usr/local/bin/

# Add FinDAG Helm repository
helm repo add findag https://charts.findag.com
helm repo update
```

### Quick Deploy

```bash
# Deploy to default namespace
helm install findag findag/findag \
  --set admin.username=admin \
  --set admin.passwordHash=$(echo -n "your_password" | sha256sum | cut -d' ' -f1) \
  --set jwt.secret=your_jwt_secret

# Deploy to custom namespace
kubectl create namespace findag
helm install findag findag/findag \
  --namespace findag \
  --set admin.username=admin \
  --set admin.passwordHash=$(echo -n "your_password" | sha256sum | cut -d' ' -f1)
```

### Production Values

```yaml
# values-production.yaml
replicaCount: 3

image:
  repository: findag/findag
  tag: "1.0.0"
  pullPolicy: IfNotPresent

admin:
  username: admin
  passwordHash: "your_password_hash"

jwt:
  secret: "your_jwt_secret"
  expiryHours: 24

persistence:
  enabled: true
  storageClass: "fast-ssd"
  size: 100Gi

resources:
  requests:
    memory: "2Gi"
    cpu: "1000m"
  limits:
    memory: "4Gi"
    cpu: "2000m"

service:
  type: LoadBalancer
  port: 8080

monitoring:
  enabled: true
  prometheus:
    enabled: true
  grafana:
    enabled: true
    adminPassword: "your_grafana_password"

ingress:
  enabled: true
  className: "nginx"
  annotations:
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
  hosts:
    - host: api.findag.com
      paths:
        - path: /
          pathType: Prefix
  tls:
    - secretName: findag-tls
      hosts:
        - api.findag.com

security:
  podSecurityContext:
    fsGroup: 1000
  containerSecurityContext:
    runAsNonRoot: true
    runAsUser: 1000
    allowPrivilegeEscalation: false
    readOnlyRootFilesystem: true
```

### Deploy with Production Values

```bash
helm install findag findag/findag \
  --namespace findag \
  -f values-production.yaml
```

---

## 🔧 Environment Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `ADMIN_USERNAME` | Admin username | `admin` | No |
| `ADMIN_PASSWORD_HASH` | Admin password hash | `default_hash` | Yes |
| `JWT_SECRET` | JWT signing secret | `auto_generated` | Yes |
| `DATABASE_PATH` | Database file path | `/data/findag.db` | No |
| `NODE_ENV` | Environment mode | `development` | No |
| `API_PORT` | API server port | `8080` | No |
| `METRICS_PORT` | Metrics server port | `9090` | No |
| `LOG_LEVEL` | Logging level | `info` | No |
| `P2P_PORT` | P2P network port | `30333` | No |
| `BOOTSTRAP_NODES` | Bootstrap node addresses | `[]` | No |

### Configuration Files

#### Database Configuration (`configs/database.toml`)

```toml
[storage]
path = "/data/findag.db"
max_size = "10GB"
compression = true

[performance]
cache_size = "1GB"
write_buffer_size = "64MB"
max_write_buffer_number = 4
```

#### Production Configuration (`configs/production.toml`)

```toml
[network]
p2p_port = 30333
max_peers = 50
bootstrap_nodes = [
    "node1.findag.com:30333",
    "node2.findag.com:30333"
]

[consensus]
round_interval_ms = 200
max_transactions_per_block = 10000
finality_threshold = 0.67

[api]
port = 8080
max_request_size = "1MB"
rate_limit_requests = 100
rate_limit_window_seconds = 60

[security]
jwt_expiry_hours = 24
max_login_attempts = 5
session_timeout_minutes = 30

[monitoring]
metrics_port = 9090
health_check_interval = 30
log_level = "info"
```

#### Security Configuration (`configs/security.toml`)

```toml
[authentication]
jwt_secret = "your_jwt_secret"
password_min_length = 8
require_special_chars = true
max_failed_attempts = 5

[encryption]
key_derivation_iterations = 100000
encryption_algorithm = "AES-256-GCM"

[network_security]
tls_enabled = true
cert_file = "/etc/ssl/certs/findag.crt"
key_file = "/etc/ssl/private/findag.key"
```

---

## 🔄 CI/CD Pipeline

### GitHub Actions Workflow

```yaml
# .github/workflows/ci-cd.yml
name: CI/CD Pipeline

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --all-features
      - name: Run security tests
        run: cargo run --bin security_test
      - name: Run performance tests
        run: cargo run --bin performance_test

  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Build Docker image
        run: docker build -t findag:${{ github.sha }} .
      - name: Push to registry
        run: |
          echo ${{ secrets.DOCKER_PASSWORD }} | docker login -u ${{ secrets.DOCKER_USERNAME }} --password-stdin
          docker push findag:${{ github.sha }}

  deploy-staging:
    needs: build
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/develop'
    steps:
      - uses: actions/checkout@v3
      - name: Deploy to staging
        run: |
          kubectl config use-context staging
          helm upgrade --install findag-staging ./helm \
            --namespace staging \
            --set image.tag=${{ github.sha }}

  deploy-production:
    needs: build
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    environment: production
    steps:
      - uses: actions/checkout@v3
      - name: Deploy to production
        run: |
          kubectl config use-context production
          helm upgrade --install findag ./helm \
            --namespace production \
            --set image.tag=${{ github.sha }}
```

### Local CI Testing

```bash
# Run CI tests locally
./scripts/run_ci_tests.ps1

# Or manually
cargo test --all-features
cargo run --bin security_test
cargo run --bin performance_test
cargo run --bin governance_test
cargo run --bin audit_test
cargo run --bin enterprise_features_test
```

---

## 🔒 Security Setup

### Generate Secure Credentials

```bash
# Generate admin password hash
echo -n "your_secure_password" | sha256sum | cut -d' ' -f1

# Generate JWT secret
openssl rand -hex 64

# Generate TLS certificates
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes
```

### Security Script

```powershell
# scripts/setup_security.ps1
param(
    [string]$AdminPassword,
    [string]$JwtSecret,
    [string]$OutputPath = "."
)

# Generate password hash
$passwordHash = (Get-FileHash -Algorithm SHA256 -InputObject ([System.Text.Encoding]::UTF8.GetBytes($AdminPassword))).Hash.ToLower()

# Generate JWT secret if not provided
if (-not $JwtSecret) {
    $JwtSecret = -join ((65..90) + (97..122) | Get-Random -Count 64 | ForEach-Object {[char]$_})
}

# Create environment file
@"
# FinDAG Security Configuration
ADMIN_USERNAME=admin
ADMIN_PASSWORD_HASH=$passwordHash
JWT_SECRET=$JwtSecret
NODE_ENV=production
"@ | Out-File -FilePath "$OutputPath\.env" -Encoding UTF8

Write-Host "Security configuration generated in $OutputPath\.env"
Write-Host "Admin Password Hash: $passwordHash"
Write-Host "JWT Secret: $JwtSecret"
```

---

## 📊 Monitoring Setup

### Prometheus Configuration

```yaml
# docker/prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files:
  - "alert_rules.yml"

alerting:
  alertmanagers:
    - static_configs:
        - targets:
          - alertmanager:9093

scrape_configs:
  - job_name: 'findag'
    static_configs:
      - targets: ['findag-node:9090']
    metrics_path: '/metrics'
    scrape_interval: 5s

  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']
```

### Grafana Dashboards

```json
// docker/grafana/dashboards/findag-overview.json
{
  "dashboard": {
    "title": "FinDAG Overview",
    "panels": [
      {
        "title": "Transaction Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(findag_transactions_total[5m])",
            "legendFormat": "TPS"
          }
        ]
      },
      {
        "title": "Block Production Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(findag_blocks_total[5m])",
            "legendFormat": "BPS"
          }
        ]
      }
    ]
  }
}
```

---

## 🚨 Troubleshooting

### Common Issues

#### 1. Database Connection Issues

```bash
# Check database file permissions
ls -la /data/findag.db

# Reset database (development only)
rm /data/findag.db
```

#### 2. Port Conflicts

```bash
# Check port usage
netstat -tulpn | grep :8080

# Change port in configuration
export API_PORT=8081
```

#### 3. Memory Issues

```bash
# Check memory usage
docker stats findag-node

# Increase memory limits
docker run --memory=4g findag:latest
```

#### 4. Network Issues

```bash
# Check P2P connectivity
curl -X GET http://localhost:8080/health

# Check peer connections
curl -X GET http://localhost:8080/validators
```

### Log Analysis

```bash
# View logs
docker logs findag-node

# Follow logs
docker logs -f findag-node

# Search for errors
docker logs findag-node | grep ERROR
```

### Performance Tuning

```bash
# Run performance tests
cargo run --bin performance_test

# Monitor system resources
htop
iotop
```

---

## 📚 Additional Resources

- [Architecture Overview](./ARCHITECTURE_OVERVIEW.md)
- [API Reference](./API_REFERENCE.md)
- [Configuration Guide](./CONFIGURATION_GUIDE.md)
- [Troubleshooting Guide](./TROUBLESHOOTING.md)
- [Security Best Practices](./SECURITY_BEST_PRACTICES.md)

---

*For support, contact the FinDAG team or create an issue in the GitHub repository.* 