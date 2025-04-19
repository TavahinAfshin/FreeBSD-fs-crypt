use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    
    let kernel_headers = match env::var("FREEBSD_SYS_HEADERS") {
        Ok(path) => path,
        Err(_) => {

            "/usr/src/sys".to_string()
        }
    };

    println!("cargo:rerun-if-changed={}", kernel_headers);
    if !std::path::Path::new(&kernel_headers).exists() {
        println!("cargo:warning=FreeBSD headers not found, generating empty bindings for development");
        std::fs::write(
            PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"),
            "// Empty bindings for development\n// Replace with actual bindings when building on FreeBSD\n"
        ).expect("Could not write empty bindings file");
        return;
    }
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", kernel_headers))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}