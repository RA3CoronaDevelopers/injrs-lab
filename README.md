# `injrs`

DLL injector library and tool written in Rust.

[![Apache-2.0](https://img.shields.io/crates/l/injrs.svg)](https://github.com/jiusanzhou/injrs/blob/master/LICENSE)

## Usage

At most time, you can use `injrs` as a simple tool.

```bash
USAGE:
injrs PROCESS_PATH/PID [Libraies...]

EXAMPLES:
1. Inject test.dll to process Calc.exe
    $ injrs Calc.exe test.dll

2. Inject test.dll and demo.dll to process with PID: 1888
    $ injrs 1888 test.dll demo.dll
```

## DLL Demo

You can build with command:

```bash
cargo build --release --example eat-hook
cargo build --release --example source
cargo build
```

Build target will locate in:

```bash
target/i686-pc-windows-msvc/release/examples/eat-hook.dll
target/i686-pc-windows-msvc/release/examples/source.exe
target/i686-pc-windows-msvc/debug/injrs.exe
```

Run the demo process independently:

```bash
./target/i686-pc-windows-msvc/release/examples/source.exe
```

Try to inject the demo dll to your target process:

```bash
./target/i686-pc-windows-msvc/debug/injrs.exe target/i686-pc-windows-msvc/release/examples/source.exe target/i686-pc-windows-msvc/release/examples/eat-hook.dll
```
