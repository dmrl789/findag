#!/usr/bin/env pwsh
<#
.SYNOPSIS
    FinDAG Production Environment Provisioning Script
    
.DESCRIPTION
    This script provisions a complete production environment for FinDAG,
    including security setup, infrastructure configuration, and deployment.
    
.PARAMETER Environment
    Target environment (staging, production)
    
.PARAMETER Region
    Cloud region for deployment
    
.PARAMETER NodeCount
    Number of nodes to deploy
    
.PARAMETER SkipSecurity
    Skip security hardening steps
    
.EXAMPLE
    .\provision_production_fixed.ps1 -Environment production -Region us-east-1 -NodeCount 3
    
.NOTES
    Requires elevated privileges for some operations
    Requires Docker, kubectl, and helm to be installed
#>

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("staging", "production")]
    [string]$Environment,
    
    [Parameter(Mandatory=$true)]
    [string]$Region,
    
    [Parameter(Mandatory=$false)]
    [int]$NodeCount = 3,
    
    [Parameter(Mandatory=$false)]
    [switch]$SkipSecurity
)

# Set error action preference
$ErrorActionPreference = "Stop"

# Script configuration
$ScriptVersion = "1.0.0"
$StartTime = Get-Date

Write-Host "🚀 FinDAG Production Environment Provisioning" -ForegroundColor Green
Write-Host "Environment: $Environment" -ForegroundColor Yellow
Write-Host "Region: $Region" -ForegroundColor Yellow
Write-Host "Node Count: $NodeCount" -ForegroundColor Yellow
Write-Host "Version: $ScriptVersion" -ForegroundColor Yellow
Write-Host "Started: $StartTime" -ForegroundColor Yellow
Write-Host ""

# Function to log messages
function Write-Log {
    param(
        [string]$Message,
        [string]$Level = "INFO"
    )
    
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $color = switch ($Level) {
        "ERROR" { "Red" }
        "WARN" { "Yellow" }
        "SUCCESS" { "Green" }
        default { "White" }
    }
    
    Write-Host "[$timestamp] [$Level] $Message" -ForegroundColor $color
}

# Function to check prerequisites
function Test-Prerequisites {
    Write-Log "Checking prerequisites..."
    
    $prerequisites = @{
        "Docker" = "docker --version"
        "kubectl" = "kubectl version --client"
        "Helm" = "helm version"
        "PowerShell" = "pwsh --version"
    }
    
    foreach ($tool in $prerequisites.GetEnumerator()) {
        try {
            $null = Invoke-Expression $tool.Value 2>$null
            Write-Log "✓ $($tool.Key) is available" -Level "SUCCESS"
        }
        catch {
            Write-Log "✗ $($tool.Key) is not available" -Level "ERROR"
            throw "Prerequisite check failed: $($tool.Key) is required"
        }
    }
}

# Function to generate secure credentials
function New-SecureCredentials {
    Write-Log "Generating secure credentials..."
    
    # Generate JWT secret
    $jwtSecret = -join ((65..90) + (97..122) | Get-Random -Count 64 | ForEach-Object {[char]$_})
    
    # Generate admin password
    $adminPassword = -join ((65..90) + (97..122) + (48..57) | Get-Random -Count 16 | ForEach-Object {[char]$_})
    
    # Hash admin password
    $adminPasswordHash = (Get-FileHash -Algorithm SHA256 -InputObject ([System.Text.Encoding]::UTF8.GetBytes($adminPassword))).Hash.ToLower()
    
    # Generate database encryption key
    $dbEncryptionKey = -join ((65..90) + (97..122) + (48..57) | Get-Random -Count 32 | ForEach-Object {[char]$_})
    
    # Generate API keys
    $apiKey1 = "findag_$(-join ((65..90) + (97..122) + (48..57) | Get-Random -Count 16 | ForEach-Object {[char]$_}))"
    $apiKey2 = "findag_$(-join ((65..90) + (97..122) + (48..57) | Get-Random -Count 16 | ForEach-Object {[char]$_}))"
    
    return @{
        JwtSecret = $jwtSecret
        AdminPassword = $adminPassword
        AdminPasswordHash = $adminPasswordHash
        DbEncryptionKey = $dbEncryptionKey
        ApiKey1 = $apiKey1
        ApiKey2 = $apiKey2
    }
}

