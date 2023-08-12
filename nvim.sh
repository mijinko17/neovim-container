#!/usr/bin/env bash

readonly container_name=neovim-container
readonly container_name_regex=/neovim-container$

function relative_path_from_home_directory() {
  local -r regex="${HOME}/(.+)"
  if [[ $1 =~ $regex ]]; then
    echo "${BASH_REMATCH[1]}"
    return 0
  else
    return 1
  fi
}

if ! current_directory_relative_path=$(relative_path_from_home_directory "$(pwd)"); then
  echo "You are not in home directory."
  exit 1
fi
readonly current_directory_absolute_path_in_containier=/home/host/$current_directory_relative_path

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

function run_with_file(){
  local -r option=$1
  local -r relative_path=$2
  local -r abs_path=$(readlink -f "$relative_path")

  if ! neovim_target_relative_path=$(relative_path_from_home_directory "$abs_path"); then
    echo "Target file does not exist in home directory."
    exit 1
  fi
  readonly neovim_target_absolute_path_in_container=/home/host/$neovim_target_relative_path
  run $current_directory_absolute_path_in_containier "$option" "$neovim_target_absolute_path_in_container"
}

function run_with_no_file(){
  local -r option=$1
  run $current_directory_absolute_path_in_containier "$option" ""
}

function run_with_no_option_no_file(){
  run_with_no_file ""
}

function run_with_no_option_with_file(){
  local -r relative_path=$1
  run_with_file "" "$relative_path"
}

function run_with_option_with_no_file(){
  local -r option=$1
  run_with_no_file "$option"
  #run $current_directory_absolute_path_in_containier "$option" ""
}

function run_with_option_with_file(){
  local -r option=$1
  local -r relative_path=$2
  run_with_file "$option" "$relative_path"
}

if [ $# -eq 0 ];then
  run_with_no_option_no_file
  exit
elif [ $# -eq 1 ]; then
  case $1 in
    --upgrade ) upgrade; exit ;;
    -?*) neovim_opt=$1; run_with_option_with_no_file "$neovim_opt"; exit;;
    *) file_path=$1; run_with_no_option_with_file "$file_path"; exit;;
  esac
else
  neovim_opt=''
  while [ $# -gt 0 ]; do
    case $1 in
      --upgrade ) upgrade; exit ;;
      --) shift; file_path=$1; break;;
      *) neovim_opt="$neovim_opt $1"
    esac
    shift
  done
  if [ "$file_path" = '' ];then
    run_with_option_with_no_file "$neovim_opt"
    exit
  else
    run_with_option_with_file "$neovim_opt" "$file_path"
    exit
  fi
fi

