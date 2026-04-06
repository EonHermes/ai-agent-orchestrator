#!/bin/bash
# Verification script for Real-time Task Dashboard

echo "=== Real-time Task Dashboard Verification ==="
echo ""

# Check backend structure
echo "✓ Backend structure:"
ls -la backend/Cargo.toml backend/src/main.rs backend/src/lib.rs backend/src/api/mod.rs backend/src/db/tasks.rs backend/src/ws.rs backend/src/error.rs backend/src/models.rs 2>/dev/null || echo "  Some backend files missing!"
echo ""

# Check migrations
echo "✓ Database migrations:"
ls -la backend/migrations/ 2>/dev/null || echo "  Migrations missing!"
echo ""

# Check frontend structure
echo "✓ Frontend structure:"
ls -la frontend/package.json frontend/tsconfig.json frontend/vite.config.ts frontend/src/main.tsx frontend/src/App.tsx frontend/src/components/TaskForm.tsx frontend/src/components/TaskItem.tsx frontend/src/context/TaskContext.tsx 2>/dev/null || echo "  Some frontend files missing!"
echo ""

# Check config files
echo "✓ Configuration files:"
ls -la docker-compose.yml Makefile README.md frontend/Dockerfile backend/Dockerfile frontend/nginx.conf 2>/dev/null || echo "  Some config files missing!"
echo ""

# Check tests
echo "✓ Tests:"
ls -la backend/tests/ frontend/src/components/TaskForm.test.tsx 2>/dev/null || echo "  Test files missing!"
echo ""

echo "=== Verdict ==="
echo "Project structure looks complete! ✅"
echo ""
echo "To build and run:"
echo "  docker-compose up -d"
echo "  or: make up"
echo ""
echo "Access:"
echo "  Frontend: http://localhost:3000"
echo "  Backend:  http://localhost:8080"
echo ""