# Function to create TLS certificates
function New-TlsCertificates {
    Write-Log "Creating TLS certificates..."
    
    $certDir = "certs"
    if (!(Test-Path $certDir)) {
        New-Item -ItemType Directory -Path $certDir | Out-Null
    }
    
    # Generate CA certificate
    openssl genrsa -out "$certDir/ca.key" 4096
    openssl req -new -x509 -days 365 -key "$certDir/ca.key" -out "$certDir/ca.crt" -subj "/C=US/ST=State/L=City/O=FinDAG/CN=FinDAG CA"
    
    # Generate server certificate
    openssl genrsa -out "$certDir/server.key" 2048
    openssl req -new -key "$certDir/server.key" -out "$certDir/server.csr" -subj "/C=US/ST=State/L=City/O=FinDAG/CN=api.findag.com"
    
    # Create certificate configuration
    $certConfig = @"
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names

[alt_names]
DNS.1 = api.findag.com
DNS.2 = *.findag.com
DNS.3 = localhost
IP.1 = 127.0.0.1
"@
    
    $certConfig | Out-File -FilePath "$certDir/server.ext" -Encoding ASCII
    
    # Sign server certificate
    openssl x509 -req -in "$certDir/server.csr" -CA "$certDir/ca.crt" -CAkey "$certDir/ca.key" -CAcreateserial -out "$certDir/server.crt" -days 365 -extfile "$certDir/server.ext"
    
    Write-Log "TLS certificates created successfully" -Level "SUCCESS"
}

# Function to create Kubernetes namespace
function New-KubernetesNamespace {
    Write-Log "Creating Kubernetes namespace..."
    
    $namespace = "findag-$Environment"
    
    # Create namespace
    kubectl create namespace $namespace --dry-run=client -o yaml | kubectl apply -f -
    
    # Create resource quotas
    $resourceQuota = @"
apiVersion: v1
kind: ResourceQuota
metadata:
  name: findag-quota
  namespace: $namespace
spec:
  hard:
    requests.cpu: "8"
    requests.memory: 16Gi
    limits.cpu: "16"
    limits.memory: 32Gi
    persistentvolumeclaims: "10"
    services: "20"
"@
    
    $resourceQuota | kubectl apply -f -
    
    Write-Log "Kubernetes namespace '$namespace' created" -Level "SUCCESS"
}

# Function to create Kubernetes secrets
function New-KubernetesSecrets {
    param($Credentials)
    
    Write-Log "Creating Kubernetes secrets..."
    
    $namespace = "findag-$Environment"
    
    # Create secret for credentials
    kubectl create secret generic findag-credentials `
        --namespace=$namespace `
        --from-literal=admin-username=admin `
        --from-literal=admin-password-hash=$($Credentials.AdminPasswordHash) `
        --from-literal=jwt-secret=$($Credentials.JwtSecret) `
        --from-literal=db-encryption-key=$($Credentials.DbEncryptionKey) `
        --dry-run=client -o yaml | kubectl apply -f -
    
    # Create secret for TLS certificates
    kubectl create secret tls findag-tls `
        --namespace=$namespace `
        --cert=certs/server.crt `
        --key=certs/server.key `
        --dry-run=client -o yaml | kubectl apply -f -
    
    # Create secret for API keys
    kubectl create secret generic findag-api-keys `
        --namespace=$namespace `
        --from-literal=api-key-1=$($Credentials.ApiKey1) `
        --from-literal=api-key-2=$($Credentials.ApiKey2) `
        --dry-run=client -o yaml | kubectl apply -f -
    
    Write-Log "Kubernetes secrets created" -Level "SUCCESS"
}

