export NDK_HOME='/home/carlosb/Desktop/tools/android-ndk-r19c'
$NDK_HOME/build/tools/make_standalone_toolchain.py --api 26 --arch arm64 --install-dir NDK/arm64
$NDK_HOME/build/tools/make_standalone_toolchain.py --api 26 --arch arm64 --install-dir NDK/arm
$NDK_HOME/build/tools/make_standalone_toolchain.py --api 26 --arch arm64 --install-dir NDK/x86

# DEFINE CARGO RUST
cp cargo-config.toml ~/.cargo/config
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android

cargo new cargo
mkdir android



