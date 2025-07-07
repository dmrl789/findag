# 📋 FinDAG Operational Runbooks

## Overview

This document provides operational procedures for running FinDAG in production, including routine maintenance, monitoring, troubleshooting, and emergency response procedures.

---

## 🔄 Routine Operations

### Daily Operations

#### 1. Health Check

**Procedure:**
```bash
# Check system health
curl -f http://localhost:8080/health

# Check metrics endpoint
curl http://localhost:9090/metrics

# Verify database connectivity
curl http://localhost:8080/validators
```

**Expected Results:**
- Health endpoint returns `{"status": "healthy"}`
- Metrics endpoint returns Prometheus metrics
- Validators endpoint returns current validator list

**Actions if Failed:**
- Check application logs: `docker logs findag-node`
- Verify container status: `docker ps`
- Check resource usage: `docker stats findag-node`

#### 2. Performance Monitoring

**Procedure:**
```bash
# Check transaction rate
curl -s http://localhost:9090/metrics | grep findag_transactions_total

# Check block production rate
curl -s http://localhost:9090/metrics | grep findag_blocks_total

# Check memory usage
curl -s http://localhost:9090/metrics | grep process_resident_memory_bytes
```

**Thresholds:**
- Transaction rate: > 1000 TPS
- Block production: > 5 BPS
- Memory usage: < 80% of allocated

#### 3. Log Review

**Procedure:**
```bash
# Check for errors in last hour
docker logs --since 1h findag-node | grep -i error

# Check for warnings
docker logs --since 1h findag-node | grep -i warn

# Check audit logs
docker logs --since 1h findag-node | grep "AUDIT"
```

**Actions:**
- Document any errors or warnings
- Escalate critical issues
- Update monitoring alerts if needed

---

### Weekly Operations

#### 1. Database Maintenance

**Procedure:**
```bash
# Check database size
du -sh /data/findag.db

# Run database optimization
curl -X POST http://localhost:8080/admin/db/optimize \
  -H "Authorization: Bearer $ADMIN_TOKEN"

# Backup database
cp /data/findag.db /backup/findag-$(date +%Y%m%d).db
```

**Expected Results:**
- Database size < 10GB
- Optimization completes successfully
- Backup file created

#### 2. Security Review

**Procedure:**
```bash
# Check for failed login attempts
docker logs --since 7d findag-node | grep "LOGIN_FAILED"

# Review API access logs
docker logs --since 7d findag-node | grep "API_ACCESS"

# Check rate limit violations
docker logs --since 7d findag-node | grep "RATE_LIMIT"
```

**Actions:**
- Investigate suspicious activity
- Update security policies if needed
- Review and rotate credentials

#### 3. Performance Analysis

**Procedure:**
```bash
# Generate weekly performance report
curl -X POST http://localhost:8080/admin/reports/performance \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -d '{"timeframe": "7d"}'

# Check governance activity
curl http://localhost:8080/governance/stats

# Review validator performance
curl http://localhost:8080/validators
```

---

### Monthly Operations

#### 1. System Updates

**Procedure:**
```bash
# Check for available updates
docker pull findag/findag:latest

# Review changelog
curl https://api.github.com/repos/findag/findag/releases/latest

# Plan maintenance window
# Schedule downtime if needed
```

#### 2. Capacity Planning

**Procedure:**
```bash
# Analyze storage growth
du -sh /data/findag.db.* | sort -h

# Check resource utilization trends
curl -s http://localhost:9090/metrics | grep -E "(cpu|memory|disk)"

# Review performance metrics
curl http://localhost:8080/admin/metrics/summary
```

#### 3. Compliance Review

**Procedure:**
```bash
# Generate compliance report
curl -X POST http://localhost:8080/admin/reports/compliance \
  -H "Authorization: Bearer $ADMIN_TOKEN"

# Export audit logs
curl -X GET http://localhost:8080/admin/audit/export \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -o audit-logs-$(date +%Y%m).json

# Review governance compliance
curl http://localhost:8080/governance/analytics
```

---

## 🔧 Maintenance Procedures

### Database Maintenance

#### Backup Procedure

**Pre-backup:**
```bash
# Notify stakeholders
echo "Starting database backup at $(date)"

# Check available space
df -h /backup
```

