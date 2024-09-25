WASM_PATH=$1
CIRCUIT_SIZE=$2

cd zkWasm
cargo run --release -- --params params testwasm setup --host standard -k $CIRCUIT_SIZE --wasm $WASM_PATH
cd ..