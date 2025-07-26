## 2.0.0

**Breaking change**: The SLEIGH compiler now produces _compressed_ .sla files. Tooling based on versions of Ghidra older than 11.1 will not understand this format. The uncompressed versions of .sla files can be produced by setting `debug_output` to `true` in the compiler options for compatibility with older tools.

The Ghidra team does not plan to support use of uncompressed .sla files going forward ([#6416](https://github.com/NationalSecurityAgency/ghidra/issues/6416)).

### Changed

- Updated the internal SLEIGH compiler to Ghidra 11.4

## 1.0.1

### Changed

- Fixed link in [README](README.md) that did not link correctly from crates.io.

## 1.0.0

Initial release! 🎉

### Added

- `SleighCompiler` for invoking SLEIGH compiler. Reported errors and warnings are included in compiler response object.
