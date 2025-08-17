#!/bin/bash

# Clipstream Setup Script
# This script helps set up the development environment with subdomain architecture

echo "🎮 Clipstream Setup Script"
echo "=========================="
echo "🌐 Subdomain Architecture:"
echo "   • API: api.clipsstream.com → port 8000"
echo "   • Web: clipsstream.com → port 3000"
echo ""

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

# Check if Docker Compose is installed
if ! command -v docker compose &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first."
    echo "   Visit: https://rustup.rs/"
    exit 1
fi

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js 18+ first."
    exit 1
fi

echo "✅ All prerequisites are installed!"
echo ""

# Ask user what they want to set up
echo "What would you like to set up?"
echo "1) Development environment (local)"
echo "2) Production environment (Docker)"
echo "3) Just build the API"
echo "4) Just set up the frontend"
read -p "Enter your choice (1-4): " choice

case $choice in
    1)
        echo "🔧 Setting up development environment..."
        
        # Set up backend
        echo "📦 Setting up Rust backend..."
        cd api/clipstream-api
        
        # Start database
        docker compose up -d postgres
        sleep 5
        
        echo "✅ Database started. You can now run:"
        echo "   cd api/clipstream-api && cargo run"
        echo "   → API will be available at http://localhost:8000"
        echo ""
        
        # Set up frontend
        echo "📦 Setting up Next.js frontend..."
        cd ../../web/clipstream
        
        if [ ! -d "node_modules" ]; then
            npm install
        fi
        
        echo "✅ Frontend set up. You can now run:"
        echo "   cd web/clipstream && npm run dev"
        echo "   → Frontend will be available at http://localhost:3000"
        echo ""
        
        echo "🚀 Development environment ready!"
        echo ""
        echo "📋 Next steps:"
        echo "1. Configure your /etc/hosts file:"
        echo "   127.0.0.1 api.clipsstream.com"
        echo "   127.0.0.1 clipsstream.com"
        echo ""
        echo "2. Set up nginx to route subdomains:"
        echo "   - api.clipsstream.com → http://127.0.0.1:8000"
        echo "   - clipsstream.com → http://127.0.0.1:3000"
        ;;
        
    2)
        echo "🐳 Setting up production environment..."
        cd api/clipstream-api
        docker compose up -d
        
        echo "✅ Backend services started!"
        echo ""
        
        cd ../../web/clipstream
        if [ ! -d "node_modules" ]; then
            npm install
        fi
        
        npm run build
        
        echo "✅ Frontend built!"
        echo "🚀 Production environment ready!"
        echo ""
        echo "📋 Configure your reverse proxy to route:"
        echo "   - api.clipsstream.com → port 8000"
        echo "   - clipsstream.com → port 3000"
        ;;
        
    3)
        echo "🦀 Building Rust API..."
        cd api/clipstream-api
        cargo build --release
        echo "✅ API built successfully!"
        echo "   Binary: api/clipstream-api/target/release/clipstream-api"
        echo "   Run with: ./target/release/clipstream-api"
        echo "   Available at: http://localhost:8000"
        ;;
        
    4)
        echo "⚛️ Setting up Next.js frontend..."
        cd web/clipstream
        npm install
        echo "✅ Frontend dependencies installed!"
        echo "   Run 'npm run dev' to start development server"
        echo "   Available at: http://localhost:3000"
        ;;
        
    *)
        echo "❌ Invalid choice. Exiting."
        exit 1
        ;;
esac

echo ""
echo "📚 Additional setup:"
echo "1. Copy example environment files:"
echo "   - api/clipstream-api/.env.example → .env"
echo "   - web/clipstream/.env.local.example → .env.local"
echo ""
echo "2. Update your environment variables:"
echo "   - Backend: DATABASE_URL, JWT_SECRET, etc."
echo "   - Frontend: NEXT_PUBLIC_API_BASE_URL=https://api.clipsstream.com"
echo ""
echo "3. For production, configure DNS:"
echo "   - api.clipsstream.com A YOUR_SERVER_IP"
echo "   - clipsstream.com A YOUR_SERVER_IP"
echo ""
echo "4. See api/README.md for detailed API documentation"
echo ""
echo "Happy coding with subdomain architecture! 🎮✨"
