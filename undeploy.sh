#!/bin/bash


# Kill the 'cargo watch' process
pkill -f 'cargo watch -q -c -w ../backend/src/ -x run'
echo 'Killed backend successfully'


# Kill the 'trunk serve' process
pkill -f 'trunk serve --port 3000'
echo 'Killed frontend successfully'

# Stop Docker Compose
cd ../backend
docker-compose down
echo 'Killed database successfully'