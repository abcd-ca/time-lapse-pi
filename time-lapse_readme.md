# Setup

* Format SD card with Raspberry Pi Imager, Choose the 64 bit lite version
* `sudo apt update`
* `sudo apt full-upgrade`
* to be able to stitch stills into an mp4 on the pi, `sudo apt install ffmpeg`

## Getting the app on the pi

### Easiest way

This way installs a lot of extra stuff and puts the source code on the pi

* install rust using the shell script on the front page of the rust website
* develop on the mac and in webstorm set up the deploy scripts to sync changes on save to the pi over sftp
* `cargo run -- 180` (where `--` indicates to cargo to send the next arg to the binary rather than cargo). `180` is the total number of images to capture
	* to run it for a long time, see "long running process" lower in this document

### Better production way

Cross compile on mac for raspberry pi and only copy the binary to the pi, no Rust dependencies or source code on the pi
* `cargo install cross` (depends on docker)
* see `deployRelease.sh`
* Copy the binary to the pi using rsync (`deployRelease.sh` also does this)
* run the binary on the pi using, `time-lapse-pi 180`
	* to run it for a long time, see "long running process" lower in this document

#### draft section

These are just notes for tests, they may not work

* setup step `rustup target add armv7-unknown-linux-gnueabihf`
* build for pi: `cargo build --target aarch64-arm-none-eabi`
 
# Preview image

Before you start a timelapse recording you can preview the image the camera will capture – lighting, angle etc. by streaming:

1. Start stream on pi: `libcamera-vid -t 0 --inline --listen -o tcp://0.0.0.0:8080`
2. View stream on mac by opening VLC > File > Open Network. URL `tcp/h264://raspberrypi.local:8080`
3. When you exit in the client, the server will also quit (haven't tried to find a way around this since I only care about a preview for setting up the timelapse

# Connecting to raspberry pi:

`ssh pi@raspberrypi.local` password `raspberry`

# Camera crate

There was a [camera crate](https://github.com/pedrosland/rascam) for the legacy Raspberry Pi OS that kind of worked but I found it easy enough to just call the external `raspistill` command. 
 A newer version of the distro doesn't come with `raspicam` set of commands, it comes with the open source, [`libcamera`](https://www.raspberrypi.com/documentation/accessories/camera.html#libcamera-and-the-legacy-raspicam-camera-stack) library which has a C++ API but no-one has created a Rust crate to interface with it yet. Maybe I could create that crate? In the meantime, I can work on a general interval based application that takes pictures every n seconds, saves images and maybe later stitches them together (or have another binary in this same package to stitch them)  

# TODO 
* update OS and try the newer CLI command
* have Rust automatically stitch the stills into an mp4 after using 
	* `ffmpeg -framerate 24 -i img%03d.png output.mp4`. Consider possibly running Rust in a docker image
	* or `ffmpeg -r 10 -pattern_type glob -i "*.jpg" -s 1920x1440 -vcodec libx264 output.mp4` **Tried this version, works well**
	* Dependency: `sudo apt	 install ffmpeg` (doesn't work in legacy Raspian OS) or could create a [docker](https://www.simplilearn.com/tutorials/docker-tutorial/raspberry-pi-docker#installing_docker_raspberry_pi) image containing ffmpeg and rust
 
## linking to libcamera library

Should need C++ `.h` files but not source files to link. [This crate](https://crates.io/crates/bindgen) might help automatically expose the API. That's how the rascam crate uses the raspicam library (which [binds to the mmal-sys library using bindgen](https://github.com/pedrosland/mmal-sys/blob/master/Cargo.toml))

# interval

take a picture every n seconds. [Here's a solution](https://stackoverflow.com/questions/56253623/how-can-i-run-a-set-of-functions-on-a-recurring-interval-without-running-the-sam) for the interval and threads, not picture specific

# camera

This is a [better quality camera](https://www.arducam.com/docs/cameras-for-raspberry-pi/raspberry-pi-libcamera-guide/) that is compatible with Raspberry Pi than the V2 cam I've been using

# Running the app on the pi
In order to prevent the long running process from disconnecting from the ssh session, prematurely ending the time lapse captures, you can execute the app like so:

## using Cargo

`cargo run -- 180` This runs a debug version of the rust app and passes one argument whose value is `180` to the app's binary (the `--` indicates to Cargo that the ensuing arguments are meant for the binary rather than Cargo). You could also run the compiled app from the targets folder after running `cargo build` or `cargo build --release`

# preventing disconnect of long running proccess

`nohup cargo run -- 180 &` This creates a background service-like task that runs in the background whether the ssh session disconnects after timing out or if you disconnect immediately intentionally. 

## How to log the background job

`nohup` logs `stdout` to `nohup.log` wherever you ran the command from (the project root on the pi in this case). So, you can do `tail -f nohup.log` to see the progress of the app. 

## How to cancel the timelapse background job

https://stackoverflow.com/a/17389526

# GPIO Pinouts
https://pinout.xyz/

# Added app as a linux service so it runs on boot (starts with button press)

Add a systemd service