# FinDAG Helm Chart

A Helm chart for deploying FinDAG - High-frequency financial DAG consensus system on Kubernetes.

## Prerequisites

- Kubernetes 1.19+
- Helm 3.0+
- kubectl configured to communicate with your cluster

## Installation

### Add the Helm repository (if using a repository)
```bash
helm repo add findag https://your-helm-repo.com
helm repo update
```

### Install the chart
```bash
# Install with default values
helm install findag ./helm

# Install with custom values
helm install findag ./helm -f values-production.yaml

# Install in a specific namespace
helm install findag ./helm --namespace findag --create-namespace
```

## Configuration

The following table lists the configurable parameters of the FinDAG chart and their default values.

| Parameter | Description | Default |
|-----------|-------------|---------|
| `replicaCount` | Number of FinDAG replicas | `3` |
| `image.repository` | Docker image repository | `ghcr.io/your-org/findag` |
| `image.tag` | Docker image tag | `latest` |
| `image.pullPolicy` | Docker image pull policy | `IfNotPresent` |
| `service.type` | Kubernetes service type | `ClusterIP` |
| `service.port` | HTTP service port | `3000` |
| `service.apiPort` | API service port | `8080` |
| `service.metricsPort` | Metrics service port | `9898` |
| `resources.limits.cpu` | CPU resource limits | `2000m` |
| `resources.limits.memory` | Memory resource limits | `4Gi` |
| `resources.requests.cpu` | CPU resource requests | `500m` |
| `resources.requests.memory` | Memory resource requests | `1Gi` |
| `autoscaling.enabled` | Enable horizontal pod autoscaling | `true` |
| `autoscaling.minReplicas` | Minimum number of replicas | `3` |
| `autoscaling.maxReplicas` | Maximum number of replicas | `10` |
| `persistence.enabled` | Enable persistent storage | `true` |
| `persistence.size` | Storage size | `10Gi` |
| `postgresql.enabled` | Enable PostgreSQL database | `true` |
| `redis.enabled` | Enable Redis cache | `true` |
| `monitoring.enabled` | Enable monitoring | `true` |

### FinDAG Specific Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `config.network.port` | FinDAG network port | `3000` |
| `config.network.apiPort` | FinDAG API port | `8080` |
| `config.network.metricsPort` | FinDAG metrics port | `9898` |
| `config.network.p2pPort` | FinDAG P2P port | `9000` |
| `config.consensus.roundInterval` | Round interval in milliseconds | `100` |
| `config.consensus.quorumSize` | Consensus quorum size | `2` |
| `config.storage.dataDir` | Data directory | `/app/data` |
| `config.security.jwtSecret` | JWT secret for authentication | `""` |

## Usage Examples

### Production Deployment
```bash
# Create production values file
cat > values-production.yaml << EOF
replicaCount: 5
autoscaling:
  enabled: true
  minReplicas: 5
  maxReplicas: 20
resources:
  limits:
    cpu: 4000m
    memory: 8Gi
  requests:
    cpu: 1000m
    memory: 2Gi
config:
  consensus:
    roundInterval: 50
    quorumSize: 3
persistence:
  size: 50Gi
postgresql:
  enabled: true
  auth:
    postgresPassword: "your-secure-password"
redis:
  enabled: true
monitoring:
  enabled: true
EOF

# Install with production values
helm install findag-prod ./helm -f values-production.yaml --namespace production
```

### Development Deployment
```bash
# Create development values file
cat > values-dev.yaml << EOF
replicaCount: 1
autoscaling:
  enabled: false
resources:
  limits:
    cpu: 1000m
    memory: 2Gi
  requests:
    cpu: 250m
    memory: 512Mi
config:
  consensus:
    roundInterval: 200
    quorumSize: 1
persistence:
  size: 5Gi
postgresql:
  enabled: false
redis:
  enabled: false
monitoring:
  enabled: false
EOF

# Install with development values
helm install findag-dev ./helm -f values-dev.yaml --namespace development
```

## Upgrading

```bash
# Upgrade with new values
helm upgrade findag ./helm -f values-production.yaml

# Upgrade with new chart version
helm upgrade findag ./helm --version 0.2.0
```

## Uninstalling

```bash
# Uninstall the release
helm uninstall findag

# Uninstall and delete persistent data
helm uninstall findag
kubectl delete pvc -l app.kubernetes.io/instance=findag
```

## Monitoring

### Prometheus Metrics
The FinDAG application exposes Prometheus metrics on port 9898. You can configure Prometheus to scrape these metrics:

```yaml
# prometheus-config.yaml
scrape_configs:
  - job_name: 'findag'
    static_configs:
      - targets: ['findag-service:9898']
```

### Grafana Dashboards
Import the provided Grafana dashboard JSON files for monitoring:
- `dashboards/findag-overview.json`
- `dashboards/findag-consensus.json`
- `dashboards/findag-network.json`

## Troubleshooting

### Check Pod Status
```bash
kubectl get pods -l app.kubernetes.io/name=findag
kubectl describe pod <pod-name>
```

### Check Logs
```bash
kubectl logs -l app.kubernetes.io/name=findag
kubectl logs -f deployment/findag
```

### Check Services
```bash
kubectl get svc -l app.kubernetes.io/name=findag
kubectl describe svc findag
```

### Check Persistent Volumes
```bash
kubectl get pvc -l app.kubernetes.io/name=findag
kubectl describe pvc findag-data
```

## Security

### Network Policies
The chart includes network policies that restrict communication to only necessary ports and services.

### Secrets Management
Store sensitive data like JWT secrets in Kubernetes secrets:

```bash
kubectl create secret generic findag-secrets \
  --from-literal=jwt-secret="your-jwt-secret" \
  --from-literal=api-key="your-api-key"
```

### RBAC
The chart creates a service account with minimal required permissions. For production, consider using more restrictive RBAC policies.

## Support

For issues and questions:
- GitHub Issues: [FinDAG Repository](https://github.com/your-org/findag)
- Documentation: [FinDAG Docs](https://docs.findag.io)
- Community: [Discord/Slack](https://community.findag.io) 