#!/bin/bash

# FinDAG Tauri Setup Script
# This script sets up the development environment for FinDAG Desktop

set -e

echo "🚀 Setting up FinDAG Tauri Development Environment"
echo "=================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "package.json" ] || [ ! -f "src-tauri/Cargo.toml" ]; then
    print_error "Please run this script from the findag-tauri directory"
    exit 1
fi

# Check prerequisites
print_status "Checking prerequisites..."

# Check Node.js
if ! command -v node &> /dev/null; then
    print_error "Node.js is not installed. Please install Node.js 18+ first."
    exit 1
fi

NODE_VERSION=$(node --version | cut -d'v' -f2 | cut -d'.' -f1)
if [ "$NODE_VERSION" -lt 18 ]; then
    print_error "Node.js version 18+ is required. Current version: $(node --version)"
    exit 1
fi
print_success "Node.js $(node --version) is installed"

# Check npm
if ! command -v npm &> /dev/null; then
    print_error "npm is not installed"
    exit 1
fi
print_success "npm $(npm --version) is installed"

# Check Rust
if ! command -v rustc &> /dev/null; then
    print_error "Rust is not installed. Please install Rust first: https://rustup.rs/"
    exit 1
fi
print_success "Rust $(rustc --version) is installed"

# Check Cargo
if ! command -v cargo &> /dev/null; then
    print_error "Cargo is not installed"
    exit 1
fi
print_success "Cargo $(cargo --version) is installed"

# Check Tauri CLI
if ! command -v tauri &> /dev/null; then
    print_warning "Tauri CLI is not installed. Installing..."
    npm install -g @tauri-apps/cli
fi
print_success "Tauri CLI is installed"

# Install frontend dependencies
print_status "Installing frontend dependencies..."
npm install
print_success "Frontend dependencies installed"

# Install Rust dependencies
print_status "Installing Rust dependencies..."
cargo install
print_success "Rust dependencies installed"

# Create necessary directories
print_status "Creating necessary directories..."
mkdir -p src-tauri/configs
mkdir -p src-tauri/assets
mkdir -p public
mkdir -p docs
print_success "Directories created"

# Create default configuration
print_status "Creating default configuration..."
cat > src-tauri/configs/default.toml << EOF
# FinDAG Default Configuration

[consensus]
round_interval_ms = 200
max_transactions_per_block = 1000
finality_threshold = 0.67

[network]
p2p_port = 30333
max_peers = 50
encryption_enabled = true

[security]
jwt_secret = "changeme_in_production"
rbac_enabled = true

[storage]
data_dir = "findag_data"
backup_enabled = true
backup_interval_hours = 24

[performance]
enable_profiling = false
debug_logging = false
metrics_enabled = true
EOF
print_success "Default configuration created"

# Create development configuration
print_status "Creating development configuration..."
cat > src-tauri/configs/development.toml << EOF
# FinDAG Development Configuration

[consensus]
round_interval_ms = 100
max_transactions_per_block = 500
finality_threshold = 0.67

[network]
p2p_port = 30334
max_peers = 10
encryption_enabled = false

[security]
jwt_secret = "dev_secret_key"
rbac_enabled = false

[storage]
data_dir = "findag_dev_data"
backup_enabled = false
backup_interval_hours = 24

[performance]
enable_profiling = true
debug_logging = true
metrics_enabled = true
EOF
print_success "Development configuration created"

# Create .gitignore if it doesn't exist
if [ ! -f ".gitignore" ]; then
    print_status "Creating .gitignore..."
    cat > .gitignore << EOF
# Dependencies
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Build outputs
dist/
build/
target/

# Environment files
.env
.env.local
.env.development.local
.env.test.local
.env.production.local

# IDE files
.vscode/
.idea/
*.swp
*.swo

# OS files
.DS_Store
Thumbs.db

# Logs
logs/
*.log

# Runtime data
pids/
*.pid
*.seed
*.pid.lock

# Coverage directory used by tools like istanbul
coverage/

