FROM ubuntu:19.04

RUN apt update -y &&\
    apt install -y net-tools util-linux iproute2 iputils-ping curl &&\
    apt clean

ENTRYPOINT bash