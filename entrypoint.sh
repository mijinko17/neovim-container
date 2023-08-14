#!/usr/bin/env bash

export USER=neovim
export HOME=/home/$USER

uid=$(stat -c "%u" .)
gid=$(stat -c "%g" .)

if [ "$uid" -ne 0 ]; then
  echo Change UID.
  if [ "$(id -g $USER)" -ne $gid ]; then
    getent group $gid >/dev/null 2>&1 || groupmod -g $gid $USER
    chgrp -R $gid $HOME
  fi
  if [ "$(id -u $USER)" -ne $uid ]; then
    usermod -u $uid $USER
  fi
  echo Finish changing UID.
fi

exec setpriv --reuid=$USER --regid=$USER --init-groups "$@"
