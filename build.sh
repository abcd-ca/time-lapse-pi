# This target builds statically linked binaries
# https://github.com/cross-rs/cross/issues/975#issuecomment-1304641814
CROSS_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_IMAGE_TOOLCHAIN="aarch64-unknown-linux-gnu" \
CROSS_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_IMAGE=ahuszagh/aarch64-cross:armv7-unknown-linux-musleabihf \
cross build --release --target=armv7-unknown-linux-musleabihf

# This target depends on 64bit version of Raspberry Pi OS and is dynamically linked (to dependencies in the pi's linux distro)
#CROSS_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_IMAGE_TOOLCHAIN="aarch64-unknown-linux-gnu" \
#CROSS_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_IMAGE=ahuszagh/aarch64-cross:armv7-unknown-linux-musleabihf \
#cross build --release --target=aarch64-unknown-linux-gnu




