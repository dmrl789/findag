# 🔧 FinDAG Troubleshooting Guide

## Overview

This guide provides systematic troubleshooting procedures for common FinDAG issues, including diagnostic tools, log analysis, and resolution steps.

---

## 🚨 Quick Diagnostic Checklist

### System Health Check

```bash
# 1. Check if container is running
docker ps | grep findag-node

# 2. Check system health endpoint
curl -f http://localhost:8080/health

# 3. Check resource usage
docker stats --no-stream findag-node

# 4. Check recent logs
docker logs --tail 50 findag-node

# 5. Check network connectivity
curl -I https://api.findag.com
```

### Emergency Commands

```bash
# Restart container
docker restart findag-node

# Check container logs
docker logs -f findag-node

# Access container shell
docker exec -it findag-node /bin/bash

# Check system resources
htop && df -h && free -h
```

---

## 🔍 Diagnostic Tools

### Log Analysis

#### Common Log Patterns

```bash
# Find errors
docker logs findag-node | grep -i error

# Find warnings
docker logs findag-node | grep -i warn

# Find authentication failures
docker logs findag-node | grep -i "auth\|login"

# Find rate limiting
docker logs findag-node | grep -i "rate.*limit"

# Find database issues
docker logs findag-node | grep -i "db\|database\|sled"

# Find network issues
docker logs findag-node | grep -i "network\|p2p\|peer"
```

#### Log Timestamps

```bash
# Check logs from last hour
docker logs --since 1h findag-node

# Check logs from specific time
docker logs --since "2025-01-01T10:00:00" findag-node

# Check logs between times
docker logs --since "2025-01-01T10:00:00" --until "2025-01-01T11:00:00" findag-node
```

### Metrics Analysis

#### System Metrics

```bash
# Get all metrics
curl -s http://localhost:9090/metrics

# Check specific metrics
curl -s http://localhost:9090/metrics | grep -E "(cpu|memory|disk)"

# Check FinDAG-specific metrics
curl -s http://localhost:9090/metrics | grep findag_
```

#### Performance Metrics

```bash
# Transaction rate
curl -s http://localhost:9090/metrics | grep findag_transactions_total

# Block production rate
curl -s http://localhost:9090/metrics | grep findag_blocks_total

# Validator count
curl -s http://localhost:9090/metrics | grep findag_validators

# Network peers
curl -s http://localhost:9090/metrics | grep findag_peers
```

### API Testing

#### Health Endpoints

```bash
# Basic health check
curl -v http://localhost:8080/health

# Detailed health check
curl -v http://localhost:8080/health/detailed

# Metrics endpoint
curl -v http://localhost:9090/metrics
```

#### Functional Endpoints

```bash
# Test validators endpoint
curl -v http://localhost:8080/validators

# Test governance stats
curl -v http://localhost:8080/governance/stats

# Test assets endpoint
curl -v http://localhost:8080/assets
```

---

## 🐛 Common Issues and Solutions

### 1. Container Won't Start

#### Symptoms
- Container exits immediately
- `docker ps` shows no running container
- `docker logs` shows startup errors

#### Diagnosis
```bash
# Check container logs
docker logs findag-node

# Check container status
docker ps -a | grep findag-node

# Check resource availability
df -h
free -h
```

#### Common Causes and Solutions

**Port Conflict:**
```bash
# Check port usage
netstat -tulpn | grep :8080
lsof -i :8080

# Solution: Change port or stop conflicting service
docker run -p 8081:8080 findag:latest
```

**Permission Issues:**
```bash
# Check file permissions
ls -la /data/findag.db
ls -la /data/

# Solution: Fix permissions
chown 1000:1000 /data/findag.db
chmod 644 /data/findag.db
```

**Missing Environment Variables:**
```bash
# Check environment
docker inspect findag-node | grep -A 10 "Env"

# Solution: Set required variables
docker run -e ADMIN_PASSWORD_HASH=xxx -e JWT_SECRET=xxx findag:latest
```

### 2. API Endpoints Not Responding

#### Symptoms
- `curl` returns connection refused
- Health endpoint returns 500 error
- API calls timeout

