FROM node:bookworm-slim as base

RUN apt-get update && \
    apt-get install git -y && \
    apt-get install curl -y  && \
    apt-get install ripgrep -y && \
    apt-get install xz-utils -y

WORKDIR /neovim
RUN curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim.appimage && \
    chmod u+x nvim.appimage && \
    ./nvim.appimage --appimage-extract && \
    ln -s /neovim/squashfs-root/AppRun /usr/bin/nvim && \
    rm ./nvim.appimage

ARG user="neovim"
RUN userdel node && \
    useradd -m $user
USER $user

COPY --chown=$user .config/ /home/$user/.config
RUN nvim --headless -c 'autocmd User PackerComplete quitall'
RUN nvim --headless -c 'MasonInstall lua-language-server shellcheck shfmt' -c qall

