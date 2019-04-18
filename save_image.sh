#!/bin/bash

mkdir -p images
docker pull busybox
docker save busybox > images/busybox.tar