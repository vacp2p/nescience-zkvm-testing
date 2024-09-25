# Test run instructions

To run corresponding tests in `RISC0` some preparations have to be done.

We assume, that one is in `risc0` folder.

Firstly, move into [scripts_and_tools](./scripts_and_tools/) directory.

Next, run

```sh
./risc0_setup.sh
```

This will fetch all necessary components for benchmarking.

Next, to prove execution we need to build one of the tests, we will use `simple_arithmetic_test` as an example.

Run

```sh
./risc0_bench_arithmetic.sh
```

This will compile benchmark, generate proof and prins statictics.

Alternatively, for memory test, you can do

```sh
./risc0_bench_memory.sh
```