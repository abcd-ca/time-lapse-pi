# About this app

This app was written in Rust. It provides a more convenient interface to configuring and starting a time-lapse recording.
To work out in the field where you have no way to connect to it over SSH, I added a button and LED to the GPIO pins. You configure the settings for the time-lapse at home then take your pi somewhere for a time-lapse. The app starts when the pi boots up and the light turns on. You press the button to start recording. The light flickers between each picture and turns solid again when it's done. 

TODO add a diagram of the wiring

You use your mac to cross compile to Raspberry Pi then copy the executable binaries to the Pi. Therefore, setting up the pi itself is extremely minimal.
You will need to [install Rust](https://www.rust-lang.org/tools/install) and [cross](https://github.com/cross-rs/cross) on your mac (not the pi) 

TODO cross compile instructions for non-macOS computers

# Setup the Raspberry Pi

This was tested on a Raspberry Pi 3B+ with a V2 Raspberry Pi Camera Module.

1. Format SD card with [Raspberry Pi Imager](https://www.raspberrypi.com/software/)
   2. Choose "Raspberry Pi OS Lite (64-bit)" – a port of Debian Bullseye with no desktop environment. Note: Raspberry Pi OS Lite (Legacy) will not work with this app as it relies on newer `libcamera` camera commands that don't come with the legacy version.
   3. Use the gear icon to set up the WiFi connection and enable SSH. `raspberrypi.local` will be the hostname and the password you pick will be the one to use when connecting to the pi with SSH later
2. Log into the pi `ssh pi@raspberrypi.local`
3. `sudo apt update`
4. `sudo apt full-upgrade`
5. To be able to stitch stills into an MP4 on the pi, `sudo apt install ffmpeg`

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
* see `/scripts/deployRelease.sh`
* Copy the binary to the pi using rsync (`deployRelease.sh` also does this)
* run the binary on the pi using, `./time-lapse-pi` (you'll need to stop the process started on boot first if there is one)

# Connecting to raspberry pi:

`ssh pi@raspberrypi.local` password `raspberry`

# Start time-lapse up on boot

1. Configure the pi as root user because the config file will be saved under the root user's config files and the `/etc/rc.local` file runs commands as root. `sudo ./time-lapse-pi config create --preset sky --capture-duration 10h --delay-start-duration 10h`
2. Edit the boot config file: `sudo nano /etc/rc.local`
3. Add the following before the `exit 0` line: 

```
# time-lapse camera------ start

# https://raspberrypi-guide.github.io/programming/run-script-on-boot
bash -c '/home/pi/rust/time-lapse-pi-rust/time-lapse-pi start > /home/pi/rust/time-lapse-pi-rust/time-lapse.log 2>&1' &

# time-lapse camera------ end
```

4. Reboot the pi `sudo reboot -h now`. After boot, the yellow LED should be solid. Press the LED to begin the time lapse. If you've configured a delay-start-duration then pushing the button will begin the delay and capturing will ensue. 

Since the pi is running the time-lapse app as root and not through a terminal, it won't timeout and disconnect, ending the time-lapse process. There is no need for `nohup` in this case.

5. If still on the network, you can view a log of the time-lapse by running `tail -f /home/pi/rust/time-lapse-pi-rust/time-lapse.log` in the project root folder

# Pi Enclosure

I 3D printed [this enclosure](https://www.thingiverse.com/thing:685074). I like it because it's got a built-in camera holder that you can articulate. The ball joint needed to be printed at 90% scale and I found I needed to use a bit of glue to keep the two halves of the camera enclosure together and the back of the pi case.  

# Other reference

## Raspberry pi GPIO Pinouts
https://pinout.xyz/

## Camera

This is a [better quality camera](https://www.arducam.com/docs/cameras-for-raspberry-pi/raspberry-pi-libcamera-guide/) that is compatible with Raspberry Pi than the V2 cam I've been using 
