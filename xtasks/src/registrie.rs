use xtasks::*;

use std::io;
use std::process::Command;

pub fn build() -> io::Result<()> {
    println!("[xtask]: Building registrie");
    Command::new("cargo")
        .arg("build")
        .args(["--package", "registrie"])
        .status()?
        .early_ret()?;

    Ok(())
}

pub fn serve() -> io::Result<()> {
    println!("[xtask]: Serving registrie");
    Command::new("cargo")
        .arg("run")
        .args(["--package", "registrie"])
        .status()?
        .early_ret()?;

    Ok(())
}
