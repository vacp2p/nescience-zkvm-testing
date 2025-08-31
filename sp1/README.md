# Test run instructions

Those are the requirements for [SP1](https://docs.succinct.xyz/getting-started/install.html), in `sp1` folder.

To prove execution one needs to build one of the tests, let`s use [simple_arithmetic_test](./tests/simple_arithmetic_test/) as an example.

To generate a proof, run the following commands:

```sh
cd simple_arithmetic_test
cd script
RUSTFLAGS='-C target-cpu=native' cargo run --release -- --prove
```

If you are on a CPU with AVX512 support, you can use the following flags for more performance: RUSTFLAGS='-C target-cpu=native -C target_feature=+avx512ifma,+avx512vl'.