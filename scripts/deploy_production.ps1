#!/usr/bin/env pwsh
<#
.SYNOPSIS
    FinDAG Production Deployment Script
    
.DESCRIPTION
    This script deploys FinDAG to production environment.
    
.PARAMETER Environment
    Target environment (staging, production)
    
.PARAMETER NodeCount
    Number of nodes to deploy
#>

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("staging", "production")]
    [string]$Environment,
    
    [Parameter(Mandatory=$false)]
    [int]$NodeCount = 3
)

# Set error action preference
$ErrorActionPreference = "Stop"

Write-Host "🚀 FinDAG Production Deployment" -ForegroundColor Green
Write-Host "Environment: $Environment" -ForegroundColor Yellow
Write-Host "Node Count: $NodeCount" -ForegroundColor Yellow
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
    
    # Check if kubectl is available
    try {
        $null = kubectl version --client 2>$null
        Write-Log "✓ kubectl is available" -Level "SUCCESS"
    }
    catch {
        Write-Log "✗ kubectl is not available" -Level "ERROR"
        throw "kubectl is required for deployment"
    }
    
    # Check if Docker is available
    try {
        $null = docker --version 2>$null
        Write-Log "✓ Docker is available" -Level "SUCCESS"
    }
    catch {
        Write-Log "✗ Docker is not available" -Level "ERROR"
        throw "Docker is required for deployment"
    }
}

# Function to generate secure credentials
function New-SecureCredentials {
    Write-Log "Generating secure credentials..."
    
    # Generate JWT secret
    $jwtSecret = -join ((65..90) + (97..122) | Get-Random -Count 64 | ForEach-Object {[char]$_})
    
    # Generate admin password
    $adminPassword = -join ((65..90) + (97..122) + (48..57) | Get-Random -Count 16 | ForEach-Object {[char]$_})
    
    # Generate API keys
    $apiKey1 = "findag_$(-join ((65..90) + (97..122) + (48..57) | Get-Random -Count 16 | ForEach-Object {[char]$_}))"
    $apiKey2 = "findag_$(-join ((65..90) + (97..122) + (48..57) | Get-Random -Count 16 | ForEach-Object {[char]$_}))"
    
    return @{
        JwtSecret = $jwtSecret
        AdminPassword = $adminPassword
        ApiKey1 = $apiKey1
        ApiKey2 = $apiKey2
    }
}

