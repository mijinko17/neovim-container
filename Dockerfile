# FROM debian:bookworm
FROM curlimages/curl:8.2.1 as curl
WORKDIR /download
RUN curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim.appimage

FROM debian:bookworm
# RUN apt-get update
# RUN apt-get install fuse -y
WORKDIR /neovim
COPY --from=curl /download/nvim.appimage .
RUN chmod u+x nvim.appimage
RUN ./nvim.appimage --appimage-extract
RUN ln -s /neovim/squashfs-root/AppRun /usr/bin/nvim
COPY .config/ /root/.config
