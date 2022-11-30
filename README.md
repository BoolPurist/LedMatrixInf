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

### What happens during building ?
It cross compiles the app into for arm cpu as 64 bit and copies it to the raspberry pie.
Cross builds the app in a container with the needed compiler via podman. 
Make sure the raspberry pie is on and can be accessed via ssh !


### Current progress on cross compiling and control led matrix

Cross compiling works right now.  However establishing a control to the led matrix fails.

It fails with the error MatrixCreationError::ThreadTimedOut

Problem: Harded coded 2 seconds for waiting until aborting the init process. 
See line 303 under https://github.com/EmbersArc/rpi_led_panel/blob/main/src/rgb_matrix.rs

Possible Solution: Increase waiting time. Need to change code in library.

Switched from [rust-rpi-rgb-led-matrix](https://github.com/rust-rpi-led-matrix/rust-rpi-rgb-led-matrix)
to [rpi_led_panel](https://github.com/EmbersArc/rpi_led_panel).

Reason: rust-rpi-rgb-led-matrix currently does not support cross compiling. See following 
[issue](https://github.com/rust-rpi-led-matrix/rust-rpi-rgb-led-matrix/issues/43)

### Fix for errors panics

If the programs panic with the error:
GpioInitializationError::So: undModuleLoaded

Sound on raspbe
Then one need to change dtparam=audio to off, dtparam=audio=off, in the file /boot/config.txt

