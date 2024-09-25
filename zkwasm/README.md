# Test run instructions

To run corresponding tests in `zkWASM` some preparations have to be done.

We assume, that one is in `zkwasm` folder.

Firstly, move into [scripts_and_tools](./scripts_and_tools/) directory.

Next, run

```sh
./zkwasm_setup.sh
```

This will create `zkWasm` folder with sorce code and attempt to build it.

Next, to prove execution we need to build one of the tests, let`s use [simple_arithmetic_test](./tests/simple_arithmetic_test/) as an example.

Run

```sh
./zkwasm_build.sh ../tests/simple_arithmetic_test
```

If run is succsessfull then you will see `output.wasm` file in the test directory.

Next, we need to setup proving circuit, to do so, run

```sh
./zkwasm_setup_circuit.sh ../../tests/simple_arithmetic_test/output.wasm 18
```

Note, that testing executed from the `zkWasm` directory, thus path have to be relative to it, or global.

Note, that second parameter (18) is a size of a circuit. Bigger executables will need bigger circuit sizes to be proven.

Now, that circuit is built, we can prove execution as follows

```sh
./zkwasm_prove.sh ../../tests/simple_arithmetic_test/output.wasm
```

If needed, proof can be verified as follows

```sh
./zkwasm_verify.sh ../../tests/simple_arithmetic_test/output.wasm
```

