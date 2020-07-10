use amqp_types::frame::{Arguments, AccessRequest, AccessRequestOk};
use crate::error::FrameDecodeErr;
use crate::common::parse_short_string;
use nom::error::ErrorKind;
use nom::number::complete::{be_u8, be_u16};

pub(crate) fn parse_access_request(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, realm) = match parse_short_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (_, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let access_request = AccessRequest::default();
    access_request.set_realm(realm);
    Ok(Arguments::AccessRequest(access_request))
}

pub(crate) fn parse_access_request_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, ticket) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let mut access_request_ok = AccessRequestOk::default();
    access_request_ok.set_ticket(ticket);
    Ok(Arguments::AccessRequestOk(access_request_ok))
}