use thiserror::Error;

#[derive(Error, Debug)]
pub enum HackathonError {
    #[error("di gumawa si {0}")]
    NoSubmission(String),
}

#[derive(Error, Debug)]
pub enum JBError {
    #[error("sample JB error")]
    SampleJbError,
}

#[derive(Error, Debug)]
pub enum AshError {
    #[error("sample Ash error")]
    SampleAshError,
}

#[derive(Error, Debug)]
pub enum KristianError {
    #[error("sample Kristian error")]
    SampleKristianError,
    #[error("ginagawa palang")]
    WorkInProgressPaSer,
}
