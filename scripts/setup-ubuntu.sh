#!/usr/bin/env bash
set -e

sudo apt update
sudo apt install -y \
    libgtk-4-dev \
    libpango1.0-dev \
    libadwaita-1-dev \
    libsqlite3-dev \
    pkg-config
