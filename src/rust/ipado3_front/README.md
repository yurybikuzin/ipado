# Sales Front

## Prerequisites

### wasm-pack

https://rustwasm.github.io/wasm-pack/installer/

### [How do I fix the Rust error "linker 'cc' not found" for Debian on Windows 10?](https://stackoverflow.com/questions/52445961/how-do-i-fix-the-rust-error-linker-cc-not-found-for-debian-on-windows-10)

```
sudo apt-get update &&  sudo apt install -y build-essential
```

### [rust-script](https://rust-script.org/)

```
cargo install rust-script
```

## Build

in project folder (`~/abc/src/rust/sales_front`)

```
wasm-pack build --target web --no-typescript
```

## Local Development

in project folder (`~/abc/src/rust/sales_front`)

```
simple-http-server 
```

open localhost:8000/src/index.html

## Google Identity Service 

https://developers.google.com/identity/sign-in/web/sign-in - DEPRECATED
https://developers.google.com/identity/oauth2/web/guides/migration-to-gis#implicit_flow_examples
https://developers.google.com/identity/branding-guidelines

## Tools

https://convertio.co/ru/png-svg/
