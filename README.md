# Led matrix on raspberry pie

Rust application to show useful information on matrix led display. 

## Prerequisites 

- Install the target for arm64 on Linux. Used raspberry pie operates on Debian 64 bit on a arm cpu.
```sh
rustup target add aarch64-unknown-linux-gnu
```
- Install podman [Link](https://podman.io/), dependency for cross 
- Install cross as a rust tool. Cross Compiler [Link](https://github.com/cross-rs/cross).
```sh
cargo install cross --git https://github.com/cross-rs/cross
```

## Building 

Execute the shell script build.sh under the project root.

### What happens during ?
It cross compiles the app into for arm cpu as 64 bit and copies it to the raspberry pie.
Cross builds the app in a container with the needed compiler via podman. 
Make sure the raspberry pie is on and can be accessed via ssh !

