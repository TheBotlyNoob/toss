#!/bin/bash

# General installs
sudo apt update
sudo apt install -y qemu qemu-system build-essential

# Now do stuff branch-specific
sudo apt install -y nasm
