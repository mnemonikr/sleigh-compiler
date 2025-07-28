use std::path::Path;

fn main() {
    let source_path = Path::new("ghidra/Ghidra/Features/Decompiler/src/decompile/cpp");
    cxx_build::bridge("src/ffi/sys.rs")
        .define("LOCAL_ZLIB", "1")
        .define("NO_GZIP", "1")
        .flag_if_supported("-std=c++14")
        .file("src/ffi/cpp/bridge.cc")
        // A fork of the slgh_compile.cc file is maintained locally to enable it
        // to be used as a library instead of as an executable
        .file("src/ffi/cpp/slgh_compile.cc")
        .include(source_path) // Header files coexist with cpp files
        .warnings(false) // Not interested in the warnings for Ghidra code
        .compile("slacomp");

    println!("cargo:rustc-link-lib=libsla.a");

    // Rerun if any of the bindings have changed
    println!("cargo:rerun-if-changed=src/ffi/sys.rs");

    // Rerun if any of the C++ bridge code has changed
    println!("cargo:rerun-if-changed=src/ffi/cpp");
}
