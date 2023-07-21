# `injrs`

DLL injector library and tool written in Rust.

[![Apache-2.0](https://img.shields.io/crates/l/injrs.svg)](https://github.com/jiusanzhou/injrs/blob/master/LICENSE)

## Usage

At most time, you can use `injrs` as a simple tool.

```bash
USAGE:
injrs PROCESS_NAME/PID [Libraies...]

EXAMPLES:
1. Inject test.dll to process Calc.exe
    $ injrs Calc.exe test.dll

2. Inject test.dll and demo.dll to process with PID: 1888
    $ injrs 1888 test.dll demo.dll
```

## DLL Demo

You can build with command:

```bash
cargo build --release --example inject
cargo build --release --example source
cargo build
```

Build target will locate in:

```bash
target/i686-pc-windows-msvc/release/examples/inject.dll
target/i686-pc-windows-msvc/release/examples/source.exe
target/i686-pc-windows-msvc/debug/injrs.exe
```

Try to inject the demo dll to your target process:

```bash
./target/i686-pc-windows-msvc/debug/injrs.exe target/i686-pc-windows-msvc/release/examples/source.exe target/i686-pc-windows-msvc/release/examples/inject.dll
```

## Usage as library

You also can write a injector project using `injrs` as a library.

```rust
use injrs::process_windows::*;
use injrs::inject_windows::*;

fn main() {
    let name = "Calc.exe";
    let dll = "./my-demo-dll.dll";
    let p = Process::find_first_by_name(name).unwrap();

    print!("inject dll to process => ");
    match process.inject(dll) {
        Err(e) => {
            println!("error: {}", e);
        },
        Ok(_) => {
            println!("success");
        }
    }
}
```
