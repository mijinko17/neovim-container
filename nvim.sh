#!/usr/bin/env bash

readonly user=neovim
readonly container_name_prefix=neovim-container
readonly image_name_prefix=mijinko17/neovim-container
readonly uid="$(id -u "$(whoami)")"

for_develop=false

if [ "$1" = --develop ]; then
  for_develop=true
  shift
fi

function base_image_tag() {
  if "${for_develop}"; then
    echo develop
  else
    echo latest
  fi
}

function local_image_tag() {
  if "${for_develop}"; then
    echo local_develop
  else
    echo local
  fi
}

function base_image_name() {
  echo $image_name_prefix:"$(base_image_tag)"
}

function local_image_name() {
  echo $image_name_prefix:"$(local_image_tag)"
}

function container_name() {
  if "${for_develop}"; then
    echo "$container_name_prefix"-develop
  else
    echo "$container_name_prefix"
  fi
}

function image_name_with_uid() {
  echo "${image_name_prefix}-uid-${uid}:$(base_image_tag)"
}

function upgrade() {
  echo 'Delete local image.'
  docker image rm "$(local_image_name)"
  echo 'Update base image.'
  docker image pull "$(base_image_name)"
  echo 'Update launch script'
  curl https://raw.githubusercontent.com/mijinko17/neovim-container/main/nvim.sh >$0
}

function build_local_image_if_need() {
  if [ ! "$(docker image ls -q $(local_image_name))" ]; then
    echo 'Local image does not exist. Build image.'
    local -r uid=$(id -u $(whoami))
    local -r gid=$(id -u $(whoami))

    docker build -t $(local_image_name) - <<EOF
    FROM $(base_image_name)
    USER root
    RUN groupmod -g $gid $user
    RUN chgrp -R $gid /home/$user
    RUN usermod -u $uid $user
    USER $user
EOF
  fi
}

function run() {
  #build_local_image_if_need

  local -r workdir=$1
  local -r nvim_opt=$2
  local -r nvim_file=$3
  #local -r name=$(container_name)

  docker run \
    --rm \
    --interactive \
    --tty \
    --volume $HOME:/home/host \
    --volume $HOME/.gitconfig:/home/$user/.gitconfig \
    --volume $HOME/.ssh:/home/$user/.ssh \
    --workdir $workdir \
    --network=host \
    $(image_name_with_uid) nvim $nvim_opt -- $nvim_file
}

function run_temp_for_single_file() {
  build_local_image_if_need

  local -r nvim_opt=$1
  local -r nvim_file=$2
  local -r file_name=$(basename "$nvim_file")
  local -r name=$(container_name)

  docker run \
    --rm \
    --name "$name" \
    --interactive \
    --tty \
    --volume $nvim_file:/home/host/$file_name \
    --workdir /home/host \
    --network=host \
    $(local_image_name) nvim $nvim_opt -- $file_name
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
