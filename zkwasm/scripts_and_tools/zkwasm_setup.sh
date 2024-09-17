set -eo

git clone --recurse-submodules https://github.com/DelphinusLab/zkWasm.git
cd zkWasm
cargo build --release
cd ..