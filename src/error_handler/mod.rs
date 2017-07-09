use std::io::{self, Write};
use std::error::Error;
use std::fmt;


pub type PropResult<T> = Result<T, PropError>;


#[derive(Debug)]
pub struct PropError {
    kind: PropErrorKind,
    code: u8,
    description: String,
    specific: String,
}

impl PropError {
    pub fn of(kind: PropErrorKind) -> Self {
        let code = kind.code();
        let desc = kind.description().to_owned();

        PropError {
            kind: kind,
            code: code,
            description: desc,
            specific: String::new(),
        }
    }

    pub fn with_specific(kind: PropErrorKind, specific: &str) -> Self {
        let err_code = kind.code();
        let desc = kind.description().to_owned();
        let specific = specific.to_owned();

        PropError {
            kind: kind,
            code: err_code,
            description: desc,
            specific: specific,
        }
    }

    pub fn kind(&self) -> &PropErrorKind {
        &self.kind
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn specific(&self) -> &str {
        &self.specific
    }

    pub fn code(&self) -> u8 {
        self.code
    }
}

impl fmt::Display for PropError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,
                 "PropError {{ kind: {:?}, code: {}, description: {:?} }}",
                 self.kind,
                 self.code,
                 self.description)
    }
}

impl ::std::convert::From<::std::io::Error> for PropError {
    fn from(e: ::std::io::Error) -> Self {
        PropError::with_specific(PropErrorKind::ReaderError, e.description())
    }
}


#[derive(Debug, PartialEq)]
pub enum PropErrorKind {
    InsufficientArgs,
    ReaderError,
    ParseError,
}

impl PropErrorKind {
    fn code(&self) -> u8 {
        match *self {
            PropErrorKind::InsufficientArgs => 1,
            PropErrorKind::ReaderError => 2,
            PropErrorKind::ParseError => 3,
        }
    }

    fn description(&self) -> &str {
        match *self {
            PropErrorKind::InsufficientArgs => "insufficient number of arguments",
            PropErrorKind::ReaderError => "could not read the property file",
            PropErrorKind::ParseError => "could not parse the property file",
        }
    }
}


/// display usage
pub fn display_usage() {
    writeln!(io::stderr(),
             "Usage: jpropparser property-file [property-file]*")
        .unwrap();
}

/// exit normally
pub fn exit_normal() {
    ::std::process::exit(0);
}

/// exit with error code
pub fn exit_error(err: PropError) {
    writeln!(io::stderr(),
             "description: {}, specific: {}, error code: {}",
             err.description(),
             err.specific(),
             err.code())
        .unwrap();
    ::std::process::exit(err.code() as i32);
}
