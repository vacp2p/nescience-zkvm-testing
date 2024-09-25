git clone https://github.com/risc0/risc0.git

cp ../benchmark_tests/guest_programs/mem_alloc_vec_test.rs risc0/benchmarks/methods/guest/src/bin/

cp ../benchmark_tests/guest_programs/simple_arithmetic_test.rs risc0/benchmarks/methods/guest/src/bin/

cp ../benchmark_tests/bench_programs/mem_alloc_vec_test.rs risc0/benchmarks/src/benches/

cp ../benchmark_tests/bench_programs/simple_arithmetic_test.rs risc0/benchmarks/src/benches/

cd  risc0/benchmarks/src/benches/
        rm -rf mod.rs
    cd ..
    rm -rf main.rs
cd ../../../

cp ../benchmark_tests/mod.rs risc0/benchmarks/src/benches/

cp ../benchmark_tests/main.rs risc0/benchmarks/src/