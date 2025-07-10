# FinDAG Desktop - Developer Onboarding Guide (Updated)

## Welcome to FinDAG Desktop Development! 🚀

This guide will help you set up your development environment and get started with the FinDAG Desktop project.

## Project Overview

FinDAG Desktop is a comprehensive, institutional-grade blockchain desktop application built with:
- **Frontend**: React 18 + TypeScript + Tailwind CSS
- **Backend**: Rust + Tauri
- **Database**: Sled (embedded database)
- **Networking**: libp2p for P2P communication

## Prerequisites

### Required Software
- **Node.js**: Version 18.0.0 or higher
- **Rust**: Version 1.70.0 or higher
- **Git**: Version 2.30.0 or higher
- **IDE**: VS Code (recommended) or your preferred editor

### System Requirements
- **RAM**: 8GB minimum, 16GB recommended
- **Storage**: 10GB free space
- **OS**: Windows 10/11, macOS 10.13+, or Linux

## Quick Start

### 1. Clone the Repository
```bash
git clone https://github.com/findag/findag-tauri.git
cd findag-tauri
```

### 2. Install Dependencies
```bash
# Install Node.js dependencies
npm ci

# Install Rust dependencies (automatic on first build)
cargo build
```

### 3. Start Development Server
```bash
# Start the development server
npm run tauri:dev
```

## Development Environment Setup

### Frontend Development

#### Project Structure
```
src/
├── components/          # React components
│   ├── Common/         # Reusable UI components
│   ├── Layout/         # Layout components
│   ├── Dashboard/      # Dashboard components
│   ├── Trading/        # Trading interface
│   ├── DAG/            # DAG visualization
│   ├── Wallet/         # Wallet management
│   ├── Network/        # Network monitoring
│   ├── Security/       # Authentication
│   ├── Compliance/     # Compliance features
│   ├── Charts/         # Charting components
│   └── Monitoring/     # Monitoring components
├── pages/              # Page components
├── contexts/           # React contexts
├── services/           # API services
├── utils/              # Utility functions
├── types/              # TypeScript types
└── styles/             # Global styles
```

#### Available Scripts
```bash
# Development
npm run dev              # Start Vite dev server
npm run tauri:dev        # Start Tauri development

# Building
npm run build            # Build frontend
npm run tauri:build      # Build Tauri application

# Testing
npm run test             # Run tests
npm run test:watch       # Run tests in watch mode
npm run test:coverage    # Run tests with coverage

# Linting
npm run lint             # Run ESLint
npm run lint:fix         # Fix linting issues

# Production
npm run build:production # Full production build
```

### Backend Development

#### Rust Crate Structure
```
src-tauri/
├── findag-core/         # Core blockchain types
├── findag-types/        # Common type definitions
├── findag-consensus/    # Consensus engine
├── findag-network/      # P2P networking
├── findag-storage/      # Storage engine
├── findag-security/     # Security services
├── findag-api/          # HTTP API server
├── findag-dagtimer/     # Time management
└── main.rs              # Application entry point
```

#### Available Commands
```bash
# Development
cargo build              # Build Rust crates
cargo test               # Run Rust tests
cargo clippy             # Run linter

# Documentation
cargo doc                # Generate documentation
cargo doc --open         # Open documentation in browser

# Clean
cargo clean              # Clean build artifacts
```

## Development Workflow

### 1. Feature Development

#### Creating a New Feature
1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Implement the feature**:
   - Add frontend components in `src/components/`
   - Add backend logic in `src-tauri/`
   - Add tests for both frontend and backend
   - Update documentation

3. **Test your changes**:
   ```bash
   npm run test:ci       # Run all tests
   npm run lint          # Check code quality
   ```

4. **Submit a pull request**:
   - Create a detailed PR description
   - Include screenshots for UI changes
   - Ensure all tests pass

### 2. Code Quality Standards

#### Frontend (React + TypeScript)
- **TypeScript**: Use strict mode, avoid `any` types
- **Components**: Functional components with hooks
- **Styling**: Tailwind CSS for styling
- **State Management**: Zustand for global state
- **Testing**: React Testing Library for component tests

#### Backend (Rust)
- **Code Style**: Follow Rust conventions
- **Error Handling**: Use proper error types
- **Documentation**: Document all public APIs
- **Testing**: Unit tests for all functions
- **Performance**: Optimize for performance

### 3. Testing Strategy

#### Frontend Testing
```bash
# Unit tests
npm run test

# Integration tests
npm run test:integration

# E2E tests
npm run test:e2e
```

#### Backend Testing
```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Performance tests
cargo bench
```

### 4. Code Review Process

