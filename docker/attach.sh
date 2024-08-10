#!/bin/bash
IMAGE_NAME="gi"
CONTAINER_NAME="gi-container"

function check_exists {
    if [ -z "$(docker ps -a -q -f name=$CONTAINER_NAME)" ]; then
        return 1
    fi
    return 0
}

check_exists
if [ $? -eq 0 ]; then
    echo "Container $CONTAINER_NAME exists. Attaching..."
    docker start $CONTAINER_NAME
    docker attach $CONTAINER_NAME
else
    echo "Container $CONTAINER_NAME does not exist. Creating a new one..."
    docker run \
        --name $CONTAINER_NAME \
        -v $PWD:/workspace \
        -it \
        -w /workspace \
        --shm-size=4g \
        $IMAGE_NAME:latest
fi