use std::{io, fmt};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum FrameDecodeErr {
    Incomplete,
    UnknowFrameType,
    UnknownClassType,
    UnknownMethodType,
    ParseAmqpHeaderFailed,
    ParseFrameFailed,
    Amqp(amqp_types::error::Error),
    Io(io::Error)
}

impl Display for FrameDecodeErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FrameDecodeErr::ParseFrameFailed => write!(f, "parse frame failed"),
            FrameDecodeErr::Io(err) => write!(f, "{}", err)
        }
    }
}

impl std::error::Error for FrameDecodeErr {}