pub mod error {
    use std::{io, result};

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Error {
        IOError(io::Error),
        HttpParseError(httparse::Error),
        SerdeError(serde_json::Error),
        HttpRequestTooShort,
        MissingPath,
    }
    pub type Result<T> = result::Result<T, Error>;
    impl From<io::Error> for Error {
        fn from(value: io::Error) -> Self {
            Error::IOError(value)
        }
    }
    impl From<httparse::Error> for Error {
        fn from(value: httparse::Error) -> Self {
            Error::HttpParseError(value)
        }
    }

    impl From<serde_json::Error> for Error {
        fn from(value: serde_json::Error) -> Self {
            Error::SerdeError(value)
        }
    }
}

