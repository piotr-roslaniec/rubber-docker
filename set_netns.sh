#!/bin/bash

if [[ $EUID -ne 0 ]]; then
    echo "You must be root to run this script"
    exit 1
fi

set -x

PID=$(cat container.pid)
NS=$(cat container.cid)
VETH="veth0"
VPEER="veth1"
VETH_ADDR="10.1.1.1"
VPEER_ADDR="10.1.1.2"

mkdir -p /var/run/netns

# Link netns file descriptor to access it from host
ln -s "/proc/$PID/ns/net" "/var/run/netns/$NS"

# Create a veth link (a bridge device)
ip link add "$VETH" type veth peer name "$VPEER" netns "$NS"

# Setup IP address of "$VETH"
ip addr add "$VETH_ADDR/24" dev "$VETH"
ip link set "$VETH" up

# Setup IP address of "$VPEER"
ip netns exec "$NS" ip addr add "$VPEER_ADDR/24" dev "$VPEER"
ip netns exec "$NS" ip link set "$VPEER" up
ip netns exec "$NS" ip link set dev lo up

# Make external traffic leaving $NS go through $VETHt
ip netns exec "$NS" ip route add default via "$VETH_ADDR"

# Share internet access between host and NS

# Enable IP-forwarding
sysctl -w net.ipv4.ip_forward=1

# Flush forward rules, policy DROP by default.
iptables -P FORWARD DROP
iptables -F FORWARD

# Flush NAT rules
iptables -t nat -F

# Enable masquerading of 10.1.1.0
iptables -t nat -A POSTROUTING -s "$VETH_ADDR/24" -j MASQUERADE

# Allow forwarding between eth0 and v-eth1
iptables -A FORWARD -i eth0 -o "$VETH" -j ACCEPT
iptables -A FORWARD -o eth0 -i "$VETH" -j ACCEPT
