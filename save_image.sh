#!/bin/bash

mkdir -p images
docker run --name ubuntu-export ubuntu:19.04
docker export ubuntu-export > "$HOME/rdocker/images/ubuntu.tar"
docker rm ubuntu-export
