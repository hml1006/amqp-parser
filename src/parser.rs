use bytes::Bytes;
use amqp_types::frame::ProtocolHeader;
use nom::{Err, Needed, IResult};
use crate::error;
use std::error::Error;
use nom::bytes::streaming::take;

pub fn parse_amqp_protocal_header(buffer: &[u8]) -> Result<ProtocolHeader, Err<Box<dyn Error>>> {
    if buffer.len() < std::mem::size_of::<ProtocolHeader>() {
        return Err(Err::Incomplete(Needed::Size(std::mem::size_of::<ProtocolHeader>() - buffer.len())))
    }

    let mut header = ProtocolHeader::default();
    let protocol = match take(4)(buffer) {
        Ok((_, protocol)) => protocol,
        Err(e) => return Err(e)
    };
    header.set_protocol(protocol);

}