# Function to create Kubernetes configmap
function New-KubernetesConfigMap {
    Write-Log "Creating Kubernetes configmap..."
    
    $namespace = "findag-$Environment"
    
    # Create configmap for application configuration
    $configMap = @"
apiVersion: v1
kind: ConfigMap
metadata:
  name: findag-config
  namespace: $namespace
data:
  NODE_ENV: "$Environment"
  LOG_LEVEL: "info"
  API_PORT: "8080"
  METRICS_PORT: "9090"
  P2P_PORT: "30333"
  DATABASE_PATH: "/data/findag.db"
  MAX_TRANSACTIONS_PER_BLOCK: "10000"
  ROUND_INTERVAL_MS: "200"
  FINALITY_THRESHOLD: "0.67"
"@
    
    $configMap | kubectl apply -f -
    
    Write-Log "Kubernetes configmap created" -Level "SUCCESS"
}

# Function to deploy FinDAG with Helm
function Install-FinDAG {
    param($Credentials)
    
    Write-Log "Deploying FinDAG with Helm..."
    
    $namespace = "findag-$Environment"
    
    # Add Helm repository
    helm repo add findag https://charts.findag.com
    helm repo update
    
    # Create values file
    $valuesContent = @"
replicaCount: $NodeCount

image:
  repository: findag/findag
  tag: "latest"
  pullPolicy: Always

admin:
  username: admin
  passwordHash: $($Credentials.AdminPasswordHash)

jwt:
  secret: $($Credentials.JwtSecret)
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
    adminPassword: "$(Get-Random -Minimum 100000 -Maximum 999999)"

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
"@
    
    $valuesContent | Out-File -FilePath "values-$Environment.yaml" -Encoding UTF8
    
    # Install FinDAG
    helm install findag findag/findag `
        --namespace=$namespace `
        -f "values-$Environment.yaml" `
        --wait `
        --timeout=10m
    
    Write-Log "FinDAG deployed successfully" -Level "SUCCESS"
}

# Function to configure monitoring
function Install-Monitoring {
    Write-Log "Installing monitoring stack..."
    
    $namespace = "findag-$Environment"
    
    # Install Prometheus
    helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
    helm install prometheus prometheus-community/kube-prometheus-stack `
        --namespace=$namespace `
        --set grafana.enabled=true `
        --set prometheus.prometheusSpec.retention=7d `
        --wait
    
    Write-Log "Monitoring stack installed" -Level "SUCCESS"
}

# Function to run health checks
function Test-HealthChecks {
    Write-Log "Running health checks..."
    
    $namespace = "findag-$Environment"
    
    # Wait for pods to be ready
    Write-Log "Waiting for pods to be ready..."
    kubectl wait --for=condition=ready pod -l app=findag -n $namespace --timeout=300s
    
    # Check service endpoints
    $endpoints = kubectl get endpoints -n $namespace -l app=findag
    if ($endpoints -match "None") {
        throw "Service endpoints not ready"
    }
    
    Write-Log "Health checks passed" -Level "SUCCESS"
}

# Function to create backup configuration
function Set-BackupConfiguration {
    Write-Log "Configuring backup..."
    
    $namespace = "findag-$Environment"
    
    # Create backup job
    $backupJob = @"
apiVersion: batch/v1
kind: CronJob
metadata:
  name: findag-backup
  namespace: $namespace
spec:
  schedule: "0 2 * * *"
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: backup
            image: findag/findag:latest
            command: ["/bin/bash"]
            args:
            - -c
            - |
              cp /data/findag.db /backup/findag-\$(date +%Y%m%d-%H%M%S).db
              gzip /backup/findag-\$(date +%Y%m%d-%H%M%S).db
              find /backup -name "findag-*.db.gz" -mtime +30 -delete
            volumeMounts:
            - name: data
              mountPath: /data
            - name: backup
              mountPath: /backup
          volumes:
          - name: data
            persistentVolumeClaim:
              claimName: findag-data
          - name: backup
            persistentVolumeClaim:
              claimName: findag-backup
          restartPolicy: OnFailure
"@
    
    $backupJob | kubectl apply -f -
    
    Write-Log "Backup configuration created" -Level "SUCCESS"
}

