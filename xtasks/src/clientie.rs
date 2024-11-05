use xtasks::*;

use std::path::Path;
use std::process::Command;
use std::{fs, io};

const WEB_OUTPUT_DIR: &str = "target/web";

pub fn build() -> io::Result<()> {
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
    fs::rename("clientie/pkg", Path::new(WEB_OUTPUT_DIR).join("wasm"))?;

    Ok(())
}

pub fn serve() -> io::Result<()> {
    // Install static-web-server if it's not already installed
    if Command::new("static-web-server")
        .arg("--version")
        .status()
        .is_err()
    {
        println!("[xtask]: Installing static-web-server");
        Command::new("cargo")
            .args(["install", "static-web-server"])
            .status()?
            .early_ret()?;
    }

    println!("[xtask]: Starting static-web-server at http://localhost:8080");
    Command::new("static-web-server")
        .args(["--host", "0.0.0.0"])
        .args(["--port", "8080"])
        .args(["--root", "./target/web"])
        .status()?;

    Ok(())
}
