FROM rust:latest 
 
RUN apt update && apt upgrade -y 
RUN apt install -y g++-mingw-w64-x86-64

WORKDIR /usr/src

CMD ["sh", "./build/build.sh", "windows"]