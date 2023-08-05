#!/usr/bin/env bash

absolute_path=$(readlink -f $1)
echo $absolute_path

regex="${HOME}/(.+)"

if [[  $absolute_path =~ $regex ]]; then
  neovim_target="/home/host/${BASH_REMATCH[1]}"
  docker run --rm --name neovim-container -it -v $HOME:/home/host neovim-container nvim $neovim_target
else
  echo "not matched"
fi
