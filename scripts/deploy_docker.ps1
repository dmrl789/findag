# FinDAG Docker Deployment Script
# This script builds and deploys FinDAG using Docker Compose

param(
    [switch]$Build,
    [switch]$Deploy,
    [switch]$Stop,
    [switch]$Clean,
    [switch]$Logs
)

Write-Host "🐳 FinDAG Docker Deployment" -ForegroundColor Green
Write-Host "==========================" -ForegroundColor Green

# Set working directory to docker folder
Set-Location docker

if ($Build) {
    Write-Host "🔨 Building FinDAG Docker image..." -ForegroundColor Cyan
    
    # Build the Docker image
    docker build -t findag:latest ..
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Docker image built successfully" -ForegroundColor Green
    } else {
        Write-Host "❌ Docker build failed!" -ForegroundColor Red
        exit 1
    }
}

if ($Deploy) {
    Write-Host "🚀 Deploying FinDAG with Docker Compose..." -ForegroundColor Cyan
    
    # Start all services
    docker-compose up -d
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ FinDAG deployed successfully!" -ForegroundColor Green
        Write-Host "`n📊 Service URLs:" -ForegroundColor Yellow
        Write-Host "  - FinDAG API: http://localhost:8080" -ForegroundColor White
        Write-Host "  - FinDAG Metrics: http://localhost:9090" -ForegroundColor White
        Write-Host "  - Prometheus: http://localhost:9091" -ForegroundColor White
        Write-Host "  - Grafana: http://localhost:3000 (admin/admin123)" -ForegroundColor White
        Write-Host "  - Redis: localhost:6379" -ForegroundColor White
        
        Write-Host "`n🔍 Checking service health..." -ForegroundColor Yellow
        Start-Sleep -Seconds 10
        
        # Check service health
        $services = @("findag-node", "findag-prometheus", "findag-grafana", "findag-redis")
        foreach ($service in $services) {
            $status = docker ps --filter "name=$service" --format "table {{.Status}}"
            if ($status -match "Up") {
                Write-Host "  ✅ $service is running" -ForegroundColor Green
            } else {
                Write-Host "  ❌ $service is not running" -ForegroundColor Red
            }
        }
    } else {
        Write-Host "❌ Docker Compose deployment failed!" -ForegroundColor Red
        exit 1
    }
}

if ($Stop) {
    Write-Host "🛑 Stopping FinDAG services..." -ForegroundColor Cyan
    docker-compose down
    Write-Host "✅ Services stopped" -ForegroundColor Green
}

if ($Clean) {
    Write-Host "🧹 Cleaning up Docker resources..." -ForegroundColor Cyan
    docker-compose down -v --rmi all
    docker system prune -f
    Write-Host "✅ Cleanup completed" -ForegroundColor Green
}

if ($Logs) {
    Write-Host "📋 Showing FinDAG logs..." -ForegroundColor Cyan
    docker-compose logs -f findag
}

# Return to original directory
Set-Location ..

Write-Host "`n📝 Usage Examples:" -ForegroundColor Cyan
Write-Host "  .\scripts\deploy_docker.ps1 -Build -Deploy    # Build and deploy" -ForegroundColor White
Write-Host "  .\scripts\deploy_docker.ps1 -Stop             # Stop services" -ForegroundColor White
Write-Host "  .\scripts\deploy_docker.ps1 -Clean            # Clean up everything" -ForegroundColor White
Write-Host "  .\scripts\deploy_docker.ps1 -Logs             # View logs" -ForegroundColor White 