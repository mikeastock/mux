use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use yaml_rust::ScanError;

use self::Error::{Io, YamlScan};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    YamlScan(ScanError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Io(ref error) => error.description(),
            YamlScan(ref error) => error.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Io(ref error) => Some(error),
            YamlScan(ref error) => Some(error),
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Io(err)
    }
}

impl From<ScanError> for Error {
    fn from(err: ScanError) -> Error {
        YamlScan(err)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error as StdError;
    use std::io;
    use yaml_rust::ScanError;
    use super::Error;
    use super::Error::*;

    macro_rules! from {
        ($from:expr => $error:pat) => {
            match Error::from($from) {
                e @ $error => {
                    assert!(e.description().len() > 5);
                } ,
                _ => panic!("{:?}", $from)
            }
        }
    }

    macro_rules! from_and_cause {
        ($from:expr => $error:pat) => {
            match Error::from($from) {
                e @ $error => {
                    let desc = e.cause().unwrap().description();
                    assert_eq!(desc, $from.description().to_owned());
                    assert_eq!(desc, e.description());
                },
                _ => panic!("{:?}", $from)
            }
        }
    }

    #[test]
    fn test_from() {
        from_and_cause!(io::Error::new(io::ErrorKind::Other, "other") => Io(..));
        // from_and_cause!(ScanError() => YamlScan(..));
    }
}
