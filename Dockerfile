FROM curlimages/curl:8.2.1 as curl
WORKDIR /download
RUN curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim.appimage

FROM node:bookworm-slim
RUN apt-get update
RUN apt-get install git -y
RUN apt-get install curl -y
RUN apt-get install ripgrep -y
RUN apt-get install xz-utils -y
WORKDIR /neovim
COPY --from=curl /download/nvim.appimage .
RUN chmod u+x nvim.appimage
RUN ./nvim.appimage --appimage-extract
RUN ln -s /neovim/squashfs-root/AppRun /usr/bin/nvim
RUN rm ./nvim.appimage
RUN userdel node
RUN useradd mijinko -m -u 1001
USER mijinko
COPY --chown=mijinko .config/ /home/mijinko/.config
RUN nvim --headless -c 'autocmd User PackerComplete quitall'
RUN nvim --headless -c 'MasonInstall lua-language-server shellcheck shfmt' -c qall
USER root
COPY entrypoint.sh /
ENTRYPOINT ["/entrypoint.sh"]

