#!/usr/bin/env bash

readonly neovim_target_absolute_path_in_host=$(readlink -f $1)
readonly current_directory_absolute_path_in_host=$(pwd)

function relative_path_from_home_directory() {
  readonly regex="${HOME}/(.+)"
  if [[ $1 =~ $regex ]]; then
    echo ${BASH_REMATCH[1]}
    return 0
  else
    return 1
  fi
}

if ! readonly current_directory_relative_path=$(relative_path_from_home_directory $current_directory_absolute_path_in_host); then
  echo "You are not in home directory."
  exit 1
fi
readonly current_directory_absolute_path_in_containier=/home/host/$current_directory_relative_path

if ! readonly neovim_target_relative_path=$(relative_path_from_home_directory $neovim_target_absolute_path_in_host); then
  echo "Target file does not exist in home directory."
  exit 1
fi
readonly neovim_target_absolute_path_in_container=/home/host/$neovim_target_relative_path

docker run \
  --rm \
  --name neovim-container \
  --interactive \
  --tty \
  --volume $HOME:/home/host \
  --workdir $current_directory_absolute_path_in_containier \
  --network=host \
  mijinko17/neovim-container:develop nvim $neovim_target_absolute_path_in_container