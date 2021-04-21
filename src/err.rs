use std::{error, fmt, io};

#[derive(Debug)]
#[non_exhaustive]
pub enum CliError {
    IO(io::Error),
    Network(ureq::Error),
    GitIgnoreNotFound(String),
    OverwriteFile,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::IO(io_err) => io_err.fmt(f),
            CliError::Network(network_err) => network_err.fmt(f),
            CliError::GitIgnoreNotFound(search_keyword) => {
                write!(f, "Gitignore for {} not found", search_keyword)
            }
            CliError::OverwriteFile => write!(
                f,
                ".gitignore exists in current working directory. Use `--write-force` to overwrite"
            ),
        }
    }
}

impl error::Error for CliError {}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> Self {
        CliError::IO(err)
    }
}

impl From<ureq::Error> for CliError {
    fn from(err: ureq::Error) -> Self {
        CliError::Network(err)
    }
}
