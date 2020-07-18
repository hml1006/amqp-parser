use std::{io, fmt};
use std::fmt::{Display, Formatter};
use std::io::Error;

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
            FrameDecodeErr::Incomplete => write!(f, "Incomplete"),
            FrameDecodeErr::UnknowFrameType => write!(f, "unknow frame type"),
            FrameDecodeErr::UnknownClassType => write!(f, "unknown class type"),
            FrameDecodeErr::UnknownMethodType => write!(f, "unknown method type"),
            FrameDecodeErr::ParseAmqpHeaderFailed => write!(f, "parse Amqp header failed"),
            FrameDecodeErr::ParseFrameFailed => write!(f, "parse frame failed"),
            FrameDecodeErr::Amqp(err) => write!(f, "amqp error: {}", err),
            FrameDecodeErr::Io(err) => write!(f, "{}", err)
        }
    }
}

impl From<io::Error> for FrameDecodeErr {
    fn from(err: Error) -> Self {
        FrameDecodeErr::Io(err)
    }
}
