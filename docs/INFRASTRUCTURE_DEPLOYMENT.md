# 🏗️ FinDAG Infrastructure Deployment Guide

## Overview

This document provides comprehensive instructions for deploying FinDAG in production environments using Docker and Kubernetes.

---

## 📋 **Prerequisites**

### **System Requirements**
- **CPU**: 4+ cores (8+ recommended for production)
- **RAM**: 8GB+ (16GB+ recommended for production)
- **Storage**: 100GB+ SSD (500GB+ recommended for production)
- **Network**: 1Gbps+ connectivity

### **Software Requirements**
- **Docker**: 20.10+ with Docker Compose
- **Kubernetes**: 1.24+ (for K8s deployment)
- **kubectl**: Latest version
- **Helm**: 3.8+ (optional, for advanced deployments)

---

## 🐳 **Docker Deployment**

### **Quick Start**

1. **Build the Docker image:**
   ```bash
   docker build -t findag:latest -f docker/Dockerfile .
   ```

2. **Deploy with Docker Compose:**
   ```bash
   cd docker
   docker-compose up -d
   ```

3. **Verify deployment:**
   ```bash
   docker-compose ps
   docker-compose logs findag
   ```

### **Service URLs**
- **FinDAG API**: http://localhost:8080
- **FinDAG Metrics**: http://localhost:9090
- **Prometheus**: http://localhost:9091
- **Grafana**: http://localhost:3000 (admin/admin123)
- **Redis**: localhost:6379

### **Using Deployment Scripts**
```powershell
# Build and deploy
.\scripts\deploy_docker.ps1 -Build -Deploy

# Check status
.\scripts\deploy_docker.ps1 -Status

# View logs
.\scripts\deploy_docker.ps1 -Logs

# Stop services
.\scripts\deploy_docker.ps1 -Stop

# Clean up
.\scripts\deploy_docker.ps1 -Clean
```

---

## ☸️ **Kubernetes Deployment**

### **Quick Start**

1. **Deploy to Kubernetes:**
   ```bash
   kubectl apply -f k8s/
   ```

2. **Check deployment status:**
   ```bash
   kubectl get pods -n findag
   kubectl get svc -n findag
   ```

3. **Access the application:**
   ```bash
   kubectl port-forward svc/findag-service 8080:80 -n findag
   ```

### **Using Deployment Scripts**
```powershell
# Deploy to Kubernetes
.\scripts\deploy_k8s.ps1 -Deploy

# Check status
.\scripts\deploy_k8s.ps1 -Status

# View logs
.\scripts\deploy_k8s.ps1 -Logs

# Delete deployment
.\scripts\deploy_k8s.ps1 -Delete
```

---

## 🔧 **Configuration**

### **Environment Variables**

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `ADMIN_USERNAME` | Admin username | `admin` | Yes |
| `ADMIN_PASSWORD_HASH` | SHA-256 hash of admin password | - | Yes |
| `JWT_SECRET` | JWT signing secret | - | Yes |
| `AUDIT_LOG_PATH` | Audit log file path | `/app/logs/audit.log` | No |
| `RATE_LIMIT_REQUESTS_PER_MINUTE` | API rate limit per minute | `100` | No |
| `RATE_LIMIT_REQUESTS_PER_HOUR` | API rate limit per hour | `1000` | No |
| `CORS_ALLOWED_ORIGINS` | Allowed CORS origins | `*` | No |

### **Configuration Files**

#### **database.toml**
```toml
[database]
path = "/app/data/findag.db"
max_open_files = 1000
cache_size = 1024
compression = true
```

#### **production.toml**
```toml
[network]
port = 8080
metrics_port = 9090
max_connections = 1000

[consensus]
round_interval_ms = 200
validator_timeout_ms = 5000
max_validators = 100

[security]
enable_auth = true
rate_limit_requests_per_minute = 100
rate_limit_requests_per_hour = 1000
max_request_size_bytes = 1048576
```

---

## 📊 **Monitoring & Observability**

### **Prometheus Configuration**

The Prometheus configuration (`docker/prometheus.yml`) includes:
- **FinDAG metrics**: Scraped every 10 seconds
- **System metrics**: Via node-exporter
- **Custom labels**: For better organization

### **Grafana Dashboards**

Pre-configured dashboards include:
- **Transaction Rate**: TPS monitoring
- **Block Production**: BPS monitoring
- **System Resources**: CPU, memory, disk usage
- **API Performance**: Response times, error rates
- **Network Metrics**: Connection counts, throughput

