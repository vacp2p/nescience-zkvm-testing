# Test run instructions


- Those are the requirements for [Nexus](https://github.com/nexus-xyz/nexus-zkvm?tab=readme-ov-file#quick-start), in `nexus` folder

To prove execution, one needs to build one of the tests, let`s use [simple_arithmetic_test](./tests/simple_arithmetic_test/) as an example.

To generate a proof, run the following commands:

```sh
cd simple_arithmetic_test
cargo nexus prove
```
