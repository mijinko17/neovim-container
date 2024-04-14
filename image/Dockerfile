FROM node:bookworm-slim as base

RUN apt-get update && \
    apt-get install git -y && \
    apt-get install curl -y  && \
    apt-get install ripgrep -y && \
    apt-get install xz-utils -y && \
    apt-get clean

WORKDIR /neovim
RUN curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim.appimage && \
    chmod u+x nvim.appimage && \
    ./nvim.appimage --appimage-extract && \
    ln -s /neovim/squashfs-root/AppRun /usr/bin/nvim && \
    rm ./nvim.appimage

ARG user="neovim"
ARG uid
RUN userdel node && \
    useradd --uid $uid -m $user
USER $user

COPY --chown=$user .config/ /home/$user/.config
COPY --chmod=0755 --chown=$user scripts/copy.sh /home/$user/bin/copy.sh
COPY --chmod=0755 --chown=$user scripts/paste.sh /home/$user/bin/paste.sh
ENV PATH $PATH:/home/$user/bin

RUN nvim --headless -c 'Lazy install' -c qall
# RUN nvim --headless -c 'MasonInstall lua-language-server shellcheck shfmt' -c qall
