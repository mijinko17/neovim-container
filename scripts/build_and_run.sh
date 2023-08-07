#!/usr/bin/env bash

readonly current_directory=$(pwd)

cd `dirname $0`
nvim_sh_path=$(readlink -f ./run.sh)
./build.sh

cd $current_directory
$nvim_sh_path $1