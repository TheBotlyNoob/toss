#!/bin/bash

# General installs
sudo apt update
sudo apt install -y qemu qemu-system build-essential

# Now do stuff branch-specific
curl -fsSL https://sh.rustup.rs | sh -s -- -y
