use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
};

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch != "x86_64" {
        let mut w = BufWriter::new(File::create("platform_config").unwrap());

        for (key, value) in env::vars() {
            if key.starts_with("CARGO_CFG_") {
                println!("{}: {:?}", key, value);
                w.write_fmt(format_args!("{}: {:?}\n", key, value)).unwrap();
            }
        }
    }

    // // CARGO_CFG_TARGET_ARCH: "x86_64"
    // panic!("stop and dump stdout");
}
