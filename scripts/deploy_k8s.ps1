# FinDAG Kubernetes Deployment Script
# This script deploys FinDAG to Kubernetes

param(
    [switch]$Deploy,
    [switch]$Delete,
    [switch]$Status,
    [switch]$Logs,
    [string]$Namespace = "findag"
)

Write-Host "☸️  FinDAG Kubernetes Deployment" -ForegroundColor Green
Write-Host "===============================" -ForegroundColor Green

if ($Deploy) {
    Write-Host "🚀 Deploying FinDAG to Kubernetes..." -ForegroundColor Cyan
    
    # Create namespace
    Write-Host "Creating namespace..." -ForegroundColor Yellow
    kubectl apply -f k8s/namespace.yml
    
    # Apply ConfigMap and Secret
    Write-Host "Applying configuration..." -ForegroundColor Yellow
    kubectl apply -f k8s/configmap.yml
    kubectl apply -f k8s/secret.yml
    
    # Create persistent volumes
    Write-Host "Creating persistent volumes..." -ForegroundColor Yellow
    kubectl apply -f k8s/pvc.yml
    
    # Deploy the application
    Write-Host "Deploying FinDAG..." -ForegroundColor Yellow
    kubectl apply -f k8s/deployment.yml
    
    # Create services
    Write-Host "Creating services..." -ForegroundColor Yellow
    kubectl apply -f k8s/service.yml
    
    # Create HPA
    Write-Host "Creating autoscaler..." -ForegroundColor Yellow
    kubectl apply -f k8s/hpa.yml
    
    Write-Host "✅ FinDAG deployed to Kubernetes!" -ForegroundColor Green
    Write-Host "`n📊 Next steps:" -ForegroundColor Yellow
    Write-Host "  - Check status: .\scripts\deploy_k8s.ps1 -Status" -ForegroundColor White
    Write-Host "  - View logs: .\scripts\deploy_k8s.ps1 -Logs" -ForegroundColor White
    Write-Host "  - Get external IP: kubectl get svc -n $Namespace" -ForegroundColor White
}

if ($Delete) {
    Write-Host "🗑️  Deleting FinDAG from Kubernetes..." -ForegroundColor Cyan
    
    # Delete all resources
    kubectl delete -f k8s/ --ignore-not-found=true
    kubectl delete namespace $Namespace --ignore-not-found=true
    
    Write-Host "✅ FinDAG deleted from Kubernetes" -ForegroundColor Green
}

if ($Status) {
    Write-Host "📊 FinDAG Status in Kubernetes:" -ForegroundColor Cyan
    
    # Check namespace
    Write-Host "`nNamespace:" -ForegroundColor Yellow
    kubectl get namespace $Namespace
    
    # Check pods
    Write-Host "`nPods:" -ForegroundColor Yellow
    kubectl get pods -n $Namespace
    
    # Check services
    Write-Host "`nServices:" -ForegroundColor Yellow
    kubectl get svc -n $Namespace
    
    # Check HPA
    Write-Host "`nAutoscaler:" -ForegroundColor Yellow
    kubectl get hpa -n $Namespace
    
    # Check PVCs
    Write-Host "`nPersistent Volumes:" -ForegroundColor Yellow
    kubectl get pvc -n $Namespace
}

if ($Logs) {
    Write-Host "📋 FinDAG Pod Logs:" -ForegroundColor Cyan
    
    # Get pod name
    $podName = kubectl get pods -n $Namespace -l app=findag -o jsonpath='{.items[0].metadata.name}'
    
    if ($podName) {
        Write-Host "Showing logs for pod: $podName" -ForegroundColor Yellow
        kubectl logs -f -n $Namespace $podName
    } else {
        Write-Host "No FinDAG pods found" -ForegroundColor Red
    }
}

Write-Host "`n📝 Usage Examples:" -ForegroundColor Cyan
Write-Host "  .\scripts\deploy_k8s.ps1 -Deploy           # Deploy to Kubernetes" -ForegroundColor White
Write-Host "  .\scripts\deploy_k8s.ps1 -Status           # Check deployment status" -ForegroundColor White
Write-Host "  .\scripts\deploy_k8s.ps1 -Logs             # View pod logs" -ForegroundColor White
Write-Host "  .\scripts\deploy_k8s.ps1 -Delete           # Delete deployment" -ForegroundColor White 