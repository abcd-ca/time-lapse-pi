# Note, add --release to test and build scripts if you want a release bug
# and change paths in the scp steps below to be release instead of build

# Run this script from root of the project

scripts/test.sh && \
scripts/build.sh && \
scp target/armv7-unknown-linux-musleabihf/debug/time-lapse-pi pi@raspberrypi.local:/home/pi/rust/time-lapse-pi-rust/