#### Before Submitting
- [ ] All tests pass
- [ ] Code follows style guidelines
- [ ] Documentation is updated
- [ ] No console.log statements
- [ ] No sensitive data in code

#### Review Checklist
- [ ] Code is readable and well-documented
- [ ] Performance considerations are addressed
- [ ] Security implications are considered
- [ ] Error handling is appropriate
- [ ] Tests cover the new functionality

## Key Features Overview

### 1. Dashboard
- **Location**: `src/components/Dashboard/`
- **Purpose**: Main application overview
- **Key Components**: Performance metrics, quick actions, system status

### 2. Trading Interface
- **Location**: `src/components/Trading/`
- **Purpose**: Cryptocurrency trading functionality
- **Key Components**: Order book, charts, order forms

### 3. DAG Explorer
- **Location**: `src/components/DAG/`
- **Purpose**: Blockchain visualization
- **Key Components**: Interactive DAG graph, block details

### 4. Wallet Management
- **Location**: `src/components/Wallet/`
- **Purpose**: Cryptocurrency wallet operations
- **Key Components**: Wallet creation, transaction history

### 5. Network Monitoring
- **Location**: `src/components/Network/`
- **Purpose**: P2P network management
- **Key Components**: Peer list, connection status

### 6. Security & Compliance
- **Location**: `src/components/Security/` and `src/components/Compliance/`
- **Purpose**: Authentication and regulatory compliance
- **Key Components**: Login forms, audit logs, compliance dashboard

## API Reference

### Frontend API Services
```typescript
// Node operations
import { nodeAPI } from '@/services/api';
await nodeAPI.getStatus();
await nodeAPI.startNode();

// Wallet operations
import { walletAPI } from '@/services/api';
await walletAPI.createWallet();
await walletAPI.sendTransaction();

// Trading operations
import { tradingAPI } from '@/services/api';
await tradingAPI.getMarketData();
await tradingAPI.placeOrder();
```

### Backend Tauri Commands
```rust
// Node management
#[tauri::command]
pub async fn start_findag_node() -> Result<(), String>

// Wallet operations
#[tauri::command]
pub async fn create_wallet() -> Result<Wallet, String>

// Trading operations
#[tauri::command]
pub async fn place_order(order: Order) -> Result<Order, String>
```

## Debugging Guide

### Frontend Debugging
```bash
# Enable React DevTools
npm run dev

# Debug in browser
# Open DevTools and check Console, Network, and React tabs
```

### Backend Debugging
```bash
# Run with debug logging
RUST_LOG=debug npm run tauri:dev

# Use Rust debugger
rust-gdb target/debug/findag-desktop
```

### Common Issues

#### Build Issues
```bash
# Clean and rebuild
npm run clean
npm ci
cargo clean
cargo build
```

#### Runtime Issues
```bash
# Check logs
tail -f ~/.findag/logs/application.log

# Reset application data
rm -rf ~/.findag/data/
```

## Performance Optimization

### Frontend Optimization
- **Code Splitting**: Use dynamic imports
- **Memoization**: Use React.memo and useMemo
- **Virtual Lists**: For large datasets
- **Lazy Loading**: For non-critical components

### Backend Optimization
- **Async Operations**: Use Tokio for concurrency
- **Memory Management**: Proper resource cleanup
- **Database Optimization**: Efficient queries and indexing
- **Network Optimization**: Connection pooling and caching

## Security Guidelines

### Frontend Security
- **Input Validation**: Validate all user inputs
- **XSS Prevention**: Sanitize user content
- **CSRF Protection**: Use proper tokens
- **Secure Storage**: Encrypt sensitive data

### Backend Security
- **Authentication**: JWT-based authentication
- **Authorization**: Role-based access control
- **Data Encryption**: Encrypt sensitive data
- **Audit Logging**: Log all security events

## Deployment

### Development Deployment
```bash
# Build for development
npm run build
npm run tauri:build
```

### Production Deployment
```bash
# Full production build
npm run build:production

# Create release
npm run release
```

## Contributing

### Getting Help
- **Documentation**: Check the docs folder
- **Issues**: Create GitHub issues for bugs
- **Discussions**: Use GitHub Discussions for questions
- **Team Chat**: Join our development team chat

### Code of Conduct
- Be respectful and inclusive
- Follow the project's coding standards
- Test your changes thoroughly
- Document your work

## Conclusion

You're now ready to contribute to FinDAG Desktop! The project is well-structured, thoroughly tested, and production-ready. Follow the guidelines in this document to ensure high-quality contributions.

**Happy coding! 🚀**

For additional support, contact the development team at team@findag.io. 