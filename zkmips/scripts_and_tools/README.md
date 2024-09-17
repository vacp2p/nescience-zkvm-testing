# ATTENTION

Some things have to be done manually

After setup, and getting compiler, go to ~/.cargo/config , and add:

```
[target.mips-unknown-linux-musl]
linker = "<path-to>/mips-linux-muslsf-cross/bin/mips-linux-muslsf-gcc"
rustflags = ["--cfg", 'target_os="zkvm"',"-C", "target-feature=+crt-static", "-C", "link-arg=-g"]
```

Additionally, be sure, that you ate at the latest nightly toolchain.