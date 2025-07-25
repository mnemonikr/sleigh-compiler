## About

This supports calling the Ghidra SLEIGH compiler from Rust code. Processors in Ghidra are described by `.slaspec` files, but Ghidra operates on the compiled `.sla` files. Now the compiled `.sla` artifacts can be produced directly from Rust code.

## Usage

```rust
let mut compiler = SleighCompiler::default();
let input_file = std::path::Path::new("Ghidra/Processors/x86/data/languages/x86-64.slaspec");
let output_file = std::path::Path::new("x86-64.sla");
compiler.compile(input_file, output_file)?;

// Success! Can now use compiled x86-64.sla file
```

## Compiler Details

The internal SLEIGH compiler is built from Ghidra 10.4.

The SLEIGH compiler may report warnings in its response which reference command line switches. For details on any reported switches, refer to [SLEIGH compiler usage](ghidra/Ghidra/Features/Decompiler/src/decompile/cpp/slgh_compile.cc#L3687-L3701).

## Related Work

Check out the [libsla](https://crates.io/crates/libsla) crate for interfacing with the Ghidra SLEIGH library to disassemble native instructions into pcode!
