use std::{error, fmt, io};

#[derive(Debug)]
#[non_exhaustive]
pub enum CliError {
    IO(io::Error),
    Network,
    OverwriteFile,
    Unknown(String)
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::IO(io_err) => io_err.fmt(f),
            CliError::Network => write!(f, "Network error"),
            CliError::OverwriteFile => write!(f, ".gitignore exists in current working directory. Use `--write-force` to overwrite"),
            CliError::Unknown(str) => write!(f, "Unknown err: {}", str)
        }
    }
}

impl error::Error for CliError {}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::IO(err)
    }
}