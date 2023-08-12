#!/usr/bin/env bash

readonly neovim_target_absolute_path_in_host=$(readlink -f "${@:$#:1}")
readonly current_directory_absolute_path_in_host=$(pwd)
readonly container_name=neovim-container
readonly container_name_regex=/neovim-container$
neovim_target_relative_path=''
neovim_target_file=''

function relative_path_from_home_directory() {
  local -r regex="${HOME}/(.+)"
  if [[ $1 =~ $regex ]]; then
    echo ${BASH_REMATCH[1]}
    return 0
  else
    return 1
  fi
}

if ! current_directory_relative_path=$(relative_path_from_home_directory $current_directory_absolute_path_in_host); then
  echo "You are not in home directory."
  exit 1
fi
readonly current_directory_absolute_path_in_containier=/home/host/$current_directory_relative_path

function convert_to_abs_in_container(){
  local -r relative=$1
}

function upgrade(){
  echo 'Delete container.'
  docker rm $container_name
  echo 'Delete old image.'
  docker image pull mijinko17/neovim-container:latest
  echo 'Update launch script'
  curl https://raw.githubusercontent.com/mijinko17/neovim-container/main/nvim.sh > $0
}

function run(){
  local -r workdir=$1
  local -r nvim_opt=$2
  local -r nvim_file=$3
  echo workdir="$workdir"
  echo nvim_opt="$nvim_opt"
  echo nvim_file="$nvim_file"
  echo "${nvim_opt}" -- "$nvim_file"
  if [ "$(docker ps -aq -f name="$container_name_regex")" ]; then
      docker start $container_name &> /dev/null
      docker exec \
      --interactive \
      --tty \
      --workdir $workdir \
      $container_name nvim $nvim_opt -- $nvim_file
      docker stop $container_name &> /dev/null
  else
    docker run \
      --name $container_name \
      --interactive \
      --tty \
      --volume $HOME:/home/host \
      --workdir $workdir \
      --network=host \
      mijinko17/neovim-container:latest nvim $nvim_opt -- $nvim_file
  fi
}

function run_with_relative_path(){
  local -r relative_path=$1
  local -r abs_path=$(readlink -f $relative_path)

  if ! neovim_target_relative_path=$(relative_path_from_home_directory $abs_path); then
    echo "Target file does not exist in home directory."
    exit 1
  fi
  readonly neovim_target_absolute_path_in_container=/home/host/$neovim_target_relative_path
  run $current_directory_absolute_path_in_containier "" $neovim_target_absolute_path_in_container
}

function run_with_relative_path_with_option(){
  local -r relative_path=$1
  local -r option=$2
  local -r abs_path=$(readlink -f $relative_path)

  if ! neovim_target_relative_path=$(relative_path_from_home_directory $abs_path); then
    echo "Target file does not exist in home directory."
    exit 1
  fi
  readonly neovim_target_absolute_path_in_container=/home/host/$neovim_target_relative_path
  run $current_directory_absolute_path_in_containier $option $neovim_target_absolute_path_in_container
}

function run_with_no_option(){
  local -r workdir=$1
  local -r nvim_file=$2
  run "$workdir" "" "$nvim_file"
}

if [ $# -eq 0 ];then
  run $current_directory_absolute_path_in_containier "" ""
elif [ $# -eq 1 ]; then
  case $1 in
    --upgrade ) upgrade;exit ;;
    -?*) neovim_opt=$1; run $current_directory_absolute_path_in_containier $neovim_opt ""; exit;;
    *) run_with_relative_path $1; exit;;
  esac
else
  neovim_opt=''
  while [ $# -gt 0 ]; do
    case $1 in
      --upgrade ) upgrade;exit ;;
      --) shift; neovim_target_relative_path=$1; break;;
      *) neovim_opt="$neovim_opt $1"
    esac
    shift
  done
  run_with_relative_path_with_option $neovim_target_relative_path $neovim_opt
  exit
fi

while [ $# -gt 1 ]; do
  case $1 in
    --upgrade ) upgrade;exit ;;
    --) shift; neovim_target_relative_path=$1; break;;
    *) neovim_opt="$neovim_opt $1"
  esac
  shift
done


#run_with_no_option /home/host/program/neovim-container /home/host/program/neovim-container/Dockerfile

#neovim_opt=''
#
#if [ $# -eq 0 ];then
#  neovim_target_file=''
#fi
#
#while [ $# -gt 1 ]; do
#  case $1 in
#    --upgrade ) upgrade;exit ;;
#    --) shift; neovim_target_relative_path=$1; break;;
#    *) neovim_opt="$neovim_opt $1"
#  esac
#  shift
#done
#
#if ! current_directory_relative_path=$(relative_path_from_home_directory $current_directory_absolute_path_in_host); then
#  echo "You are not in home directory."
#  exit 1
#fi
#readonly current_directory_absolute_path_in_containier=/home/host/$current_directory_relative_path
#
#if ! neovim_target_relative_path=$(relative_path_from_home_directory $neovim_target_absolute_path_in_host); then
#  echo "Target file does not exist in home directory."
#  exit 1
#fi
#readonly neovim_target_absolute_path_in_container=/home/host/$neovim_target_relative_path
#
#neovim_opt="$neovim_opt $neovim_target_absolute_path_in_container"
#
#run $neovim_opt $

#if [ "$(docker ps -aq -f name="$container_name_regex")" ]; then
#    docker start $container_name &> /dev/null
#    docker exec \
#    --interactive \
#    --tty \
#    --workdir $current_directory_absolute_path_in_containier \
#    $container_name nvim $neovim_opt $neovim_target_file
#
#    docker stop $container_name &> /dev/null
#else
#  docker run \
#    --name $container_name \
#    --interactive \
#    --tty \
#    --volume $HOME:/home/host \
#    --workdir $current_directory_absolute_path_in_containier \
#    --network=host \
#    mijinko17/neovim-container:latest nvim $neovim_opt $neovim_target_file
#fi

