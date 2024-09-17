pushd ../tests/zkmips_tester
ELF_PATH=$1
RUST_LOG=info ELF_PATH=$ELF_PATH SEG_OUTPUT=/tmp/output cargo run --release bench
popd