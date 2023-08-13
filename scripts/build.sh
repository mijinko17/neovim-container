#!/usr/bin/env bash

cd $(dirname $0)

docker build -t mijinko17/neovim-container:develop --network host ..
