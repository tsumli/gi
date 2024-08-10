#!/bin/bash
NAME="gi"
docker build \
    -t $NAME:latest \
    --file docker/Dockerfile \
    --network host \
    --build-arg USER=$(id -un) \
    .