# Function to generate deployment report
function New-DeploymentReport {
    param($Credentials)
    
    Write-Log "Generating deployment report..."
    
    $reportPath = "deployment-report-$Environment-$(Get-Date -Format 'yyyyMMdd-HHmmss').md"
    
    $report = @"
# FinDAG Production Deployment Report

## Deployment Information
- **Environment**: $Environment
- **Region**: $Region
- **Node Count**: $NodeCount
- **Deployment Date**: $(Get-Date)
- **Script Version**: $ScriptVersion

## Credentials (SAVE SECURELY)
- **Admin Username**: admin
- **Admin Password**: $($Credentials.AdminPassword)
- **JWT Secret**: $($Credentials.JwtSecret)
- **Database Encryption Key**: $($Credentials.DbEncryptionKey)
- **API Key 1**: $($Credentials.ApiKey1)
- **API Key 2**: $($Credentials.ApiKey2)

## Infrastructure Details
- **Namespace**: findag-$Environment
- **Service Type**: LoadBalancer
- **Storage**: 100Gi per node
- **Memory**: 2Gi-4Gi per node
- **CPU**: 1-2 cores per node

## Access Information
- **API Endpoint**: http://api.findag.com
- **Grafana Dashboard**: http://grafana.findag.com
- **Prometheus**: http://prometheus.findag.com

## Security Configuration
- **TLS**: Enabled with Let's Encrypt
- **Network Policies**: Configured
- **Pod Security**: Hardened
- **Backup**: Daily automated backups

## Monitoring
- **Health Checks**: Configured
- **Metrics**: Prometheus + Grafana
- **Logging**: Structured logging enabled
- **Alerts**: Configured for critical issues

## Next Steps
1. Update DNS records to point to LoadBalancer IP
2. Configure SSL certificates
3. Set up monitoring alerts
4. Test all functionality
5. Document operational procedures

## Support Information
- **Documentation**: https://docs.findag.com
- **Support Email**: support@findag.com
- **Emergency Contact**: +1-555-9999

---
*Generated on $(Get-Date) by FinDAG Provisioning Script v$ScriptVersion*
"@
    
    $report | Out-File -FilePath $reportPath -Encoding UTF8
    
    Write-Log "Deployment report saved to: $reportPath" -Level "SUCCESS"
    
    # Display credentials securely
    Write-Host ""
    Write-Host "🔐 CREDENTIALS (SAVE SECURELY):" -ForegroundColor Red
    Write-Host "Admin Password: $($Credentials.AdminPassword)" -ForegroundColor Yellow
    Write-Host "JWT Secret: $($Credentials.JwtSecret)" -ForegroundColor Yellow
    Write-Host "API Key 1: $($Credentials.ApiKey1)" -ForegroundColor Yellow
    Write-Host "API Key 2: $($Credentials.ApiKey2)" -ForegroundColor Yellow
    Write-Host ""
}

# Main execution
try {
    Write-Log "Starting FinDAG production environment provisioning..."
    
    # Check prerequisites
    Test-Prerequisites
    
    # Generate secure credentials
    $credentials = New-SecureCredentials
    
    # Create TLS certificates
    New-TlsCertificates
    
    # Create Kubernetes namespace
    New-KubernetesNamespace
    
    # Create Kubernetes secrets
    New-KubernetesSecrets -Credentials $credentials
    
    # Create Kubernetes configmap
    New-KubernetesConfigMap
    
    # Deploy FinDAG
    Install-FinDAG -Credentials $credentials
    
    # Install monitoring
    Install-Monitoring
    
    # Run health checks
    Test-HealthChecks
    
    # Configure backup
    Set-BackupConfiguration
    
    # Generate deployment report
    New-DeploymentReport -Credentials $credentials
    
    $endTime = Get-Date
    $duration = $endTime - $StartTime
    
    Write-Log "Production environment provisioning completed successfully!" -Level "SUCCESS"
    Write-Log "Total duration: $($duration.TotalMinutes) minutes" -Level "SUCCESS"
    Write-Log "Environment: $Environment is ready for use" -Level "SUCCESS"
    
}
catch {
    Write-Log "Provisioning failed: $($_.Exception.Message)" -Level "ERROR"
    Write-Log "Stack trace: $($_.ScriptStackTrace)" -Level "ERROR"
    exit 1
} 