# Note, can modify test.sh and build.sh to use --release for a release build. Release build will compile slower but run faster

# This target builds statically linked binaries
# https://github.com/cross-rs/cross/issues/975#issuecomment-1304641814
CROSS_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_IMAGE_TOOLCHAIN="aarch64-unknown-linux-gnu" \
CROSS_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_IMAGE=ahuszagh/aarch64-cross:armv7-unknown-linux-musleabihf \
cross build --target=armv7-unknown-linux-musleabihf

# This target depends on 64bit version of Raspberry Pi OS and is dynamically linked (to dependencies in the pi's linux distro)
#CROSS_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_IMAGE_TOOLCHAIN="aarch64-unknown-linux-gnu" \
#CROSS_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_IMAGE=ahuszagh/aarch64-cross:armv7-unknown-linux-musleabihf \
#cross build --target=aarch64-unknown-linux-gnu



