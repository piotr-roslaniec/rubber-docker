#!/bin/bash

if [[ $EUID -ne 0 ]]; then
    echo "You must be root to run this script"
    exit 1
fi

set -x

CPU_SHARES=20 # TODO should be parameter

PID=$(cat container.pid)
CID=$(cat container.cid)
CGROUP_CPU_DIR="/sys/fs/cgroup/cpu/rubber_docker/$CID"
CGROUP_CPU_TASKS="$CGROUP_CPU_DIR/tasks"
CGROUP_CPU_SHARES="$CGROUP_CPU_DIR/cpu.shares"

# Insert the container to new cpu cgroup named 'rubber_docker/container_id'
sudo mkdir -p "$CGROUP_CPU_DIR"
sudo echo "$PID" >"$CGROUP_CPU_TASKS"

# Set the 'cpu.shares' in our cpu cgroup
sudo echo "$CPU_SHARES" >"$CGROUP_CPU_SHARES"
