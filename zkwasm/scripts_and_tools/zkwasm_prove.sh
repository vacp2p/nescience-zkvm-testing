WASM_PATH=$1

cd zkWasm
./target/release/zkwasm-cli --params ./params testwasm prove --output ./output --wasm $WASM_PATH
cd ..