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

/// Write bindings for a single picoTCP header to the provided `File`.
fn write_file_bindings(out_file: &mut File, header_path: &Path) {
    let header_path = header_path.to_str().expect("Could not decode Path to String");
    let mut bindings = Builder::new(header_path);
    bindings.builtins();
    //bindings.forbid_unknown_types();
    // bindings.link("picotcp", bindgen::LinkType::Static);
    // bindings.clang_arg("-L").clang_arg(picotcp_lib.to_str().expect("Could not decode Path to String"));
    let generated_bindings = bindings.generate().expect(&format!("Failed to generate bindings for {}", header_path));

    out_file.write(generated_bindings.to_string().as_bytes()).unwrap();
}

/// Write a file or a directory's bindings, wrapped in `mod { }`
fn write_bindings(out_file: &mut File, headers_path: &Path) {
    let file_name = headers_path.file_name().unwrap().to_str().expect("Could not decode Path to String");


    if headers_path.file_name().unwrap() == "arch" {
        // Files in include/arch/ are a bit hackish and do some do not work
        // with clang.
        // So we'll just skip them (for now).
    }
    else if headers_path.is_dir() {
        // If it's a directory, write its bindings recursively.
        let module = file_name;
        out_file.write(format!("pub mod {} {{\n", module).as_bytes()).unwrap();
        println!("Generating bindings for module: {}", module);

        for path in read_dir(headers_path).unwrap() {
            write_bindings(out_file, path.unwrap().path().as_path());
        }

        out_file.write(b"}").unwrap();
    }
    else {
        // If it's a file, just write its bindings.
        assert!(file_name.ends_with(".h"));
        let module = &file_name[0..file_name.len()-2];
        out_file.write(format!("pub mod {} {{\n", module).as_bytes()).unwrap();
        println!("Generating bindings for module: {}", module);

        write_file_bindings(out_file, headers_path);

        out_file.write(b"}\n\n").unwrap();
    }

}

fn generate_bindings(out_file: &mut File, headers_path: &Path) {
    println!("Generating bindings for {}", headers_path.to_str().expect("Could not decode Path to String"));

    for path in read_dir(headers_path).unwrap() {
        write_bindings(out_file, path.unwrap().path().as_path());
    }
}

fn link_picotcp(picotcp_lib_dir: &Path) {
    println!("cargo:rustc-link-lib=static=picotcp");
    println!("cargo:rustc-link-search=native={}", picotcp_lib_dir.to_str().expect("Could not decode Path to String"));
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir).to_path_buf();
    let out_file_name: PathBuf = Path::new("picotcp.rs").to_path_buf();
    let picotcp_prefix: PathBuf = { let mut p = out_dir.clone(); p.push("picotcp"); p };
    let picotcp_headers_dir: PathBuf = { let mut p = picotcp_prefix.clone(); p.push("include"); p };
    let picotcp_lib_dir: PathBuf = { let mut p = picotcp_prefix.clone(); p.push("lib"); p };
    let mut out_file = File::create(out_file_name).expect("Failed to open file");

    //build_picotcp(picotcp_prefix.as_path());

    generate_bindings(&mut out_file, picotcp_headers_dir.as_path());

    link_picotcp(picotcp_lib_dir.as_path());
}
