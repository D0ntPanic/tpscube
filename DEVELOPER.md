# Developer Notes / Build Instructions 

## Dependencies

### Install Rust

- Install [rust](https://www.rust-lang.org/tools/install), for example (on MacOS and Linux):
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- For Windows, use the recommended installer (64-bit recommended, and select default option `1`).

### Windows Dependencies

- [LLVM](https://github.com/llvm/llvm-project/releases/)
- [cmake](https://cmake.org/download/)

Note: Make sure to add both packages to your path and create a new command-prompt after installing them for rust to find the dependencies.  Also, if you don't have any development environment yet, LLVM will require you to install the visual studio build environment which can include cmake for you.

### (Optional) web dependencies:

Optional steps for building and testing web-based builds:

- Install wasm32 and bindgen: 
```sh
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
```

Another optional install for local HTTPS testing required for progressive web app testing:

- MacOS:
```sh
brew install nss mkcert
mkcert -install
mkcert localhost #creates localhost certificate
```

- Ubuntu:
```sh
sudo apt install mkcert
mkcert -install
mkcert localhost #creates localhost certificate
```

## Building

(From the folder where you downloaded or cloned this repository)

- MacOS/Linux:
```sh
make native
./target/release/tpscube
```

- Windows:

```sh
cargo build --release
start target\release\tpscube.exe
```

