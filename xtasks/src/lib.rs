pub mod utils;
pub use utils::*;

use std::path::Path;
use std::process::Command;
use std::{fs, io};

const WEB_OUTPUT_DIR: &str = "target/web";

pub fn build_clientie() -> io::Result<()> {
    // Install wasm-pack if it's not already installed
    if Command::new("wasm-pack").arg("--version").status().is_err() {
        println!("[xtask]: Installing wasm-pack");
        Command::new("cargo")
            .args(["install", "wasm-pack"])
            .status()?
            .early_ret()?;
    }

    // Build Clientie
    println!("[xtask]: Building clientie");
    Command::new("wasm-pack")
        .arg("build")
        .arg("--no-typescript")
        .args(["--target", "web"])
        .arg("clientie")
        .status()?
        .early_ret()?;

    // Move the web files
    println!("[xtask]: Moving clientie web files to target/web");
    _ = fs::remove_dir_all(WEB_OUTPUT_DIR);
    copy_dir_all("clientie/web", WEB_OUTPUT_DIR)?;
    fs::rename("clientie/pkg", Path::new(WEB_OUTPUT_DIR).join("pkg"))?;

    Ok(())
}
