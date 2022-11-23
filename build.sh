#!/bin/bash

set -eu

# You need to start this script in the folder with CARGO.toml file for this to work.
# Reason: BUILD_PATH is relative to the project root.
USER='pie'
HOST_NAME='ledMatrixPi'
TARGET_FOLDER='/home/pie/ledMatrix/'
BUILD_PATH='./target/aarch64-unknown-linux-gnu/debug/led_matrix'
# Make sure podman is used over docker. if system has docker installed.
# Docker requires sudo rights which are not easy to pass to cross.
export CROSS_CONTAINER_ENGINE=podman
cross build --target aarch64-unknown-linux-gnu

rsync "$BUILD_PATH" "${USER}@${HOST_NAME}:${TARGET_FOLDER}" 

