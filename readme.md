# Camera crate

There was a [camera crate](https://github.com/pedrosland/rascam) for the legacy Raspberry Pi OS that kind of worked but I found it easy enough to just call the external `raspistill` command. 
 A newer version of the distro doesn't come with `raspicam` set of commands, it comes with the open source, [`libcamera`](https://www.raspberrypi.com/documentation/accessories/camera.html#libcamera-and-the-legacy-raspicam-camera-stack) library which has a C++ API but no-one has created a Rust crate to interface with it yet. Maybe I could create that crate? In the meantime, I can work on a general interval based application that takes pictures every n seconds, saves images and maybe later stitches them together (or have another binary in this same package to stitch them)  

TODO update OS and try the newer CLI command

## linking to libcamera library

Should need C++ `.h` files but not source files to link. [This crate](https://crates.io/crates/bindgen) might help automatically expose the API. That's how the rascam crate uses the raspicam library (which [binds to the mmal-sys library using bindgen](https://github.com/pedrosland/mmal-sys/blob/master/Cargo.toml))

# interval

take a picture every n seconds. [Here's a solution](https://stackoverflow.com/questions/56253623/how-can-i-run-a-set-of-functions-on-a-recurring-interval-without-running-the-sam) for the interval and threads, not picture specific

# camera

This is a [better quality camera](https://www.arducam.com/docs/cameras-for-raspberry-pi/raspberry-pi-libcamera-guide/) that is compatible with Raspberry Pi than the V2 cam I've been using

