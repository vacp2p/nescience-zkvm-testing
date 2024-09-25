# Test run instructions

Firstly there is requirements

- [Nexus](https://github.com/nexus-xyz/nexus-zkvm?tab=readme-ov-file#quick-start)

Next, we assume, that one is in `nexus` folder.

To prove execution we need to build one of the tests, let`s use [simple_arithmetic_test](./tests/simple_arithmetic_test/) as an example.

To generate a proof, run the following commands:

```sh
cd simple_arithmetic_test
cargo nexus prove
```