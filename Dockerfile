FROM node:bookworm-slim as base
RUN apt-get update
RUN apt-get install git -y
RUN apt-get install curl -y
RUN apt-get install ripgrep -y
RUN apt-get install xz-utils -y
WORKDIR /neovim
RUN curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim.appimage
RUN chmod u+x nvim.appimage
RUN ./nvim.appimage --appimage-extract
RUN ln -s /neovim/squashfs-root/AppRun /usr/bin/nvim
RUN rm ./nvim.appimage
ARG user="neovim"
RUN userdel node
RUN useradd -m $user
USER $user
COPY --chown=$user .config/ /home/$user/.config
RUN nvim --headless -c 'autocmd User PackerComplete quitall'
RUN nvim --headless -c 'MasonInstall lua-language-server shellcheck shfmt' -c qall

