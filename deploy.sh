#!/bin/bash
components=$1
IFS=',' read -ra arr <<<"$components"
contains_component() {
    local target="$1"
    # If the array is empty, all components should start
    if [ ${#arr[@]} -eq 0 ]; then
        return 0
    fi

    # Loop through the array and check each element
    for i in "${arr[@]}"; do
        if [[ $i == "$target" ]]; then
            return 0
        fi
    done
    return 1
}

# Call the function
if contains_component "backend"; then

    cd ../backend
    # 1. Start a docker-compose file and wait for it to run.
    docker-compose up -d

    # Check if Docker Compose is running successfully
    if [ $? -eq 0 ]; then
        echo "Docker Compose deployed successfully."
        echo "Now waiting for 5 seconds to allow database to fully start"
    else
        echo "Failed to start Docker Compose." >&2
        exit 1
    fi

    # Give Docker some time to start up
    countdown() {
        count=$1
        while [ $count -gt 0 ]; do
            echo -ne "Waiting for $count seconds...\r"
            sleep 1
            let count=count-1
        done
    }
    countdown 5

    # Run migrations
    sqlx migrate run
    if [ $? -eq 0 ]; then
        echo "Migrations completed successfully."
    else
        echo "Failed to run migrations." >&2
        exit 1
    fi

    if [ ! -d "../logs" ]; then
        mkdir -p "../logs"
        echo 'Successfully created logs directory'
    fi
    # 2. Start a backend service with 'cargo watch -q -c -w ../backend/src/ -x run'
    # Run this in the background and redirect both stdout and stderr to a log file.
    cargo watch -q -c -w src/ -x run 2>&1 | tee ../logs/backend.log &
    echo 'backend started'
fi

if contains_component "frontend"; then
    cd ../frontend
    # 3. Start the frontend service with 'trunk serve --port 3000'
    # Run this in the background and redirect both stdout and stderr to a log file.
    trunk serve --port 3000 2>&1 | tee ../logs/frontend.log &
    echo 'frontend started'
fi