**Backup:**
```bash
# Create backup
cp /data/findag.db /backup/findag-$(date +%Y%m%d-%H%M%S).db

# Verify backup integrity
sha256sum /data/findag.db /backup/findag-$(date +%Y%m%d-%H%M%S).db

# Compress backup
gzip /backup/findag-$(date +%Y%m%d-%H%M%S).db
```

**Post-backup:**
```bash
# Update backup inventory
echo "$(date): Backup completed" >> /var/log/findag-backups.log

# Clean old backups (keep 30 days)
find /backup -name "findag-*.db.gz" -mtime +30 -delete
```

#### Database Optimization

**Procedure:**
```bash
# Stop accepting new transactions
curl -X POST http://localhost:8080/admin/maintenance/start \
  -H "Authorization: Bearer $ADMIN_TOKEN"

# Run optimization
curl -X POST http://localhost:8080/admin/db/optimize \
  -H "Authorization: Bearer $ADMIN_TOKEN"

# Resume normal operations
curl -X POST http://localhost:8080/admin/maintenance/end \
  -H "Authorization: Bearer $ADMIN_TOKEN"
```

### Software Updates

#### Rolling Update Procedure

**Pre-update:**
```bash
# Verify current version
curl http://localhost:8080/health | jq .version

# Check compatibility
curl https://api.findag.com/compatibility/check

# Backup current state
cp /data/findag.db /backup/pre-update-$(date +%Y%m%d-%H%M%S).db
```

**Update:**
```bash
# Pull new image
docker pull findag/findag:latest

# Stop current container
docker stop findag-node

# Start new container
docker run -d \
  --name findag-node-new \
  -p 8080:8080 \
  -p 9090:9090 \
  -v findag-data:/data \
  -e ADMIN_USERNAME=$ADMIN_USERNAME \
  -e ADMIN_PASSWORD_HASH=$ADMIN_PASSWORD_HASH \
  -e JWT_SECRET=$JWT_SECRET \
  findag/findag:latest

# Verify new container
sleep 30
curl -f http://localhost:8080/health

# Remove old container
docker rm findag-node
docker rename findag-node-new findag-node
```

**Post-update:**
```bash
# Verify functionality
curl http://localhost:8080/validators
curl http://localhost:8080/governance/stats

# Monitor for issues
docker logs -f findag-node
```

---

## 🚨 Emergency Procedures

### System Outage Response

#### 1. Immediate Response (0-5 minutes)

**Procedure:**
```bash
# Check system status
curl -f http://localhost:8080/health || echo "System down"

# Check container status
docker ps | grep findag-node

# Check resource usage
docker stats --no-stream findag-node

# Notify stakeholders
echo "ALERT: FinDAG system outage detected at $(date)" | \
  mail -s "FinDAG Emergency Alert" admin@findag.com
```

**Actions:**
- Document outage start time
- Assess impact scope
- Begin incident response

#### 2. Investigation (5-15 minutes)

**Procedure:**
```bash
# Check recent logs
docker logs --since 10m findag-node

# Check system resources
htop
df -h
free -h

# Check network connectivity
ping -c 3 8.8.8.8
curl -I https://api.findag.com
```

**Actions:**
- Identify root cause
- Determine recovery strategy
- Update stakeholders

#### 3. Recovery (15-30 minutes)

**Procedure:**
```bash
# Restart container if needed
docker restart findag-node

# Verify recovery
sleep 30
curl -f http://localhost:8080/health

# Check data integrity
curl http://localhost:8080/validators
```

**Actions:**
- Execute recovery plan
- Verify system functionality
- Document recovery steps

### Data Corruption Response

#### 1. Detection

**Procedure:**
```bash
# Check database integrity
curl -X POST http://localhost:8080/admin/db/verify \
  -H "Authorization: Bearer $ADMIN_TOKEN"

# Check for corruption indicators
docker logs findag-node | grep -i "corrupt\|error\|invalid"
```

#### 2. Recovery

**Procedure:**
```bash
# Stop system
docker stop findag-node

# Restore from backup
cp /backup/findag-$(date +%Y%m%d).db /data/findag.db

# Verify backup integrity
sha256sum /backup/findag-$(date +%Y%m%d).db

# Restart system
docker start findag-node

# Verify recovery
sleep 30
curl -f http://localhost:8080/health
```

