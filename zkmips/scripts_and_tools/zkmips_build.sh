TEST_PATH=$1

pushd $TEST_PATH
cargo build --release --target=mips-unknown-linux-musl
popd