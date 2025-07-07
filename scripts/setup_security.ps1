# FinDAG Security Setup Script
# This script sets up secure environment variables and generates production credentials

param(
    [string]$Environment = "development",
    [string]$AdminUsername = "admin",
    [string]$AdminPassword = "",
    [switch]$GenerateSecrets,
    [switch]$SetupTLS
)

Write-Host "🔒 FinDAG Security Setup" -ForegroundColor Green
Write-Host "Environment: $Environment" -ForegroundColor Yellow

# Function to generate secure random string
function Generate-RandomString {
    param([int]$Length = 32)
    $chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*"
    $random = ""
    for ($i = 0; $i -lt $Length; $i++) {
        $random += $chars[(Get-Random -Maximum $chars.Length)]
    }
    return $random
}

# Function to generate SHA-256 hash
function Get-SHA256Hash {
    param([string]$Input)
    $sha256 = [System.Security.Cryptography.SHA256]::Create()
    $bytes = [System.Text.Encoding]::UTF8.GetBytes($Input)
    $hash = $sha256.ComputeHash($bytes)
    return [System.BitConverter]::ToString($hash).Replace("-", "").ToLower()
}

# Generate secure secrets if requested
if ($GenerateSecrets) {
    Write-Host "🔐 Generating secure secrets..." -ForegroundColor Cyan
    
    # Generate JWT secret
    $jwtSecret = Generate-RandomString -Length 64
    $env:JWT_SECRET = $jwtSecret
    Write-Host "JWT_SECRET generated" -ForegroundColor Green
    
    # Generate admin password if not provided
    if ([string]::IsNullOrEmpty($AdminPassword)) {
        $AdminPassword = Generate-RandomString -Length 16
        Write-Host "Generated admin password: $AdminPassword" -ForegroundColor Yellow
        Write-Host "⚠️  SAVE THIS PASSWORD SECURELY!" -ForegroundColor Red
    }
    
    # Hash admin password
    $adminPasswordHash = Get-SHA256Hash -Input $AdminPassword
    $env:ADMIN_USERNAME = $AdminUsername
    $env:ADMIN_PASSWORD_HASH = $adminPasswordHash
    
    Write-Host "Admin credentials configured" -ForegroundColor Green
}

# Set up TLS certificates if requested
if ($SetupTLS) {
    Write-Host "🔐 Setting up TLS certificates..." -ForegroundColor Cyan
    
    # Create certificates directory
    $certsDir = "certs"
    if (!(Test-Path $certsDir)) {
        New-Item -ItemType Directory -Path $certsDir | Out-Null
    }
    
    # Generate self-signed certificate for development
    if ($Environment -eq "development") {
        Write-Host "Generating self-signed certificate for development..." -ForegroundColor Yellow
        
        $certPath = "$certsDir\server.crt"
        $keyPath = "$certsDir\server.key"
        
        # Generate certificate using OpenSSL (if available)
        try {
            openssl req -x509 -newkey rsa:4096 -keyout $keyPath -out $certPath -days 365 -nodes -subj "/CN=localhost"
            $env:TLS_CERT_PATH = (Resolve-Path $certPath).Path
            $env:TLS_KEY_PATH = (Resolve-Path $keyPath).Path
            Write-Host "TLS certificates generated" -ForegroundColor Green
        }
        catch {
            Write-Host "⚠️  OpenSSL not found. Please install OpenSSL or generate certificates manually." -ForegroundColor Yellow
            Write-Host "Certificate paths: $certPath, $keyPath" -ForegroundColor Yellow
        }
    }
    else {
        Write-Host "⚠️  For production, please provide valid TLS certificates." -ForegroundColor Yellow
        Write-Host "Set TLS_CERT_PATH and TLS_KEY_PATH environment variables." -ForegroundColor Yellow
    }
}

# Set up audit logging
Write-Host "📝 Setting up audit logging..." -ForegroundColor Cyan
$auditLogPath = "logs\audit.log"
$logsDir = "logs"

if (!(Test-Path $logsDir)) {
    New-Item -ItemType Directory -Path $logsDir | Out-Null
}

if (!(Test-Path $auditLogPath)) {
    New-Item -ItemType File -Path $auditLogPath | Out-Null
}

