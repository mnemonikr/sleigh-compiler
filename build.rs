use std::path::Path;

fn main() {
    let source_path = Path::new("ghidra/Ghidra/Features/Decompiler/src/decompile/cpp");

    // The following sources were pulled from the Makefile
    const SLACOMP_SOURCE_FILES: &[&str] = &[
        // CORE=	xml marshal space float address pcoderaw translate opcodes globalcontext
        "xml.cc",
        "marshal.cc",
        "space.cc",
        "float.cc",
        "address.cc",
        "pcoderaw.cc",
        "translate.cc",
        "opcodes.cc",
        "globalcontext.cc",
        // SLEIGH=	sleigh pcodeparse pcodecompile sleighbase slghsymbol \
        // slghpatexpress slghpattern semantics context filemanage
        "sleigh.cc",
        "pcodeparse.cc",
        "pcodecompile.cc",
        "sleighbase.cc",
        "slghsymbol.cc",
        "slghpatexpress.cc",
        "slghpattern.cc",
        "semantics.cc",
        "context.cc",
        "slaformat.cc",   // New in Ghidra 11
        "compression.cc", // New in Ghidra 11
        "filemanage.cc",
        // SLACOMP=slgh_compile slghparse slghscan
        "slghparse.cc",
        "slghscan.cc",
    ];

    let zlib_path = Path::new("ghidra/Ghidra/Features/Decompiler/src/decompile/zlib");
    const ZLIB_SOURCE_FILES: &[&str] = &[
        "adler32.c",
        "deflate.c",
        "inffast.c",
        "inflate.c",
        "inftrees.c",
        "trees.c",
        "zutil.c",
    ];

    cxx_build::bridge("src/ffi/sys.rs")
        .define("LOCAL_ZLIB", "1")
        .define("NO_GZIP", "1")
        .flag_if_supported("-std=c++14")
        .files(SLACOMP_SOURCE_FILES.iter().map(|s| source_path.join(s)))
        .files(ZLIB_SOURCE_FILES.iter().map(|s| zlib_path.join(s)))
        .file("src/ffi/cpp/bridge.cc")
        // A fork of the slgh_compile.cc file is maintained locally to enable it
        // to be used as a library instead of as an executable
        .file("src/ffi/cpp/slgh_compile.cc")
        .include(source_path) // Header files coexist with cpp files
        .warnings(false) // Not interested in the warnings for Ghidra code
        .compile("slacomp.a");

    // Rerun if any of the bindings have changed
    println!("cargo:rerun-if-changed=src/ffi/sys.rs");

    // Rerun if any of the C++ bridge code has changed
    println!("cargo:rerun-if-changed=src/ffi/cpp");
}
