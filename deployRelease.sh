./test.sh && \
./build.sh && \
scp target/armv7-unknown-linux-musleabihf/release/preview pi@raspberrypi.local:/home/pi/rust/time-lapse-pi-rust/ && \
scp target/armv7-unknown-linux-musleabihf/release/led pi@raspberrypi.local:/home/pi/rust/time-lapse-pi-rust/ && \
scp target/armv7-unknown-linux-musleabihf/release/button pi@raspberrypi.local:/home/pi/rust/time-lapse-pi-rust/ && \
scp target/armv7-unknown-linux-musleabihf/release/time-lapse-pi pi@raspberrypi.local:/home/pi/rust/time-lapse-pi-rust/