# Tauri
src-tauri/target/
src-tauri/WixTools.exe

# FinDAG specific
findag_data/
findag_dev_data/
*.db
*.sled
EOF
    print_success ".gitignore created"
fi

# Create development scripts
print_status "Creating development scripts..."
cat > scripts/dev.sh << 'EOF'
#!/bin/bash
# Development script for FinDAG Tauri

echo "🚀 Starting FinDAG Tauri Development Server"
echo "==========================================="

# Check if we're in the right directory
if [ ! -f "package.json" ] || [ ! -f "src-tauri/Cargo.toml" ]; then
    echo "❌ Please run this script from the findag-tauri directory"
    exit 1
fi

# Start development server
echo "📦 Starting development server..."
npm run tauri:dev
EOF

cat > scripts/build.sh << 'EOF'
#!/bin/bash
# Build script for FinDAG Tauri

echo "🔨 Building FinDAG Tauri Application"
echo "===================================="

# Check if we're in the right directory
if [ ! -f "package.json" ] || [ ! -f "src-tauri/Cargo.toml" ]; then
    echo "❌ Please run this script from the findag-tauri directory"
    exit 1
fi

# Build the application
echo "📦 Building application..."
npm run tauri:build

echo "✅ Build completed successfully!"
echo "📁 Output files are in the src-tauri/target/release directory"
EOF

cat > scripts/test.sh << 'EOF'
#!/bin/bash
# Test script for FinDAG Tauri

echo "🧪 Running FinDAG Tauri Tests"
echo "=============================="

# Check if we're in the right directory
if [ ! -f "package.json" ] || [ ! -f "src-tauri/Cargo.toml" ]; then
    echo "❌ Please run this script from the findag-tauri directory"
    exit 1
fi

# Run frontend tests
echo "🧪 Running frontend tests..."
npm test

# Run backend tests
echo "🧪 Running backend tests..."
cd src-tauri
cargo test
cd ..

echo "✅ All tests completed!"
EOF

# Make scripts executable
chmod +x scripts/dev.sh
chmod +x scripts/build.sh
chmod +x scripts/test.sh
print_success "Development scripts created"

# Create documentation
print_status "Creating documentation..."
mkdir -p docs
cat > docs/DEVELOPMENT.md << EOF
# FinDAG Tauri Development Guide

## Quick Start

1. **Setup Environment**
   \`\`\`bash
   ./scripts/setup.sh
   \`\`\`

2. **Start Development Server**
   \`\`\`bash
   ./scripts/dev.sh
   \`\`\`

3. **Run Tests**
   \`\`\`bash
   ./scripts/test.sh
   \`\`\`

4. **Build for Production**
   \`\`\`bash
   ./scripts/build.sh
   \`\`\`

## Project Structure

- \`src-tauri/\`: Rust backend (Tauri)
- \`src/\`: React frontend
- \`configs/\`: Configuration files
- \`scripts/\`: Development scripts
- \`docs/\`: Documentation

## Development Workflow

1. Make changes to the code
2. Run tests: \`./scripts/test.sh\`
3. Start development server: \`./scripts/dev.sh\`
4. Test your changes
5. Commit and push

## Useful Commands

- \`npm run tauri:dev\`: Start development server
- \`npm run tauri:build\`: Build for production
- \`npm test\`: Run frontend tests
- \`cargo test\`: Run backend tests
- \`cargo clippy\`: Run Rust linter
- \`npm run lint\`: Run TypeScript linter
EOF
print_success "Documentation created"

# Final setup message
echo ""
print_success "🎉 FinDAG Tauri development environment is ready!"
echo ""
echo "📋 Next steps:"
echo "   1. Start development: ./scripts/dev.sh"
echo "   2. Run tests: ./scripts/test.sh"
echo "   3. Build for production: ./scripts/build.sh"
echo ""
echo "📚 Documentation: docs/DEVELOPMENT.md"
echo "🔧 Configuration: src-tauri/configs/"
echo ""
echo "Happy coding! 🚀" 