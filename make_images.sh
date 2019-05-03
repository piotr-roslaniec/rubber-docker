#!/bin/bash

docker build -t ubuntu:19.04-tools images/ubuntu:19.04-tools

docker run --name ubuntu-export ubuntu:19.04-tools
docker export ubuntu-export >"$HOME/rdocker/images/ubuntu.tar"
docker rm ubuntu-export