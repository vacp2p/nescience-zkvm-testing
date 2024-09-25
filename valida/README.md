# Test run instructions

To run corresponding tests in `Valida` some preparations have to be done.

We assume, that one is in `valida` folder.

Firstly, move into [scripts_and_tools](./scripts_and_tools/) directory.

Next, run

```sh
./valida_setup.sh
```

This will fetch all necessary components to build Rist code in `Valida`.

Next, to prove execution we need to build one of the tests, let`s use [simple_arithmetic_test](./tests/simple_arithmetic_test/) as an example.

Run

```sh
./build-rust.sh simple_arithmetic_test
```

If run is succsessfull then you will see `simple_arithmetic_test`, `simple_arithmetic_test.ll`, `simple_arithmetic_test.o` files in the scripts directory.

Next, we need prove execution, to do so run

```sh
./valida_prove.sh simple_arithmetic_test
```

If proof succsessfull, you will see `simple_arithmetic_test.proof` file in scripts derectory.

If needed, proof can be verified as follows

```sh
./valida_verify.sh simple_arithmetic_test
```