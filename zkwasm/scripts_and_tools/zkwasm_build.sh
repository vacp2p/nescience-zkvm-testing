WASM_PATH=$1

pushd $WASM_PATH
wasm-pack build --release --out-name test.wasm --out-dir pkg
wasm-opt -Oz -o output.wasm pkg/test.wasm

rm -rf pkg output params
popd