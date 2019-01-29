#!/bin/bash

docker image build \
    --build-arg VCS_REF=`git rev-parse --short HEAD` \
    --build-arg BUILD_DATE=`date -u +"%Y-%m-%dT%H:%M:%SZ"` \
    --build-arg BINARY_NAME="adder" \
    -t sieciech/rust-test .
