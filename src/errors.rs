use thiserror::Error;

#[derive(Error, Debug)]
pub enum HackathonError {
    #[error("di gumawa si {0}")]
    NoSubmission(String),
}
