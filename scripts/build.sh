#!/usr/bin/env bash

cd $(dirname $0)

uid=$(id -u "$(whoami)")
docker build -t mijinko17/neovim-container-uid-"$uid":develop --network host --build-arg uid="$uid" ..
