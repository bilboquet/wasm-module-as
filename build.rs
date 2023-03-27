use std::env;
use std::process::Command;

#[allow(unreachable_code, unused_variables)]
fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    eprintln!("out_dir: {}", &out_dir);

    let node_tools_dir = out_dir + "/node_modules/.bin/";
    let asc = node_tools_dir + "asc";

    Command::new(asc)
        .args(["wasm.ts", "-o", "wasm-module.wasm"])
        .status()
        .unwrap();

    println!("cargo:rerun-if-changed=wasm.ts");
}
