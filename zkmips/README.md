# Test run instructions

To run corresponding tests in `zkMIPS` some preparations have to be done.

We assume, that one is in `zkmips` folder.

Firstly, move into [scripts_and_tools](./scripts_and_tools/) directory.

Next, run

```sh
./zkmips_setup.sh
```

This will get mips compiler.

After setup, and getting compiler, go to ~/.cargo/config , and add:

```
[target.mips-unknown-linux-musl]
linker = "<path-to>/mips-linux-muslsf-cross/bin/mips-linux-muslsf-gcc"
rustflags = ["--cfg", 'target_os="zkvm"',"-C", "target-feature=+crt-static", "-C", "link-arg=-g"]
```

Additionally, be sure, that you are at the latest nightly toolchain.

Next, to prove execution we need to build one of the tests, let`s use [simple_arithmetic_test](./tests/simple_arithmetic_test/) as an example.

Run

```sh
./zkmips_build.sh ../tests/simple_arithmetic_test
```

If build is succsessfull you will see `simple_arithmetic_test` binary in `tests/simple_arithmetic_test/target/mips-unknown-linux-musl/release/`.

Now, to benchmark this test we must do the following

```sh
./zkmips_bench.sh <path-to-elf>
```

Where `path-to-elf` is abovementioned path to `simple_arithmetic_test` binary relative to [zkmips_tester](./tests/zkmips_tester/) folder of global.

This will build a tester, and after that, bench proving and do verification of a proof.