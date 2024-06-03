/// thisweek-cli
use crate::errors::*;
use anyhow::Result;
use clap::ArgMatches;

pub fn run(subcommands: &ArgMatches) -> Result<()> {
    if subcommands.subcommand_matches("add").is_some() {
        println!("Adding new task...");
        return Err(KristianError::WorkInProgressPaSer.into());
    }

    if subcommands.subcommand_matches("delete").is_some() {
        println!("Deleting new task...");
        return Err(KristianError::WorkInProgressPaSer.into());
    }

    Ok(())
}
