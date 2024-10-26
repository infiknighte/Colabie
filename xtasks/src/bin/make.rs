use std::io;

fn main() -> io::Result<()> {
    println!("[xtask]: Making the Colabie project, finally ;)");
    xtasks::build_clientie()?;

    Ok(())
}