#### Diagnosis
```bash
# Check if service is listening
netstat -tulpn | grep :8080

# Test local connectivity
curl -v http://localhost:8080/health

# Check container logs
docker logs --tail 100 findag-node
```

#### Solutions

**Service Not Started:**
```bash
# Restart container
docker restart findag-node

# Check startup logs
docker logs findag-node | head -50
```

**Database Connection Issues:**
```bash
# Check database file
ls -la /data/findag.db

# Verify database integrity
curl -X POST http://localhost:8080/admin/db/verify \
  -H "Authorization: Bearer $ADMIN_TOKEN"
```

**Memory Issues:**
```bash
# Check memory usage
docker stats findag-node

# Increase memory limit
docker run --memory=4g findag:latest
```

### 3. High Memory Usage

#### Symptoms
- Container using >80% of allocated memory
- System becomes slow
- Out of memory errors in logs

#### Diagnosis
```bash
# Check memory usage
docker stats findag-node
free -h

# Check memory metrics
curl -s http://localhost:9090/metrics | grep memory

# Check for memory leaks
docker logs findag-node | grep -i "memory\|oom"
```

#### Solutions

**Increase Memory Limit:**
```bash
# Stop container
docker stop findag-node

# Start with more memory
docker run --memory=4g --memory-swap=4g findag-node
```

**Optimize Database:**
```bash
# Run database optimization
curl -X POST http://localhost:8080/admin/db/optimize \
  -H "Authorization: Bearer $ADMIN_TOKEN"
```

**Restart Container:**
```bash
# Restart to clear memory
docker restart findag-node
```

### 4. Database Issues

#### Symptoms
- Database corruption errors
- Slow query performance
- Storage space issues

#### Diagnosis
```bash
# Check database size
du -sh /data/findag.db

# Check disk space
df -h /data

# Check database integrity
curl -X POST http://localhost:8080/admin/db/verify \
  -H "Authorization: Bearer $ADMIN_TOKEN"
```

#### Solutions

**Database Corruption:**
```bash
# Stop service
docker stop findag-node

# Restore from backup
cp /backup/findag-$(date +%Y%m%d).db /data/findag.db

# Verify backup integrity
sha256sum /backup/findag-$(date +%Y%m%d).db

# Restart service
docker start findag-node
```

**Storage Space:**
```bash
# Clean old backups
find /backup -name "findag-*.db" -mtime +30 -delete

# Compress old logs
find /var/log -name "*.log" -mtime +7 -exec gzip {} \;

# Expand storage if needed
# Contact infrastructure team
```

### 5. Network Connectivity Issues

#### Symptoms
- P2P peers not connecting
- API calls failing
- Bridge operations failing

#### Diagnosis
```bash
# Check network connectivity
ping -c 3 8.8.8.8

# Check DNS resolution
nslookup api.findag.com

# Check firewall rules
iptables -L

# Check P2P port
netstat -tulpn | grep :30333
```

#### Solutions

**Firewall Issues:**
```bash
# Allow required ports
iptables -A INPUT -p tcp --dport 8080 -j ACCEPT
iptables -A INPUT -p tcp --dport 30333 -j ACCEPT
iptables -A INPUT -p tcp --dport 9090 -j ACCEPT
```

**DNS Issues:**
```bash
# Check DNS configuration
cat /etc/resolv.conf

# Use alternative DNS
echo "nameserver 8.8.8.8" >> /etc/resolv.conf
echo "nameserver 8.8.4.4" >> /etc/resolv.conf
```

### 6. Authentication Issues

#### Symptoms
- Login failures
- JWT token errors
- Authorization failures

#### Diagnosis
```bash
# Check authentication logs
docker logs findag-node | grep -i "auth\|login\|jwt"

# Test login endpoint
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "test"}'

# Check JWT configuration
echo $JWT_SECRET
```

#### Solutions

**Invalid Credentials:**
```bash
# Reset admin password
echo -n "new_password" | sha256sum | cut -d' ' -f1

# Update environment variable
export ADMIN_PASSWORD_HASH="new_hash"
docker restart findag-node
```