### Security Incident Response

#### 1. Detection

**Procedure:**
```bash
# Check for unauthorized access
docker logs --since 1h findag-node | grep "UNAUTHORIZED"

# Check for suspicious activity
docker logs --since 1h findag-node | grep "SUSPICIOUS"

# Review recent API calls
curl -X GET http://localhost:8080/admin/audit/recent \
  -H "Authorization: Bearer $ADMIN_TOKEN"
```

#### 2. Response

**Procedure:**
```bash
# Block suspicious IPs
iptables -A INPUT -s <suspicious_ip> -j DROP

# Revoke compromised tokens
curl -X POST http://localhost:8080/admin/security/revoke-tokens \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -d '{"user_id": "compromised_user"}'

# Update security policies
curl -X POST http://localhost:8080/admin/security/update-policies \
  -H "Authorization: Bearer $ADMIN_TOKEN"
```

---

## 📊 Monitoring and Alerting

### Key Metrics

#### System Health Metrics

```bash
# Uptime
curl -s http://localhost:9090/metrics | grep process_uptime_seconds

# Memory usage
curl -s http://localhost:9090/metrics | grep process_resident_memory_bytes

# CPU usage
curl -s http://localhost:9090/metrics | grep process_cpu_seconds_total
```

#### Business Metrics

```bash
# Transaction rate
curl -s http://localhost:9090/metrics | grep findag_transactions_total

# Block production rate
curl -s http://localhost:9090/metrics | grep findag_blocks_total

# Active validators
curl -s http://localhost:9090/metrics | grep findag_validators_active
```

### Alert Thresholds

| Metric | Warning | Critical | Action |
|--------|---------|----------|--------|
| Uptime | < 99.9% | < 99% | Check system health |
| Memory | > 80% | > 90% | Restart container |
| CPU | > 70% | > 85% | Scale resources |
| TPS | < 500 | < 100 | Check network |
| BPS | < 3 | < 1 | Check consensus |

### Alert Response

**Procedure:**
```bash
# Acknowledge alert
echo "Alert acknowledged at $(date)" >> /var/log/findag-alerts.log

# Investigate cause
curl -f http://localhost:8080/health
docker logs --since 5m findag-node

# Take corrective action
# Document response
echo "Action taken: $ACTION at $(date)" >> /var/log/findag-alerts.log
```

---

## 📝 Documentation and Reporting

### Incident Reports

**Template:**
```
INCIDENT REPORT
===============

Date: [Date]
Time: [Start Time - End Time]
Severity: [Low/Medium/High/Critical]
Affected Systems: [List systems]

DESCRIPTION
-----------
[Detailed description of the incident]

ROOT CAUSE
----------
[Analysis of root cause]

ACTIONS TAKEN
-------------
[Step-by-step actions taken]

RESOLUTION
----------
[How the incident was resolved]

LESSONS LEARNED
--------------
[What was learned and improvements needed]

PREVENTION
----------
[Measures to prevent recurrence]
```

### Monthly Reports

**Procedure:**
```bash
# Generate system report
curl -X POST http://localhost:8080/admin/reports/system \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -o system-report-$(date +%Y%m).json

# Generate performance report
curl -X POST http://localhost:8080/admin/reports/performance \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -o performance-report-$(date +%Y%m).json

# Generate security report
curl -X POST http://localhost:8080/admin/reports/security \
  -H "Authorization: Bearer $ADMIN_TOKEN" \
  -o security-report-$(date +%Y%m).json
```

---

## 🔗 Escalation Procedures

### Escalation Matrix

| Level | Response Time | Contact | Actions |
|-------|---------------|---------|---------|
| L1 | 15 minutes | On-call engineer | Initial investigation |
| L2 | 30 minutes | Senior engineer | Technical resolution |
| L3 | 1 hour | System architect | Architecture review |
| L4 | 2 hours | CTO/VP Engineering | Strategic decisions |

### Contact Information

```
Primary On-Call: +1-555-0123 (Engineer)
Secondary On-Call: +1-555-0124 (Senior Engineer)
System Architect: +1-555-0125
CTO: +1-555-0126
Emergency: +1-555-9999
```

---

*This document should be reviewed and updated monthly. Last updated: January 2025* 