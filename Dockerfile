# FROM alpine:latest

# RUN apk update
# RUN apk upgrade
# RUN apk add curl file git
# RUN apk add gcc
# RUN apk add rust-doc
# # RUN apk search rust-std
# # RUN apk add rust-std

# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -y | sh 
# # RUN source $HOME/.cargo/env



# CMD [""]

FROM rust:latest

RUN apt upgrade
RUN apt update
# RUN apt install -y \
#     clang \
#     gcc \
#     g++ \
#     zlib1g-dev \
#     libmpc-dev \
#     libmpfr-dev \
#     libgmp-dev

CMD [""]
