#!/usr/bin/env bash

set -exu


sudo rsync -aP ./target/x86_64-unknown-linux-musl/release/xdxd-snapshot-rotator /root/xdxd-snapshot-rotator/xdxd-snapshot-rotator
