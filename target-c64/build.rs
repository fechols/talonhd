use std::{env, path::PathBuf, process::Command};

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let compiler = env::var("MOS_CLANG").unwrap_or_else(|_| "mos-c64-clang".into());
    for name in ["startup", "reu", "blit", "hud"] {
        let source = root.join(format!("asm/{name}.S"));
        println!("cargo:rerun-if-changed={}", source.display());
        let object = out.join(format!("{name}.o"));
        let status = Command::new(&compiler)
            .args(["-c", "-g", "-o"])
            .arg(&object)
            .arg(&source)
            .status()
            .expect("cannot run mos-c64-clang");
        assert!(status.success(), "assembly failed: {name}");
        println!("cargo:rustc-link-arg={}", object.display());
    }
    println!("cargo:rerun-if-changed=link.ld");
    println!("cargo:rerun-if-env-changed=MOS_CLANG");
    // Pass -T to clang directly (not via -Wl) so it replaces the default script.
    println!("cargo:rustc-link-arg=-T");
    println!("cargo:rustc-link-arg={}", root.join("link.ld").display());
    println!("cargo:rustc-link-arg=-Wl,-Map,{}", out.join("talonhd.map").display());
}
