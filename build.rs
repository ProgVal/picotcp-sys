extern crate bindgen;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use bindgen::Builder;

fn build_picotcp(prefix: &Path) {
    println!("Building picotcp to: {}", prefix.to_string_lossy());
    Command::new("make")
            .arg("-C").arg("picotcp") // $RUSTOTCP_SOURCE/picotcp/
            .env("PREFIX", prefix.as_os_str())
            .status().unwrap();
}

fn generate_bindings(out_file: &Path, picotcp_header: &Path) {
    println!("Generating bindings for {}", picotcp_header.to_str().expect("Could not decode Path to String"));
    let mut bindings = Builder::new(picotcp_header.to_str().expect("Could not decode Path to String"));
    bindings.builtins();
    bindings.forbid_unknown_types();
    // bindings.link("picotcp", bindgen::LinkType::Static);
    // bindings.clang_arg("-L").clang_arg(picotcp_lib.to_str().expect("Could not decode Path to String"));
    let generated_bindings = bindings.generate().expect("Failed to generate bindings");
    let mut file = File::create(out_file).expect("Failed to open file");
    file.write(generated_bindings.to_string().as_bytes()).unwrap();
}

fn link_picotcp(picotcp_lib_dir: &Path) {
    println!("cargo:rustc-link-lib=static=picotcp");
    println!("cargo:rustc-link-search=native={}", picotcp_lib_dir.to_str().expect("Could not decode Path to String"));
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir).to_path_buf();
    let out_file: PathBuf = Path::new("picotcp.rs").to_path_buf();
    let picotcp_prefix: PathBuf = { let mut p = out_dir.clone(); p.push("picotcp"); p };
    let picotcp_header: PathBuf = { let mut p = picotcp_prefix.clone(); p.push("include/pico_stack.h"); p };
    let picotcp_lib_dir: PathBuf = { let mut p = picotcp_prefix.clone(); p.push("lib"); p };

    build_picotcp(picotcp_prefix.as_path());

    generate_bindings(out_file.as_path(), picotcp_header.as_path());

    link_picotcp(picotcp_lib_dir.as_path());
}