### **Key Metrics**

| Metric | Description | Type |
|--------|-------------|------|
| `findag_transactions_total` | Total transactions processed | Counter |
| `findag_blocks_produced_total` | Total blocks produced | Counter |
| `findag_active_validators` | Number of active validators | Gauge |
| `findag_round_duration_seconds` | Round duration in seconds | Histogram |
| `findag_http_request_duration_seconds` | HTTP request duration | Histogram |

---

## 🔒 **Security Configuration**

### **Production Security Checklist**

- [ ] **Change default admin password**
- [ ] **Use strong, unique JWT secret**
- [ ] **Configure proper TLS certificates**
- [ ] **Restrict CORS origins to specific domains**
- [ ] **Set up firewall rules**
- [ ] **Enable security monitoring and alerting**
- [ ] **Regular security audits**

### **TLS Configuration**

For production, configure TLS certificates:

```bash
# Generate certificates
openssl req -x509 -newkey rsa:4096 -keyout server.key -out server.crt -days 365 -nodes

# Set environment variables
export TLS_CERT_PATH=/path/to/server.crt
export TLS_KEY_PATH=/path/to/server.key
```

---

## 📈 **Scaling & Performance**

### **Horizontal Pod Autoscaler (Kubernetes)**

The HPA configuration automatically scales based on:
- **CPU utilization**: 70% threshold
- **Memory utilization**: 80% threshold
- **Min replicas**: 3
- **Max replicas**: 10

### **Resource Limits**

| Resource | Requests | Limits |
|----------|----------|--------|
| **CPU** | 250m | 1000m |
| **Memory** | 512Mi | 2Gi |
| **Storage** | 10Gi | - |

### **Performance Tuning**

1. **Database Optimization:**
   ```toml
   [database]
   cache_size = 2048  # Increase for more RAM
   max_open_files = 2000
   ```

2. **Network Tuning:**
   ```toml
   [network]
   max_connections = 2000
   ```

3. **Consensus Tuning:**
   ```toml
   [consensus]
   round_interval_ms = 150  # Faster rounds
   validator_timeout_ms = 3000
   ```

---

## 🚨 **Troubleshooting**

### **Common Issues**

#### **Docker Issues**
```bash
# Check Docker status
docker info
docker system df

# Clean up Docker
docker system prune -a
docker volume prune
```

#### **Kubernetes Issues**
```bash
# Check pod status
kubectl describe pod <pod-name> -n findag

# Check logs
kubectl logs <pod-name> -n findag

# Check events
kubectl get events -n findag --sort-by='.lastTimestamp'
```

#### **Application Issues**
```bash
# Check health endpoint
curl http://localhost:8080/health

# Check metrics
curl http://localhost:9090/metrics

# Check logs
docker-compose logs findag
```

### **Performance Issues**

1. **High CPU Usage:**
   - Increase CPU limits
   - Optimize database queries
   - Reduce round frequency

2. **High Memory Usage:**
   - Increase memory limits
   - Reduce cache size
   - Check for memory leaks

3. **Slow Response Times:**
   - Check network latency
   - Optimize database
   - Increase resources

---

## 🔄 **Backup & Recovery**

### **Data Backup**

```bash
# Backup database
docker exec findag-node cp /app/data/findag.db /app/data/findag.db.backup

# Backup configuration
docker exec findag-node tar -czf /app/configs/configs.tar.gz /app/configs/

# Backup logs
docker exec findag-node tar -czf /app/logs/logs.tar.gz /app/logs/
```

### **Disaster Recovery**

1. **Restore from backup:**
   ```bash
   docker exec findag-node cp /app/data/findag.db.backup /app/data/findag.db
   ```

2. **Restart services:**
   ```bash
   docker-compose restart findag
   ```

---

## 📚 **Additional Resources**

### **Documentation**
- [FinDAG API Reference](api_reference.md)
- [Security Hardening Guide](security_guide.md)
- [Performance Tuning Guide](performance_guide.md)

### **Monitoring**
- [Prometheus Documentation](https://prometheus.io/docs/)
- [Grafana Documentation](https://grafana.com/docs/)

### **Kubernetes**
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [Helm Charts](https://helm.sh/docs/)

---

## 🎯 **Next Steps**

1. **Deploy to staging environment**
2. **Run performance tests**
3. **Configure monitoring alerts**
4. **Set up CI/CD pipeline**
5. **Deploy to production**
6. **Monitor and optimize**

---

*Last Updated: January 2025*
*Version: 1.0* 