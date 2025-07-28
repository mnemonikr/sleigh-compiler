## 2.0.2

### Fixed

- Duplicate object collisions when used with `libsla` no longer occur. Use `libsla-sys` crate instead of building Ghidra source.

## 2.0.1

### Changed

- Clarified compatibility issues introduced in 2.0.0 in [README](README.md)

## 2.0.0

💥 **Breaking change**: The SLEIGH compiler now produces _compressed_ .sla files. Tooling based on versions of Ghidra older than 11.1 will not understand this format. Tooling based on older versions of Ghidra should use version 1.0 of this compiler.

### Changed

- Updated the internal SLEIGH compiler to Ghidra 11.4

## 1.0.1

### Changed

- Fixed link in [README](README.md) that did not link correctly from crates.io.

## 1.0.0

Initial release! 🎉

### Added

- `SleighCompiler` for invoking SLEIGH compiler. Reported errors and warnings are included in compiler response object.
