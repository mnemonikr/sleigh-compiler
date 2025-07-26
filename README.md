## About

This supports calling the Ghidra SLEIGH compiler from Rust code. Processors in Ghidra are described by `.slaspec` files, but SLEIGH libraries operates on the compiled `.sla` files. Now the compiled `.sla` artifacts can be produced directly from Rust code.

## What's New?

For latest details check out the [CHANGELOG](./CHANGELOG.md).

## Usage

```rust
let mut compiler = SleighCompiler::default();
let input_file = std::path::Path::new("Ghidra/Processors/x86/data/languages/x86-64.slaspec");
let output_file = std::path::Path::new("x86-64.sla");
compiler.compile(input_file, output_file)?;

// Success! Can now use compiled x86-64.sla file
```

## Compiler Details

The internal SLEIGH compiler is built from Ghidra 11.4.

The SLEIGH compiler may report warnings in its response which reference command line switches. For details on any reported switches, refer to [SLEIGH compiler usage](https://github.com/NationalSecurityAgency/ghidra/blob/1801dc1ee62be73faae29961ec2f17a59423f156/Ghidra/Features/Decompiler/src/decompile/cpp/slgh_compile.cc#L3736-L3747).

### Compatibility

In Ghidra 11.1 the .sla file format was changed to a compressed format. To produce .sla files compatible with tooling based on older versions of Ghidra, set `debug_output: true` in the compiler options.

```rust
let mut compiler = SleighCompiler::new(SleighCompilerOptions {
    // This will produce .sla files compatible with older tooling
    debug_output: true,
    ..Default::default()
});
let input_file = std::path::Path::new("Ghidra/Processors/x86/data/languages/x86-64.slaspec");
let output_file = std::path::Path::new("x86-64.sla");
compiler.compile(input_file, output_file)?;
```

Note that these uncompressed files are _not_ compatible with tooling based on later versions of Ghidra, and the Ghidra team has no plans to add such support ([#6416](https://github.com/NationalSecurityAgency/ghidra/issues/6416)).

## Related Work

Check out the [libsla](https://crates.io/crates/libsla) crate for interfacing with the Ghidra SLEIGH library to disassemble native instructions into pcode!
