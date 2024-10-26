use std::io;
use std::process::Command;
use xtasks::*;

fn main() -> io::Result<()> {
    xtasks::build_clientie()?;

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
