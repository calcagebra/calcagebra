#!/bin/bash
set -eux

apt-get update

PKG="upx mold"

apt-get install -y $PKG