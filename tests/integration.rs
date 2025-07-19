//! Integration test: verifies that the Arroyo UDF plugin dynamic library can be loaded.
//!
//! BDD: As a downstream Arroyo user, I want to ensure the UDF plugin shared library loads successfully so that I can use its exported functions in streaming queries.

use libloading::Library;

#[test]
fn loads_udf_plugin_library() {
    // This test assumes the shared library is built to target/release or target/debug.
    // We'll look for the library in both locations, preferring release.
    fn dylib_name(base: &str) -> String {
        let (prefix, suffix) = match std::env::consts::OS {
            "windows" => ("", "dll"),
            "macos" => ("lib", "dylib"),
            _ => ("lib", "so"),
        };
        format!("{}{}.{}", prefix, base, suffix)
    }

    let lib_name = dylib_name("arroyo_soc_udf");

    // Get the directory of the running test binary
    let exe_dir = std::env::current_exe()
        .expect("Failed to get path of running test binary")
        .parent()
        .expect("Test binary has no parent directory")
        .to_path_buf();

    // Look for the dynamic library in the same directory as the test binary
    let lib_path = exe_dir.join(lib_name);

    assert!(
        lib_path.exists(),
        "Could not find built UDF dynamic library at {:?}. Did you run `cargo test` or `cargo build`?",
        lib_path
    );

    // Attempt to load the library
    let _lib = unsafe { Library::new(&lib_path) }
        .expect("Failed to load UDF dynamic library");
}
