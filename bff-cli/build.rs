use std::env;

use shadow_rs::{BuildPattern, ShadowBuilder};

fn main() {
    ShadowBuilder::builder()
        .build_pattern(BuildPattern::Lazy)
        .build()
        .unwrap();

    set_windows_exe_options();
}

// https://github.com/rust-lang/rust/commit/5e6bb83268518dcd74c96b5504f485b71e604e4c
// https://github.com/BurntSushi/ripgrep/commit/db6bb21a629d5b1ec1bfe89c693b280497c9eedc
// Add a manifest file to bff-cli.exe.
fn set_windows_exe_options() {
    static MANIFEST: &str = "pkg/windows/Manifest.xml";

    let Ok(target_os) = env::var("CARGO_CFG_TARGET_OS") else { return };
    let Ok(target_env) = env::var("CARGO_CFG_TARGET_ENV") else { return };
    if !(target_os == "windows" && target_env == "msvc") {
        return;
    }

    let mut manifest = env::current_dir().unwrap();
    manifest.push(MANIFEST);
    let manifest = manifest.to_str().unwrap();

    println!("cargo:rerun-if-changed={}", MANIFEST);
    // Embed the Windows application manifest file.
    println!("cargo:rustc-link-arg-bin=bff-cli=/MANIFEST:EMBED");
    println!("cargo:rustc-link-arg-bin=bff-cli=/MANIFESTINPUT:{manifest}");
    // Turn linker warnings into errors.
    println!("cargo:rustc-link-arg-bin=bff-cli=/WX");
}
