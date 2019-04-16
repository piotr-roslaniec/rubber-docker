#!/bin/bash

docker pull ubuntu:14.04
docker save ubuntu:14.04 > ~/rubber-docker/images/ubuntu.tar