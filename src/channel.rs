use crate::common::{get_method_type, parse_short_string, parse_long_string};
use amqp_types::frame::{ChannelClose, Arguments, ChannelCloseOk, Class, ChannelFlowOk, ChannelFlow, ChannelOpenOk, ChannelOpen};
use crate::error::FrameDecodeErr;
use nom::error::ErrorKind;
use nom::number::complete::{be_u8, be_u16};

pub(crate) fn parse_channel_open(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, out_of_band) = match parse_short_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let channel_open = ChannelOpen::default();
    channel_open.set_out_of_band(out_of_band);
    Ok(Arguments::ChannelOpen(channel_open))
}


pub(crate) fn parse_channel_open_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, channel_id) = match parse_long_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let channel_open_ok = ChannelOpenOk::default();
    channel_open_ok.set_channel_id(channel_id);
    Ok(Arguments::ChannelOpenOk(channel_open_ok))
}

pub(crate) fn parse_channel_flow(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, active) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok((buffer, value)) => (buffer, if let 0 = value { false } else { true }),
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let channel_flow = ChannelFlow::default();
    channel_flow.set_active(active);
    Ok(Arguments::ChannelFlow(channel_flow))
}

pub(crate) fn parse_channel_flow_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, active) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok((buffer, value)) => (buffer, if let 0 = value { false } else { true }),
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let channel_flow = ChannelFlowOk::default();
    channel_flow.set_active(active);
    Ok(Arguments::ChannelFlowOk(channel_flow))
}

pub(crate) fn parse_channel_close(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, reply_code) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, reply_text) = match parse_short_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, class_id) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let class_type = Class::from(class_id);
    if let Class::Unknown = class_type {
        return Err(FrameDecodeErr::ParseFrameFailed);
    }
    let (_, method_id) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let method_type = match get_method_type(class_type.clone(), method_id) {
        Ok(method_type) => method_type,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let channel_close = ChannelClose::default();
    channel_close.set_reply_code(reply_code);
    channel_close.set_reply_text(reply_text);
    channel_close.set_class(class_type);
    channel_close.set_method(method_type);
    Ok(Arguments::ChannelClose(channel_close))
}

pub(crate) fn parse_channel_close_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let channel_close_ok = ChannelCloseOk::default();
    channel_close_ok.set_dummy(dummy);
    Ok(Arguments::ChannelCloseOk(channel_close_ok))
}
