
echo "======================================="
cd frontend
echo "Building Frontend"
npm run build
echo "Frontend Build Success!"
cd ..
echo "======================================="
echo "Building Backend"
cargo build
echo "Backend Build Success!"
echo "======================================="
echo "Continue with host script.."
echo "======================================="

