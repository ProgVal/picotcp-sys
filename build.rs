extern crate bindgen;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fs::read_dir;
use std::path::PathBuf;
use std::process::Command;

use bindgen::Builder;

/// Run picoTCP's Makefile
fn build_picotcp(prefix: &Path) {
    println!("Building picotcp to: {}", prefix.to_string_lossy());
    Command::new("make")
            .arg("-C").arg("picotcp") // $RUSTOTCP_SOURCE/picotcp/
            .env("PREFIX", prefix.as_os_str())
            .status().unwrap();
}

fn generate_bindings(out_file: &mut File, header_path: &Path) {
    let header_path = header_path.to_str().expect("Could not decode Path to String");
    let mut bindings = Builder::new(header_path);
    bindings.builtins();
    bindings.forbid_unknown_types();
    // bindings.link("picotcp", bindgen::LinkType::Static);
    // bindings.clang_arg("-L").clang_arg(picotcp_lib.to_str().expect("Could not decode Path to String"));
    let generated_bindings = bindings.generate().expect(&format!("Failed to generate bindings for {}", header_path));

    out_file.write(generated_bindings.to_string().as_bytes()).unwrap();
}

fn write_includes(includer_file: &mut File, includer_file_path: &Path, path: &Path) {
    let file_name = path.file_name().unwrap().to_str().expect("Could not decode Path to String");


    if path.file_name().unwrap() == "arch" {
        // Files in include/arch/ are a bit hackish and do some do not work
        // with clang.
        // So we'll just skip them (for now).
    }
    else if path == includer_file_path {
        // Do not self-include.
    }
    else if path.is_dir() {
        for path in read_dir(path).unwrap() {
            write_includes(includer_file, includer_file_path, path.unwrap().path().as_path());
        }
    }
    else {
        // If it's a file, just include it.
        assert!(file_name.ends_with(".h"));
        includer_file.write(format!("#include \"{}\"\n", path.to_str().unwrap()).as_bytes()).unwrap();
    }
}

fn link_picotcp(picotcp_lib_dir: &Path) {
    println!("cargo:rustc-link-lib=static=picotcp");
    println!("cargo:rustc-link-search=native={}", picotcp_lib_dir.to_str().expect("Could not decode Path to String"));
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir).to_path_buf();
    let out_file_path: PathBuf = Path::new("picotcp.rs").to_path_buf();
    let picotcp_prefix: PathBuf = { let mut p = out_dir.clone(); p.push("picotcp"); p };
    let picotcp_headers_dir: PathBuf = { let mut p = picotcp_prefix.clone(); p.push("include"); p };
    let includer_file_path: PathBuf = { let mut p = picotcp_headers_dir.clone(); p.push("picotcp_include_all.h"); p };
    let picotcp_lib_dir: PathBuf = { let mut p = picotcp_prefix.clone(); p.push("lib"); p };
    let mut out_file = File::create(out_file_path).expect("Failed to open file");

    build_picotcp(picotcp_prefix.as_path());

    {
        let mut includer_file = File::create(includer_file_path.as_path()).expect("Failed to open file");
        write_includes(&mut includer_file, includer_file_path.as_path(), picotcp_headers_dir.as_path());
    }

    generate_bindings(&mut out_file, includer_file_path.as_path());

    link_picotcp(picotcp_lib_dir.as_path());
}
