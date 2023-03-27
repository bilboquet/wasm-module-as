# Build a wasm module in AS

This is not a real Rust project, it's just that I'm used to cargo to build a project.
This is a reimplementation in AS of a wasm module implemented in Rust for test purpose.

## Dependencies

`asc`
Installed by `npm install assemblyscript` to install in the current dir/node_modules

## Using external function
The equivalent of this `Rust` code (where we import external functions defined in a `.wai` file):
```Rust
wai_bindgen_rust::import!("host-exported.wai");
```
with:
```Command
$ cat host-exported.wai
echo: func()
```
is, in `assemblyscript`
```
@external("host-exported", "echo")
declare function echo(): void;
```

## Build

build in wrapped in build.rs
it generates a `wasm-module.wasm`

## Tips

```
wasmer inspect wasm-module.wasm
```
gives useful information about the module.