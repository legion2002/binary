#!/bin/bash
# Simple development script to run both backend and frontend
# Usage: ./dev.sh

set -e

# Colors
GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${CYAN}🚀 Starting Binary Markets Development Environment${NC}"
echo ""

# Trap to kill all background processes on exit
trap 'echo -e "\n${YELLOW}Shutting down...${NC}"; kill 0' EXIT

# Start backend
echo -e "${GREEN}Starting backend on http://localhost:3000${NC}"
(cd backend && cargo run --bin backend) &
BACKEND_PID=$!

# Wait a bit for backend to start
sleep 2

# Start frontend
echo -e "${GREEN}Starting frontend on http://localhost:5173${NC}"
(cd frontend && pnpm dev) &
FRONTEND_PID=$!

echo ""
echo -e "${CYAN}========================================${NC}"
echo -e "${GREEN}Backend:${NC}  http://localhost:3000"
echo -e "${GREEN}Frontend:${NC} http://localhost:5173"
echo -e "${CYAN}========================================${NC}"
echo ""
echo -e "${YELLOW}Press Ctrl+C to stop all services${NC}"

# Wait for any process to exit
wait
