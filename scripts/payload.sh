#!/bin/bash

sudo mount -o remount,rw /

echo "pi:raspberry" | sudo chpasswd
sync

sudo mount -o remount,ro /
