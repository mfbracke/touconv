use quick_xml::Error as QxError;
use std::io::Error as IoError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error during parsing")]
    Parse(ParseError),
    #[error("error while processing arguments")]
    Args(ArgsError),
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("input/output error")]
    Io { source: std::io::Error },
    #[error("invalid input ({explanation})")]
    InvalidInput { explanation: String },
}

#[derive(thiserror::Error, Debug)]
pub enum ArgsError {
    #[error("input/output error")]
    Io { source: std::io::Error },
    #[error("there is no parser corresponding to the specified format")]
    NoSuchParser,
    #[error("there is no writer corresponding to the specified format")]
    NoSuchWriter,
}

impl From<IoError> for ArgsError {
    fn from(err: IoError) -> Self {
        ArgsError::Io { source: err }
    }
}

impl From<QxError> for ParseError {
    fn from(err: QxError) -> Self {
        match err {
            QxError::Io(error) => ParseError::Io { source: error },
            _ => ParseError::InvalidInput {
                explanation: format!("{:#?}", err),
            },
        }
    }
}