**JWT Issues:**
```bash
# Generate new JWT secret
openssl rand -hex 64

# Update JWT secret
export JWT_SECRET="new_secret"
docker restart findag-node
```

### 7. Performance Issues

#### Symptoms
- Slow API responses
- Low transaction throughput
- High latency

#### Diagnosis
```bash
# Check response times
curl -w "@curl-format.txt" -o /dev/null -s http://localhost:8080/health

# Check transaction rate
curl -s http://localhost:9090/metrics | grep findag_transactions_total

# Check CPU usage
docker stats findag-node
```

#### Solutions

**Resource Constraints:**
```bash
# Increase CPU/memory limits
docker run --cpus=2 --memory=4g findag:latest

# Scale horizontally
# Deploy additional nodes
```

**Database Performance:**
```bash
# Optimize database
curl -X POST http://localhost:8080/admin/db/optimize \
  -H "Authorization: Bearer $ADMIN_TOKEN"

# Check for slow queries
docker logs findag-node | grep -i "slow\|query"
```

---

## 🔧 Advanced Troubleshooting

### Debug Mode

#### Enable Debug Logging

```bash
# Set debug log level
export LOG_LEVEL=debug
docker restart findag-node

# Check debug logs
docker logs -f findag-node | grep DEBUG
```

#### Profiling

```bash
# Enable profiling
curl -X POST http://localhost:8080/admin/debug/profile/start \
  -H "Authorization: Bearer $ADMIN_TOKEN"

# Collect profile data
curl -X GET http://localhost:8080/admin/debug/profile \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -o profile.json

# Stop profiling
curl -X POST http://localhost:8080/admin/debug/profile/stop \
  -H "Authorization: Bearer $ADMIN_TOKEN"
```

### Database Recovery

#### Manual Database Repair

```bash
# Stop service
docker stop findag-node

# Create database backup
cp /data/findag.db /backup/findag-repair-$(date +%Y%m%d-%H%M%S).db

# Run database repair
docker run --rm -v /data:/data findag:latest \
  /bin/bash -c "findag-db-repair /data/findag.db"

# Restart service
docker start findag-node
```

#### Data Export/Import

```bash
# Export data
curl -X GET http://localhost:8080/admin/data/export \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -o data-export-$(date +%Y%m%d).json

# Import data
curl -X POST http://localhost:8080/admin/data/import \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -F "file=@data-export-20250101.json"
```

---

## 📊 Monitoring and Alerting

### Custom Alerts

#### Create Alert Rules

```yaml
# alert-rules.yml
groups:
  - name: findag_alerts
    rules:
      - alert: HighMemoryUsage
        expr: process_resident_memory_bytes > 3e9
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High memory usage detected"
          description: "Memory usage is above 3GB for 5 minutes"

      - alert: LowTransactionRate
        expr: rate(findag_transactions_total[5m]) < 100
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "Low transaction rate"
          description: "Transaction rate is below 100 TPS"
```

#### Alert Response

```bash
# Check alert status
curl -s http://localhost:9090/api/v1/alerts

# Acknowledge alert
curl -X POST http://localhost:9090/api/v1/alerts \
  -H "Content-Type: application/json" \
  -d '{"status": "acknowledged"}'
```

---

## 📞 Support and Escalation

### Contact Information

```
Primary Support: support@findag.com
Emergency: +1-555-9999
Documentation: https://docs.findag.com
GitHub Issues: https://github.com/findag/findag/issues
```

### Escalation Procedure

1. **Level 1**: On-call engineer (15 min response)
2. **Level 2**: Senior engineer (30 min response)
3. **Level 3**: System architect (1 hour response)
4. **Level 4**: CTO/VP Engineering (2 hour response)

### Information to Collect

When reporting issues, collect:

```bash
# System information
docker version
docker info
uname -a

# Application logs
docker logs findag-node > findag-logs.txt

# Configuration
docker inspect findag-node > container-inspect.txt

# Metrics snapshot
curl -s http://localhost:9090/metrics > metrics.txt

# Health status
curl -s http://localhost:8080/health > health.txt
```

---

*This guide should be updated regularly based on new issues and solutions. Last updated: January 2025* 