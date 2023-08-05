#!/usr/bin/env bash

readonly absolute_path=$(readlink -f $1)
readonly regex="${HOME}/(.+)"

if [[ $absolute_path =~ $regex ]]; then
  readonly neovim_target="/home/host/${BASH_REMATCH[1]}"
  docker run --rm --name neovim-container -it -v $HOME:/home/host --network=host mijinko17/neovim-container:latest nvim $neovim_target
else
  echo "not matched"
fi
