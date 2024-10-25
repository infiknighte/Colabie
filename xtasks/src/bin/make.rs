use std::path::Path;
use std::process::{Command, ExitStatus};
use std::{fs, io};

const WEB_OUTPUT_DIR: &str = "target/web";

fn main() -> io::Result<()> {
    println!("[MAKE]: Making the Colabie project, finally ;)");

    // Install wasm-pack if it's not already installed
    if Command::new("wasm-pack").arg("--version").status().is_err() {
        println!("[MAKE]: Installing wasm-pack");
        Command::new("cargo")
            .args(["install", "wasm-pack"])
            .status()?
            .early_ret()?;
    }

    // Build Clientie
    println!("[MAKE]: Building clientie");
    Command::new("wasm-pack")
        .arg("build")
        .arg("--no-typescript")
        .args(["--target", "web"])
        .arg("clientie")
        .status()?
        .early_ret()?;

    // Move the web files
    println!("[MAKE]: Moving clientie web files to target/web");
    _ = fs::remove_dir_all(WEB_OUTPUT_DIR);
    copy_dir_all("clientie/web", WEB_OUTPUT_DIR)?;
    fs::rename("clientie/pkg", Path::new(WEB_OUTPUT_DIR).join("pkg"))?;

    Ok(())
}

trait EarlyRet<E>: Sized {
    fn early_ret(self) -> Result<Self, E>;
}

impl EarlyRet<io::Error> for ExitStatus {
    fn early_ret(self) -> io::Result<Self> {
        match self.success() {
            true => Ok(self),
            false => Err(io::Error::new(io::ErrorKind::Other, "Command failed")),
        }
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
