use bytes::Bytes;
use amqp_types::frame::ProtocolHeader;
use nom::{Err, Needed, IResult, tag};
use crate::{error};
use crate::util::{read_tag, read_u8};

const PROTOCOL_HEADER_SIZE: usize = 8;

// parse protocol header
pub fn parse_amqp_protocal_header(buffer: &[u8]) -> Result<ProtocolHeader, Box<dyn std::error::Error>> {
    if buffer.len() < PROTOCOL_HEADER_SIZE {
        return Err(Box::new(error::Error::new(error::ErrorKind::NeedContinue)))
    }

    let mut header = ProtocolHeader::default();
    let (buffer, protocol) = match read_tag(buffer, b"AMQP"){
        Ok(ret) => ret,
        Err(_) => return Err(Box::new(error::Error::new(error::ErrorKind::FrameError)))
    };
    header.set_protocol(Vec::from(protocol));
    let (buffer, major_id) = match read_u8(buffer) {
        Ok(ret) => ret,
        Err(_) => return Err(Box::new(error::Error::new(error::ErrorKind::FrameError)))
    };
    header.set_major_id(major_id);
    let (buffer, minor_id) = match read_u8(buffer) {
        Ok(ret) => ret,
        Err(_) => return Err(Box::new(error::Error::new(error::ErrorKind::FrameError)))
    };
    header.set_minor_id(minor_id);
    let (buffer, major_version) = match read_u8(buffer) {
        Ok(ret) => ret,
        Err(_) => return Err(Box::new(error::Error::new(error::ErrorKind::FrameError)))
    };
    header.set_major_version(major_version);
    let (_, minor_version) = match read_u8(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(Box::new(error::Error::new(error::ErrorKind::FrameError)))
    };
    header.set_minor_version(minor_version);
    Ok(header)
}

