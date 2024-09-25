# Test run instructions

Firstly there is requirements

- [SP1](https://docs.succinct.xyz/getting-started/install.html)

Next, we assume, that one is in `sp1` folder.

To prove execution we need to build one of the tests, let`s use [simple_arithmetic_test](./tests/simple_arithmetic_test/) as an example.

To generate a proof, run the following commands:

```sh
cd simple_arithmetic_test
cd script
cargo run --release -- --prove
```