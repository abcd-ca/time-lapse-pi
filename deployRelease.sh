# This target builds statically linked binaries
cross build --release --target=armv7-unknown-linux-musleabihf && \
scp target/armv7-unknown-linux-musleabihf/release/preview pi@raspberrypi.local:/home/pi/rust/time-lapse-pi-rust/ && \
scp target/armv7-unknown-linux-musleabihf/release/led pi@raspberrypi.local:/home/pi/rust/time-lapse-pi-rust/ && \
scp target/armv7-unknown-linux-musleabihf/release/button pi@raspberrypi.local:/home/pi/rust/time-lapse-pi-rust/ && \
scp target/armv7-unknown-linux-musleabihf/release/time-lapse-pi pi@raspberrypi.local:/home/pi/rust/time-lapse-pi-rust/

# This target depends on 64bit version of Raspberry Pi OS and is dynamically linked (to dependencies in the pi's linux distro)
#cross build --release --target=aarch64-unknown-linux-gnu && \
#scp target/aarch64-unknown-linux-gnu/release/preview pi@raspberrypi.local:/home/pi/rust/time-lapse-pi-rust/ && \
#scp target/aarch64-unknown-linux-gnu/release/led pi@raspberrypi.local:/home/pi/rust/time-lapse-pi-rust/ && \
#scp target/aarch64-unknown-linux-gnu/release/button pi@raspberrypi.local:/home/pi/rust/time-lapse-pi-rust/ && \
#scp target/aarch64-unknown-linux-gnu/release/time-lapse-pi pi@raspberrypi.local:/home/pi/rust/time-lapse-pi-rust/

