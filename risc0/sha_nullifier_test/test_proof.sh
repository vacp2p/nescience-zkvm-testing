cargo update -p sha2 --precise 0.10.8
RISC0_DEV_MODE=0 RUST_LOG=info RISC0_INFO=1 cargo run --release
