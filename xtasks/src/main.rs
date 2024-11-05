use xtasks::*;

use clap::Parser;
use std::io;
use std::process::Command;

mod clientie;
mod registrie;

#[derive(Parser)]
enum Xtask {
    Make,
    ServeClientie,
    ServeRegistrie,
    WatchClientie,
    WatchRegistrie,
    QuickDevRegistrie,
}

fn main() -> io::Result<()> {
    let command = Xtask::parse();

    match command {
        Xtask::Make => {
            println!("[xtask]: Making the Colabie project, finally ;)");
            clientie::build()?;
            registrie::build()
        }
        Xtask::ServeClientie => {
            clientie::build()?;
            clientie::serve()
        }
        Xtask::ServeRegistrie => {
            registrie::build()?;
            registrie::serve()
        }
        Xtask::WatchClientie => watch("clientie/", "run --package xtasks serve-clientie"),
        Xtask::WatchRegistrie => watch("registrie/src", "run --package xtasks serve-registrie"),
        Xtask::QuickDevRegistrie => watch(
            "registrie/tests",
            "test --package registrie -q quick_dev -- --nocapture",
        ),
    }
}

fn watch(dir: &str, run_command: &str) -> io::Result<()> {
    // Install cargo-watch if it's not already installed
    if Command::new("cargo")
        .args(["watch", "--version"])
        .status()?
        .early_ret()
        .is_err()
    {
        println!("[xtask]: Installing cargo-watch");
        Command::new("cargo")
            .args(["install", "cargo-watch"])
            .status()?
            .early_ret()?;
    }

    Command::new("cargo")
        .arg("watch")
        .arg("-qc")
        .args(["-w", dir])
        .args(["-x", run_command])
        .status()?;

    Ok(())
}
