if [ "$1" = "macos-arm" ]; then
  rustup target add aarch64-apple-darwin
  cargo build --release --target=aarch64-apple-darwin
elif [ "$1" = "linux" ]; then
  cd /usr/src
  rustup target add x86_64-unknown-linux-gnu
  cargo build --release --target=x86_64-unknown-linux-gnu
elif [ "$1" = "windows" ]; then
  cd /usr/src
  rustup target add x86_64-pc-windows-gnu 
  rustup toolchain install stable-x86_64-pc-windows-gnu 
  cargo build --release --target=x86_64-pc-windows-gnu
fi