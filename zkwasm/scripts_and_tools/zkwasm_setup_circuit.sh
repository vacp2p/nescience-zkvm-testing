WASM_PATH=$1

cd zkWasm
cargo run --release -- --params params testwasm setup --host standard -k 18 --wasm $WASM_PATH
cd ..