/// thisweek-cli
use crate::errors::*;
use anyhow::Result;

pub fn run() -> Result<()> {
    Err(HackathonError::NoSubmission(String::from("Kristian")).into())
}
