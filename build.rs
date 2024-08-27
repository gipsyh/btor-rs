use std::env;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let src_dir = env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| "Environmental variable `CARGO_MANIFEST_DIR` not defined.".to_string())?;

    println!(
        "cargo:rustc-link-search=native={}",
        PathBuf::from(src_dir).display()
    );
    println!("cargo:rerun-if-changed=./libbtor2aiger.a");
    println!("cargo:rerun-if-changed=./libboolector.a");

    println!("cargo:rustc-link-lib=static=lgl");
    println!("cargo:rustc-link-lib=static=boolector");
    println!("cargo:rustc-link-lib=static=btor2parser");
    println!("cargo:rustc-link-lib=static=btor2aiger");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    Ok(())
}
