use crate::error::FrameDecodeErr;
use amqp_types::frame::{Arguments, ConnectionClose, Class, ConnectionOpenOk, ConnectionOpen, ConnectionTuneOk, ConnectionTune, ConnectionSecureOk, ConnectionSecure, ConnectionStartOk, ConnectionCloseOk, Property, ConnectionProperties};
use nom::error::ErrorKind;
use nom::number::complete::{be_u8, be_u16, be_u32};
use crate::common::{get_method_type, parse_short_string, parse_long_string, parse_field_table};
use amqp_types::ConnectionStart;

pub(crate) fn parse_connection_start(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, major_version) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, minor_version) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, server_properties) = match parse_field_table(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, mechanisms) = match parse_long_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, locales) = match parse_long_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let connection_start = ConnectionStart::default();
    connection_start.set_version_major(major_version);
    connection_start.set_version_minor(minor_version);
    connection_start.set_server_properties(server_properties);
    connection_start.set_mechanisms(mechanisms);
    connection_start.set_locales(locales);

    Ok(Arguments::ConnectionStart(connection_start))
}

pub(crate) fn parse_connection_start_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, client_properties) = match parse_field_table(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, mechanism) = match parse_short_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, response) = match parse_long_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, locale) = match parse_short_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let connection_start_ok = ConnectionStartOk::default();
    connection_start_ok.set_client_properties(client_properties);
    connection_start_ok.set_mechanism(mechanism);
    connection_start_ok.set_response(response);
    connection_start_ok.set_locale(locale);

    Ok(Arguments::ConnectionStartOk(connection_start_ok))
}

pub(crate) fn parse_connection_secure(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, challenge) = match parse_long_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let connection_secure = ConnectionSecure::default();
    connection_secure.set_challenge(challenge);
    Ok(Arguments::ConnectionSecure(connection_secure))
}

pub(crate) fn parse_connection_secure_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, response) = match parse_long_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let connection_secure_ok = ConnectionSecureOk::default();
    connection_secure_ok.set_response(response);
    Ok(Arguments::ConnectionSecureOk(connection_secure_ok))
}

pub(crate) fn parse_connection_tune(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, channel_max) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, frame_max) = match be_u32::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, heartbeat) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let connection_tune = ConnectionTune::default();
    connection_tune.set_channel_max(channel_max);
    connection_tune.set_frame_max(frame_max);
    connection_tune.set_heartbeat(heartbeat);
    Ok(Arguments::ConnectionTune(connection_tune))
}

pub(crate) fn parse_connection_tune_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, channel_max) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, frame_max) = match be_u32::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, heartbeat) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let connection_tune_ok = ConnectionTuneOk::default();
    connection_tune_ok.set_channel_max(channel_max);
    connection_tune_ok.set_frame_max(frame_max);
    connection_tune_ok.set_heartbeat(heartbeat);
    Ok(Arguments::ConnectionTuneOk(connection_tune_ok))
}

pub(crate) fn parse_connection_open(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, vhost) = match parse_short_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, capabilities) = match parse_short_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, insist) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok((buffer, value)) => (buffer, if let 0 = value { false } else { true }),
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let connection_open = ConnectionOpen::default();
    connection_open.set_vhost(vhost);
    connection_open.set_capabilities(capabilities);
    connection_open.set_insist(insist);
    Ok(Arguments::ConnectionOpen(connection_open))
}

pub(crate) fn parse_connection_open_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, known_hosts) = match parse_short_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let connection_open_ok = ConnectionOpenOk::default();
    connection_open_ok.set_known_hosts(known_hosts);
    Ok(Arguments::ConnectionOpenOk(connection_open_ok))
}

pub(crate) fn parse_connection_close(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
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
    let connection_close = ConnectionClose::default();
    connection_close.set_reply_code(reply_code);
    connection_close.set_reply_text(reply_text);
    connection_close.set_class(class_type);
    connection_close.set_method(method_type);
    Ok(Arguments::ConnectionClose(connection_close))
}

pub(crate) fn parse_connection_close_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let connection_close_ok = ConnectionCloseOk::default();
    connection_close_ok.set_dummy(dummy);
    Ok(Arguments::ConnectionCloseOk(connection_close_ok))
}

pub(crate) fn parse_connection_properties(buffer: &[u8]) -> Result<Property, FrameDecodeErr> {
    let (buffer, flags) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let property = ConnectionProperties::default();
    property.set_flags(flags);
    Ok(Property::Connection(property))
}