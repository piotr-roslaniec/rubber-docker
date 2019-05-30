#!/bin/bash

./make_images.sh

vagrant suspend
vagrant destroy -f ; vagrant up && vagrant ssh