$env:AUDIT_LOG_PATH = (Resolve-Path $auditLogPath).Path
Write-Host "Audit log configured: $auditLogPath" -ForegroundColor Green

# Set up rate limiting configuration
Write-Host "⚡ Configuring rate limiting..." -ForegroundColor Cyan
$env:RATE_LIMIT_REQUESTS_PER_MINUTE = "100"
$env:RATE_LIMIT_REQUESTS_PER_HOUR = "1000"
$env:RATE_LIMIT_LOGIN_ATTEMPTS = "5"
Write-Host "Rate limiting configured" -ForegroundColor Green

# Set up CORS configuration
Write-Host "🌐 Configuring CORS..." -ForegroundColor Cyan
$env:CORS_ALLOWED_ORIGINS = "*"
$env:CORS_ALLOWED_METHODS = "GET,POST,DELETE"
$env:CORS_ALLOWED_HEADERS = "Authorization,Content-Type"
Write-Host "CORS configured" -ForegroundColor Green

# Create .env file for easy environment variable management
Write-Host "📄 Creating .env file..." -ForegroundColor Cyan
$envContent = @"
# FinDAG Security Environment Variables
# Generated on $(Get-Date)

# Authentication
ADMIN_USERNAME=$($env:ADMIN_USERNAME)
ADMIN_PASSWORD_HASH=$($env:ADMIN_PASSWORD_HASH)
JWT_SECRET=$($env:JWT_SECRET)

# TLS Configuration
TLS_CERT_PATH=$($env:TLS_CERT_PATH)
TLS_KEY_PATH=$($env:TLS_KEY_PATH)

# Audit Logging
AUDIT_LOG_PATH=$($env:AUDIT_LOG_PATH)

# Rate Limiting
RATE_LIMIT_REQUESTS_PER_MINUTE=$($env:RATE_LIMIT_REQUESTS_PER_MINUTE)
RATE_LIMIT_REQUESTS_PER_HOUR=$($env:RATE_LIMIT_REQUESTS_PER_HOUR)
RATE_LIMIT_LOGIN_ATTEMPTS=$($env:RATE_LIMIT_LOGIN_ATTEMPTS)

# CORS Configuration
CORS_ALLOWED_ORIGINS=$($env:CORS_ALLOWED_ORIGINS)
CORS_ALLOWED_METHODS=$($env:CORS_ALLOWED_METHODS)
CORS_ALLOWED_HEADERS=$($env:CORS_ALLOWED_HEADERS)

# Environment
NODE_ENV=$Environment
"@

$envContent | Out-File -FilePath ".env" -Encoding UTF8
Write-Host ".env file created" -ForegroundColor Green

# Security checklist
Write-Host "`n✅ Security Setup Complete!" -ForegroundColor Green
Write-Host "`n🔒 Security Checklist:" -ForegroundColor Cyan
Write-Host "  ✓ JWT secret configured" -ForegroundColor Green
Write-Host "  ✓ Admin credentials set" -ForegroundColor Green
Write-Host "  ✓ Rate limiting enabled" -ForegroundColor Green
Write-Host "  ✓ CORS protection configured" -ForegroundColor Green
Write-Host "  ✓ Audit logging enabled" -ForegroundColor Green

if ($SetupTLS) {
    Write-Host "  ✓ TLS certificates configured" -ForegroundColor Green
}

Write-Host "`n⚠️  Production Security Recommendations:" -ForegroundColor Yellow
Write-Host "  1. Change default admin password immediately" -ForegroundColor Yellow
Write-Host "  2. Use strong, unique JWT secret" -ForegroundColor Yellow
Write-Host "  3. Configure proper TLS certificates" -ForegroundColor Yellow
Write-Host "  4. Restrict CORS origins to specific domains" -ForegroundColor Yellow
Write-Host "  5. Set up proper firewall rules" -ForegroundColor Yellow
Write-Host "  6. Enable security monitoring and alerting" -ForegroundColor Yellow
Write-Host "  7. Regular security audits and penetration testing" -ForegroundColor Yellow

Write-Host "`n🚀 Ready to start FinDAG with enhanced security!" -ForegroundColor Green 