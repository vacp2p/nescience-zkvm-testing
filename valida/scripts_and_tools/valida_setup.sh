set -eo

wget https://github.com/lita-xyz/llvm-valida-releases/releases/latest/download/llvm-valida-v0.3.0-alpha-linux-x86_64.tar.gz
tar xzvf llvm-valida-v0.3.0-alpha-linux-x86_64.tar.gz
cd llvm-valida-release
mv clang ..
mv DelendumEntryPoint.o ..
mv ld.lld ..
mv llc ..
mv valida ..
mv valida.ld ..
mv libstdio.a ..
cd ..
rm -rf llvm-valida-release
rm -rf llvm-valida-v0.3.0-alpha-linux-x86_64.tar.gz