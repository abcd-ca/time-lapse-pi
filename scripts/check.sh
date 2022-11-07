#cross test --target=aarch64-unknown-linux-gnu
CROSS_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_IMAGE_TOOLCHAIN="aarch64-unknown-linux-gnu" \
CROSS_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_IMAGE=ahuszagh/aarch64-cross:armv7-unknown-linux-musleabihf \
cross check --target=armv7-unknown-linux-musleabihf
