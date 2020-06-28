use std::fmt;
use nom::lib::std::fmt::Formatter;

pub enum ErrorKind {
    NeedContinue,
    ReadProtocolHeaderTagFailed,
    FrameError,
    WrongProtocol,
    VersionNotSupported
}

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        match self {
            ErrorKind::NeedContinue => "NeedContinue",
            ErrorKind::ReadProtocolHeaderTagFailed => "Read protocol header tag failed",
            ErrorKind::FrameError => "Frame error",
            ErrorKind::WrongProtocol => "Wrong protocol name",
            ErrorKind::VersionNotSupported => "Version not supported"
        }.parse().unwrap()
    }
}

pub struct Error {
    kind: ErrorKind
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error{ kind}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind.to_string())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind.to_string())
    }
}

impl std::error::Error for Error {

}