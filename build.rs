use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("nasm").args(&["src/boot.asm", "-felf64", "-o"])
                       .arg(&format!("{}/boot.o", out_dir))
                       .status().unwrap();

    Command::new("ar").args(&["crs", "libboot.a", "boot.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=boot");
    println!("cargo:rerun-if-changed=src/boot.asm");
}