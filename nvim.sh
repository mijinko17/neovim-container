#!/usr/bin/env bash

readonly container_name_prefix=neovim-container
for_develop=false

if [ "$1" = --develop ]; then
  for_develop=true
  shift
fi

function container_tag() {
  if "${for_develop}"; then
    echo develop
  else
    echo latest
  fi
}

function image_name() {
  echo mijinko17/neovim-container:"$(container_tag)"
}

function container_name() {
  if "${for_develop}"; then
    echo "$container_name_prefix"-develop
  else
    echo "$container_name_prefix"
  fi
}

function container_temp_name() {
  if "${for_develop}"; then
    echo "$container_name_prefix"-temp-develop
  else
    echo "$container_name_prefix"-temp
  fi
}

function upgrade() {
  echo 'Delete container.'
  docker rm "$(container_name)"
  echo 'Delete old image.'
  docker image pull mijinko17/neovim-container:latest
  echo 'Update launch script'
  curl https://raw.githubusercontent.com/mijinko17/neovim-container/main/nvim.sh >$0
}

function run() {
  local -r workdir=$1
  local -r nvim_opt=$2
  local -r nvim_file=$3
  local -r name=$(container_name)
  local -r name_regex=/$(container_name)$

  if [ "$(docker ps -aq -f name="$name_regex")" ]; then
    docker start $name &>/dev/null
    docker exec \
      --interactive \
      --tty \
      --user neovim \
      --workdir $workdir \
      "$name" nvim $nvim_opt -- $nvim_file
    docker stop $name &>/dev/null
  else
    docker run \
      --name $name \
      --interactive \
      --tty \
      --volume $HOME:/home/host \
      --workdir $workdir \
      --network=host \
      "$(image_name)" nvim $nvim_opt -- $nvim_file
  fi
}

function run_temp_for_single_file() {
  local -r nvim_opt=$1
  local -r nvim_file=$2
  local -r file_name=$(basename "$nvim_file")
  local -r name=$(container_temp_name)

  docker run \
    --rm \
    --name "$name" \
    --interactive \
    --tty \
    --volume $nvim_file:/home/host/$file_name \
    --workdir /home/host \
    --network=host \
    mijinko17/neovim-container:latest nvim $nvim_opt -- $file_name
}

function relative_path_from_home_directory() {
  local -r regex="${HOME}/(.+)|${HOME}"
  if [[ $1 =~ $regex ]]; then
    echo "${BASH_REMATCH[1]}"
    return 0
  else
    return 1
  fi
}

neovim_opt=''
file_path=''

if [ $# -eq 0 ]; then
  neovim_opt=''
  file_path=''
elif [ $# -eq 1 ]; then
  case $1 in
  --upgrade)
    upgrade
    exit
    ;;
  -?*) neovim_opt=$1 ;;
  *) file_path=$1 ;;
  esac
else
  while [ $# -gt 0 ]; do
    case $1 in
    --upgrade)
      upgrade
      exit
      ;;
    --)
      shift
      file_path=$1
      break
      ;;
    *) neovim_opt="$neovim_opt $1" ;;
    esac
    shift
  done
fi

absolute_file_path=$(readlink -f "$file_path")

if ! current_directory_relative_path=$(relative_path_from_home_directory "$(pwd)"); then
  echo "You are not in home directory."
  run_temp_for_single_file "$neovim_opt" "$absolute_file_path"
  exit
fi
readonly current_directory_absolute_path_in_containier=/home/host/$current_directory_relative_path

if [ "$file_path" != '' ]; then
  abs_path=$(readlink -f "$file_path")
  if ! neovim_target_relative_path=$(relative_path_from_home_directory "$abs_path"); then
    echo "Target file does not exist in home directory."
    run_temp_for_single_file "$neovim_opt" "$absolute_file_path"
    exit
  fi
  readonly neovim_target_absolute_path_in_container=/home/host/$neovim_target_relative_path
fi

run "$current_directory_absolute_path_in_containier" "$neovim_opt" "$neovim_target_absolute_path_in_container"
exit
