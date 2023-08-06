#!/usr/bin/env bash

readonly absolute_path=$(readlink -f $1)
readonly absolute_current_dir=$(pwd)
readonly regex="${HOME}/(.+)"



if [[ $absolute_path =~ $regex ]]; then
  readonly neovim_target="/home/host/${BASH_REMATCH[1]}"
  if [[ $absolute_current_dir =~ $regex ]]; then
    readonly absolute_current_dir_in_container="/home/host/${BASH_REMATCH[1]}"
    docker run --rm --name neovim-container -it -v $HOME:/home/host -w /home/host --network=host mijinko17/neovim-container:latest nvim $neovim_target
  else
  echo "not matched"
fi
else
  echo "not matched"
fi
