use anyhow::Result;
use clap::{Arg, Command};

mod errors;

mod ash;
mod jb;
mod kristian;

fn main() -> Result<()> {
    let matches = Command::new("jbash-cli")
        .version("0.1.0")
        .about("Rust CLI Capstone for JB and Ash")
        // JB feel free to modify this for your flags
        .subcommand(Command::new("jb").about("JB's work"))
        // Ash feel free to modify this for your flags
        .subcommand(Command::new("ash").about("Ash's work"))
        .subcommand(
            Command::new("kristian")
                .about("thisweek...?")
                .subcommand(
                    Command::new("add")
                        .about("Add a new <TASK> for this week")
                        .arg(Arg::new("TASK").long("task").short('t')),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete a task with <TASK_ID>")
                        .arg(Arg::new("TASK_ID").long("task-id").short('i')),
                ),
        )
        .get_matches();

    if matches.subcommand_matches("jb").is_some() {
        jb::run()?;
        return Ok(());
    }

    if matches.subcommand_matches("ash").is_some() {
        ash::run()?;
        return Ok(());
    }

    if let Some(cmd) = matches.subcommand_matches("kristian") {
        kristian::run(cmd)?;
        return Ok(());
    }

    Ok(())
}
