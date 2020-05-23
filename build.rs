use std::env;
use std::ffi::OsStr;
use std::fs;
use std::process::Command;
use std::str;

#[cfg(target_os = "macos")]
fn configure_macos() {
    let wine_prefix = Command::new("brew")
        .args(&["--prefix", "wine"])
        .output()
        .expect("Could not execute `brew --prefix wine`. Is Homebrew installed?")
        .stdout;

    // output should have trailing newline
    assert!(wine_prefix[wine_prefix.len() - 1] == b'\n');

    // get rid of trailing newline
    let wine_prefix = &wine_prefix[0..(wine_prefix.len() - 1)];

    // always link to 64 bit wine on macOS
    println!("cargo:rustc-link-search={}/lib64", str::from_utf8(wine_prefix).unwrap());

    println!("cargo:rustc-link-lib=wine");
}

fn main() {
    #[cfg(target_os = "macos")]
    configure_macos();

    // let current_dir = env::current_dir();
    // let out_dir = env::var("OUT_DIR");

    // let mut wine_source_dir = current_dir.clone();
    // wine_source_dir.push("wine");

    // let mut wine_build_dir = out_dir.clone();
    // wine_build_dir.push("wine-build");

    // let mut wine_install_dir = out_dir.clone();
    // wine_install_dir.push("wine");

    // fs::create_dir(wine_build_dir).unwrap();

    // Command::new("bash")
    //     .arg("./configure")
}
