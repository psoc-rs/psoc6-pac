use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
fn main() {
    let target = env::var("TARGET").unwrap();
    let host_triple = env::var("HOST").unwrap();

    if host_triple == target {
        println!("cargo:rustc-cfg=native");
    }
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        if target.starts_with("thumbv6m-") {
           File::create(out.join("device.x"))
                .unwrap()
                .write_all(include_bytes!("device.x"))
                .unwrap();
            println!("cargo:rustc-cfg=armv6m");
            //when compiling for the CM4 core use a different device.x
        }else if target.starts_with("thumbv7em-") {           
             File::create(out.join("device.x"))
                .unwrap()
                .write_all(include_bytes!("device_cm4.x"))
                .unwrap();
            println!("cargo:rustc-cfg=armv7em");
        }
        println!("cargo:rerun-if-changed=build.rs");
    }
}
