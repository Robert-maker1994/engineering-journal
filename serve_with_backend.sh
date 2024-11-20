#!/bin/bash

# Start your backend process
echo "Starting backend..."
# Replace this with the command to start your backend
# For example, if your backend is a Rust application:
cargo run --bin mdbook-backend &

# Capture the PID of the backend process
BACKEND_PID=$!

# Wait for a few seconds to ensure the backend is up and running
sleep 5

# Run the mdbook serve command
echo "Starting mdBook..."
mdbook serve

# When mdbook serve is terminated, kill the backend process
kill $BACKEND_PID