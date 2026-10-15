// Links libmojoguard. With AEGIS_MOJO_LIB_DIR set, link the real Mojo shared lib;
// otherwise compile the C stub that implements the same ABI.
fn main() {
    println!("cargo:rerun-if-env-changed=AEGIS_MOJO_LIB_DIR");
    println!("cargo:rerun-if-changed=../contracts/mojoguard.h");
    println!("cargo:rerun-if-changed=../mojo-engine/stub/mojoguard_stub.c");
    match std::env::var("AEGIS_MOJO_LIB_DIR") {
        Ok(dir) => {
            println!("cargo:rustc-link-search=native={dir}");
            println!("cargo:rustc-link-lib=dylib=mojoguard");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{dir}");
        }
        Err(_) => {
            cc::Build::new()
                .file("../mojo-engine/stub/mojoguard_stub.c")
                .include("../contracts")
                .opt_level(2)
                .compile("mojoguard_stub");
        }
    }
}
