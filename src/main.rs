use anyhow::Result;
use clap::Command;

mod errors;

mod ash;
mod jb;
mod kristian;

fn main() -> Result<()> {
    let matches = Command::new("jbash-cli")
        .version("0.1.0")
        .about("Rust CLI Capstone for JB and Ash")
        .subcommand(Command::new("jb").about("JB's work"))
        .subcommand(Command::new("ash").about("Ash's work"))
        .subcommand(Command::new("kristian").about("Kristian's work"))
        .get_matches();

    if matches.subcommand_matches("jb").is_some() {
        jb::run()?;
        return Ok(());
    }

    if matches.subcommand_matches("ash").is_some() {
        ash::run()?;
        return Ok(());
    }

    if matches.subcommand_matches("kristian").is_some() {
        kristian::run()?;
        return Ok(());
    }

    Ok(())
}