# Function to create Kubernetes namespace
function New-KubernetesNamespace {
    Write-Log "Creating Kubernetes namespace..."
    
    $namespace = "findag-$Environment"
    
    # Create namespace
    kubectl create namespace $namespace --dry-run=client -o yaml | kubectl apply -f -
    
    Write-Log "Namespace '$namespace' created" -Level "SUCCESS"
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
        --from-literal=admin-password=$($Credentials.AdminPassword) `
        --from-literal=jwt-secret=$($Credentials.JwtSecret) `
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
    
    # Create configmap using kubectl
    kubectl create configmap findag-config `
        --namespace=$namespace `
        --from-literal=NODE_ENV=$Environment `
        --from-literal=LOG_LEVEL=info `
        --from-literal=API_PORT=8080 `
        --from-literal=METRICS_PORT=9090 `
        --from-literal=P2P_PORT=30333 `
        --from-literal=DATABASE_PATH=/data/findag.db `
        --from-literal=MAX_TRANSACTIONS_PER_BLOCK=10000 `
        --from-literal=ROUND_INTERVAL_MS=200 `
        --from-literal=FINALITY_THRESHOLD=0.67 `
        --dry-run=client -o yaml | kubectl apply -f -
    
    Write-Log "Kubernetes configmap created" -Level "SUCCESS"
}

# Function to deploy FinDAG
function Install-FinDAG {
    Write-Log "Deploying FinDAG..."
    
    $namespace = "findag-$Environment"
    
    # Create deployment YAML file
    $deploymentYaml = @"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: findag
  namespace: $namespace
spec:
  replicas: $NodeCount
  selector:
    matchLabels:
      app: findag
  template:
    metadata:
      labels:
        app: findag
    spec:
      containers:
      - name: findag
        image: findag/findag:latest
        ports:
        - containerPort: 8080
        - containerPort: 9090
        env:
        - name: NODE_ENV
          valueFrom:
            configMapKeyRef:
              name: findag-config
              key: NODE_ENV
        - name: LOG_LEVEL
          valueFrom:
            configMapKeyRef:
              name: findag-config
              key: LOG_LEVEL
        - name: API_PORT
          valueFrom:
            configMapKeyRef:
              name: findag-config
              key: API_PORT
        - name: METRICS_PORT
          valueFrom:
            configMapKeyRef:
              name: findag-config
              key: METRICS_PORT
        - name: P2P_PORT
          valueFrom:
            configMapKeyRef:
              name: findag-config
              key: P2P_PORT
        - name: DATABASE_PATH
          valueFrom:
            configMapKeyRef:
              name: findag-config
              key: DATABASE_PATH
        - name: MAX_TRANSACTIONS_PER_BLOCK
          valueFrom:
            configMapKeyRef:
              name: findag-config
              key: MAX_TRANSACTIONS_PER_BLOCK
        - name: ROUND_INTERVAL_MS
          valueFrom:
            configMapKeyRef:
              name: findag-config
              key: ROUND_INTERVAL_MS
        - name: FINALITY_THRESHOLD
          valueFrom:
            configMapKeyRef:
              name: findag-config
              key: FINALITY_THRESHOLD
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: findag-credentials
              key: jwt-secret
        - name: ADMIN_USERNAME
          valueFrom:
            secretKeyRef:
              name: findag-credentials
              key: admin-username
        - name: ADMIN_PASSWORD
          valueFrom:
            secretKeyRef:
              name: findag-credentials
              key: admin-password
        resources:
          requests:
            memory: "2Gi"
            cpu: "1000m"
          limits:
            memory: "4Gi"
            cpu: "2000m"
        volumeMounts:
        - name: data
          mountPath: /data
      volumes:
      - name: data
        persistentVolumeClaim:
          claimName: findag-data
"@
    
    $deploymentYaml | Out-File -FilePath "k8s/findag-deployment.yaml" -Encoding UTF8
    kubectl apply -f k8s/findag-deployment.yaml
    
    Write-Log "FinDAG deployment created" -Level "SUCCESS"
}

# Function to create service
function New-Service {
    Write-Log "Creating service..."
    
    $namespace = "findag-$Environment"
    
    # Create service YAML file
    $serviceYaml = @"
apiVersion: v1
kind: Service
metadata:
  name: findag
  namespace: $namespace
spec:
  type: LoadBalancer
  ports:
  - port: 8080
    targetPort: 8080
    protocol: TCP
    name: api
  - port: 9090
    targetPort: 9090
    protocol: TCP
    name: metrics
  selector:
    app: findag
"@
    
    $serviceYaml | Out-File -FilePath "k8s/findag-service.yaml" -Encoding UTF8
    kubectl apply -f k8s/findag-service.yaml
    
    Write-Log "Service created" -Level "SUCCESS"
}

# Function to create persistent volume claim
function New-PersistentVolumeClaim {
    Write-Log "Creating persistent volume claim..."
    
    $namespace = "findag-$Environment"
    
    # Create PVC YAML file
    $pvcYaml = @"
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: findag-data
  namespace: $namespace
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 100Gi
"@
    
    $pvcYaml | Out-File -FilePath "k8s/findag-pvc.yaml" -Encoding UTF8
    kubectl apply -f k8s/findag-pvc.yaml
    
    Write-Log "Persistent volume claim created" -Level "SUCCESS"
}

# Function to run health checks
function Test-HealthChecks {
    Write-Log "Running health checks..."
    
    $namespace = "findag-$Environment"
    
    # Wait for pods to be ready
    Write-Log "Waiting for pods to be ready..."
    kubectl wait --for=condition=ready pod -l app=findag -n $namespace --timeout=300s
    
    # Check pod status
    $pods = kubectl get pods -n $namespace -l app=findag
    Write-Log "Pod status: $pods" -Level "SUCCESS"
    
    # Check service
    $service = kubectl get service findag -n $namespace
    Write-Log "Service status: $service" -Level "SUCCESS"
    
    Write-Log "Health checks completed" -Level "SUCCESS"
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
- **Node Count**: $NodeCount
- **Deployment Date**: $(Get-Date)

## Credentials (SAVE SECURELY)
- **Admin Username**: admin
- **Admin Password**: $($Credentials.AdminPassword)
- **JWT Secret**: $($Credentials.JwtSecret)
- **API Key 1**: $($Credentials.ApiKey1)
- **API Key 2**: $($Credentials.ApiKey2)

## Infrastructure Details
- **Namespace**: findag-$Environment
- **Service Type**: LoadBalancer
- **Storage**: 100Gi per node
- **Memory**: 2Gi-4Gi per node
- **CPU**: 1-2 cores per node

## Access Information
- **API Endpoint**: http://localhost:8080 (or LoadBalancer IP)
- **Metrics Endpoint**: http://localhost:9090 (or LoadBalancer IP)

## Next Steps
1. Get LoadBalancer IP: kubectl get service findag -n findag-$Environment
2. Test API endpoint
3. Configure monitoring
4. Set up backup procedures
5. Document operational procedures

## Support Information
- **Documentation**: https://docs.findag.com
- **Support Email**: support@findag.com

---
*Generated on $(Get-Date) by FinDAG Deployment Script*
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
    Write-Log "Starting FinDAG production deployment..."
    
    # Check prerequisites
    Test-Prerequisites
    
    # Generate secure credentials
    $credentials = New-SecureCredentials
    
    # Create Kubernetes namespace
    New-KubernetesNamespace
    
    # Create Kubernetes secrets
    New-KubernetesSecrets -Credentials $credentials
    
    # Create Kubernetes configmap
    New-KubernetesConfigMap
    
    # Create persistent volume claim
    New-PersistentVolumeClaim
    
    # Deploy FinDAG
    Install-FinDAG
    
    # Create service
    New-Service
    
    # Run health checks
    Test-HealthChecks
    
    # Generate deployment report
    New-DeploymentReport -Credentials $credentials
    
    Write-Log "Production deployment completed successfully!" -Level "SUCCESS"
    Write-Log "Environment: $Environment is ready for use" -Level "SUCCESS"
    Write-Log "Next: Get LoadBalancer IP and test API endpoint" -Level "SUCCESS"
    
}
catch {
    Write-Log "Deployment failed: $($_.Exception.Message)" -Level "ERROR"
    Write-Log "Stack trace: $($_.ScriptStackTrace)" -Level "ERROR"
    exit 1
} 