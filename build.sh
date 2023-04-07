if [ "$1" = "macos-arm" ]; then
  rustup target add aarch64-apple-darwin
  cargo build --release --target=aarch64-apple-darwin
elif [ "$1" = "linux" ]; then
  rustup target add x86_64-unknown-linux-musl	
  cargo build --release --target=x86_64-unknown-linux-musl	
elif [ "$1" = "windows" ]; then
  rustup target add x86_64-pc-windows-msvc
  cargo build --release --target=x86_64-pc-windows-msvc
fi