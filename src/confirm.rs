use amqp_types::frame::{Arguments, ConfirmSelectOk, ConfirmSelect};
use crate::error::FrameDecodeErr;
use nom::error::ErrorKind;
use nom::number::complete::be_u8;

pub(crate) fn parse_confirm_select(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let confirm_select = ConfirmSelect::default();
    if 1 == (flags & (1 << 0)) {
        confirm_select.set_no_wait(true);
    }
    Ok(Arguments::ConfirmSelect(confirm_select))
}

pub(crate) fn parse_confirm_select_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let confirm_select_ok = ConfirmSelectOk::default();
    confirm_select_ok.set_dummy(dummy);
    Ok(Arguments::ConfirmSelectOk(confirm_select_ok))
}