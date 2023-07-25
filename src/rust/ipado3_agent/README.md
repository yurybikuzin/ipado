### https://stackoverflow.com/questions/31492799/cross-compile-a-rust-application-from-linux-to-windows

`rustup target add x86_64-pc-windows-gnu`
`cargo build --target x86_64-pc-windows-gnu --release -p maskt_agent`

### failed to run custom build command for `openssl-sys v0.9.75`

#### https://www.reddit.com/r/rust/comments/uwjeoc/cross_compile_openssl_to_windows/

Many crates support openssl and *rustls, the second on is much, much easier to get working with cross compile*.

Another option is adding to you project Cargo.toml:
openssl-sys = { version = "*", features = ["vendored"] }

### linker `x86_64-w64-mingw32-gcc` not found

https://stackoverflow.com/questions/71623206/error-linker-x86-64-w64-mingw32-gcc-not-found

#### https://www.reddit.com/r/rust/comments/5k8uab/crosscompiling_from_ubuntu_to_windows_with_rustup/

`sudo apt install -y mingw-w64`


