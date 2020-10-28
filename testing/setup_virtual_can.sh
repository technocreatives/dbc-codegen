#! /usr/bin/env bash
set -exu

if [ $(id -u) -ne 0 ]
  then echo "Please run as root"
  exit
fi

ip link add dev vcan0 type vcan
ip link set up vcan0
echo "Done"
