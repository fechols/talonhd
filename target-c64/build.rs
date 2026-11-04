use std::{env, path::PathBuf, process::Command};

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let compiler = env::var("MOS_CLANG").unwrap_or_else(|_| "mos-c64-clang".into());
    let mut sources = vec!["startup", "reu", "blit", "hud", "digital", "mem", "bands", "border", "arith"];
    // Unit-cost microbenchmarks (feature `bench`).
    if env::var_os("CARGO_FEATURE_BENCH").is_some() {
        sources.push("bench");
    }
    for name in sources {
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
    // Paged phases share the window's addresses (link.ld); region overflows are still reported.
    println!("cargo:rustc-link-arg=-Wl,--no-check-sections");
    // Integer libcalls go to asm/arith.S's `__wrap___X` (one u8 and one u16 divide core, shift-and-add
    // multiplies, in place of libcrt's larger ones). libcrt keeps every divide in one archive member, so
    // defining `__X` itself would be a duplicate symbol; --wrap also catches the libcalls LLVM generates
    // during LTO. tools/test_arith.py checks them.
    for helper in [
        "__udivqi3", "__umodqi3", "__udivmodqi4", "__udivhi3", "__umodhi3", "__divhi3", "__modhi3", "__mulqi3",
        "__mulhi3",
    ] {
        println!("cargo:rustc-link-arg=-Wl,--wrap={helper}");
    }
    println!("cargo:rustc-link-arg=-T");
    println!("cargo:rustc-link-arg={}", root.join("link.ld").display());
    println!("cargo:rustc-link-arg=-Wl,-Map,{}", out.join("talonhd.map").display());
}
