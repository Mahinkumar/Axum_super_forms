#!/bin/bash

echo "======================================="
cd frontend
echo "Building Frontend"
npm run build
echo "Frontend Build Success!"
cd ..
echo "======================================="
echo "Building Backend"
cargo build --release
echo "Backend Build Success!"
echo "======================================="
read -p "Start the Backend Server ? (y/n) : " yn
echo "======================================="

case $yn in 
	y|Y ) echo Starting Backend server;;
	n|N ) echo exiting...;
		exit;;
	* ) echo invalid response;
		exit 1;;
esac

cargo run --release
