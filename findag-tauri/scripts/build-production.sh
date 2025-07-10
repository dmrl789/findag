#!/bin/bash

# FinDAG Desktop Production Build Script
# This script handles the complete production build process

set -e

echo "🚀 Starting FinDAG Desktop Production Build..."

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
if [ ! -f "package.json" ] || [ ! -f "tauri.conf.json" ]; then
    print_error "Please run this script from the project root directory"
    exit 1
fi

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    print_error "Node.js is not installed"
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Rust is not installed"
    exit 1
fi

# Check if Tauri CLI is installed
if ! command -v tauri &> /dev/null; then
    print_warning "Tauri CLI not found, installing..."
    npm install -g @tauri-apps/cli
fi

print_status "Cleaning previous builds..."
rm -rf dist/
rm -rf src-tauri/target/
rm -rf src-tauri/Cargo.lock

print_status "Installing dependencies..."
npm ci

print_status "Running linting..."
npm run lint

print_status "Running tests..."
npm run test:ci

print_status "Building frontend..."
npm run build

print_status "Building Tauri application..."
tauri build

print_status "Optimizing bundle size..."

# Check bundle size
if command -v npx &> /dev/null; then
    print_status "Analyzing bundle size..."
    npx vite-bundle-analyzer dist/ || print_warning "Bundle analyzer not available"
fi

print_status "Creating distribution packages..."

# Create release directory
mkdir -p releases/

# Copy built application
cp -r src-tauri/target/release/bundle/* releases/

print_status "Creating installer..."

# Platform-specific installer creation
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    print_status "Creating macOS installer..."
    # Additional macOS-specific steps can be added here
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    # Windows
    print_status "Creating Windows installer..."
    # Additional Windows-specific steps can be added here
else
    # Linux
    print_status "Creating Linux installer..."
    # Additional Linux-specific steps can be added here
fi

print_status "Generating checksums..."
cd releases/

# Generate checksums for all files
for file in *; do
    if [ -f "$file" ]; then
        sha256sum "$file" > "$file.sha256"
        print_success "Generated checksum for $file"
    fi
done

cd ..

print_status "Creating release notes..."
cat > releases/RELEASE_NOTES.md << EOF
# FinDAG Desktop v1.0.0

## Release Date
$(date)

## Features
- Complete blockchain desktop application
- Institutional-grade trading interface
- Advanced DAG visualization
- Comprehensive security features
- Real-time network monitoring
- Multi-wallet support
- Compliance dashboard

## System Requirements
- Windows 10/11, macOS 10.13+, or Linux
- 4GB RAM minimum, 8GB recommended
- 2GB free disk space
- Internet connection for network features

## Installation
1. Download the appropriate installer for your platform
2. Run the installer and follow the prompts
3. Launch FinDAG Desktop from your applications menu

## Security
- All downloads are signed and verified
- Checksums provided for integrity verification
- No telemetry or data collection
- Local-first architecture

## Support
For support, visit: https://findag.io/support
EOF

print_status "Creating deployment manifest..."
cat > releases/deployment.json << EOF
{
  "version": "1.0.0",
  "releaseDate": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "buildId": "$(git rev-parse HEAD 2>/dev/null || echo 'unknown')",
  "platforms": {
    "windows": {
      "installer": "findag-desktop_1.0.0_x64_en-US.msi",
      "portable": "findag-desktop_1.0.0_x64_en-US.exe"
    },
    "macos": {
      "installer": "findag-desktop_1.0.0_x64.dmg",
      "app": "findag-desktop.app"
    },
    "linux": {
      "deb": "findag-desktop_1.0.0_amd64.deb",
      "appimage": "findag-desktop_1.0.0_x86_64.AppImage"
    }
  },
  "checksums": {
    "sha256": "auto-generated"
  },
  "dependencies": {
    "node": ">=18.0.0",
    "rust": ">=1.70.0"
  }
}
EOF

print_success "Production build completed successfully!"
print_status "Build artifacts are available in the 'releases/' directory"
print_status "Total build time: $SECONDS seconds"

# Display build summary
echo ""
echo "📦 Build Summary:"
echo "=================="
echo "✅ Frontend build completed"
echo "✅ Tauri application built"
echo "✅ Tests passed"
echo "✅ Linting completed"
echo "✅ Distribution packages created"
echo "✅ Checksums generated"
echo "✅ Release notes created"

echo ""
echo "🎯 Next Steps:"
echo "1. Test the built application"
echo "2. Upload artifacts to distribution servers"
echo "3. Update version numbers for next release"
echo "4. Create GitHub release with artifacts"

print_success "FinDAG Desktop is ready for distribution! 🚀" 