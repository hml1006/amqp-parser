use bytes::Bytes;
use amqp_types::frame::{ProtocolHeader, Arguments, Class, Method, ConnectionMethod, ChannelMethod, AccessMethod, ExchangeMethod, QueueMethod, BasicMethod, TxMethod, ConnectionStartOk, ConnectionSecure, ConnectionSecureOk, ConnectionTune, ConnectionTuneOk, ConnectionOpen, ConnectionOpenOk, ConnectionClose, ConnectionCloseOk, ChannelOpen, ChannelOpenOk, ChannelFlow};
use nom::{Err, Needed, IResult, tag};
use crate::{error};
use amqp_types::{Frame, FrameType, ShortStr, LongStr, FieldArray, FieldTable, FieldValue, FieldName, Decimal, ConnectionStart};
use nom::number::complete::{be_u16, be_u32, be_i16, be_i8, be_u8, be_i32, be_u64, be_i64, be_f32, be_f64};
use nom::bytes::streaming::{take, tag};
use crate::error::FrameDecodeErr;
use amqp_types::basic_types::FieldValueKind;

pub const PROTOCOL_HEADER_SIZE: usize = 8;

// +-frame type: u8-+---channel id: u16---+-----length: u32-----+----payload---+--frame end--+
// |   1|2|3|4      |       0x0000        |     payload length  |              |  0xce       |
// +----------------+---------------------+---------------------+--------------+-------------+
// size_of(frame_type + channel_id + length)
pub const FRAME_PREFIX_LENGTH: usize = 7;

// parse protocol header
pub fn parse_amqp_protocal_header(buffer: &[u8]) -> Result<ProtocolHeader, FrameDecodeErr> {
    if buffer.len() < PROTOCOL_HEADER_SIZE {
        return Err(FrameDecodeErr::Incomplete);
    }

    let mut header = ProtocolHeader::default();
    let (buffer, protocol) = match tag("AMQP")(buffer){
        Ok(ret) => ret,
        Err(_) => return Err(FrameDecodeErr::ParseAmqpHeaderFailed)
    };
    header.set_protocol(Vec::from(protocol));
    let (buffer, major_id) = match be_u8(buffer) {
        Ok(ret) => ret,
        Err(_) => return Err(FrameDecodeErr::ParseAmqpHeaderFailed)
    };
    header.set_major_id(major_id);
    let (buffer, minor_id) = match be_u8(buffer) {
        Ok(ret) => ret,
        Err(_) => return Err(FrameDecodeErr::ParseAmqpHeaderFailed)
    };
    header.set_minor_id(minor_id);
    let (buffer, major_version) = match be_u8(buffer) {
        Ok(ret) => ret,
        Err(_) => return Err(FrameDecodeErr::ParseAmqpHeaderFailed)
    };
    header.set_major_version(major_version);
    let (_, minor_version) = match be_u8(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseAmqpHeaderFailed)
    };
    header.set_minor_version(minor_version);
    Ok(header)
}


pub(crate) fn parse_channel_id_and_length(buffer: &[u8]) -> Result<(&[u8], u16, u32), FrameDecodeErr> {
    let (buffer, channel_id) = match be_u16(buffer) {
        Ok(ret) => ret,
        Err(e) => {
            match e {
                Err::Incomplete(_) => return Err(FrameDecodeErr::Incomplete),
                _ => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
    };
    let (buffer, length) = match be_u32(buffer) {
        Ok(ret) => ret,
        Err(e) => {
            match e {
                Err::Incomplete(_) => return Err(FrameDecodeErr::Incomplete),
                _ => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
    };
    Ok((buffer, channel_id, length))
}

pub(crate) fn parse_arguments(class: Class, method: Method, buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {

}

pub(crate) fn get_method_type(class: Class, method_id: u16) -> Result<Method, FrameDecodeErr> {
    match class {
        Class::Connection => {
            let method = ConnectionMethod::from(method_id);
            if let ConnectionMethod::Unknown = method {
                return Err(FrameDecodeErr::UnknownMethodType);
            } else {
                return Ok(Method::ConnectionMethod(method));
            }
        }
        Class::Channel => {
            let method = ChannelMethod::from(method_id);
            if let ChannelMethod::Unknown = method {
                return Err(FrameDecodeErr::UnknownMethod);
            } else {
                return Ok(Method::ChannelMethod(method));
            }
        }
        Class::Access => {
            let method = AccessMethod::from(method_id);
            if let AccessMethod::Unknown = method {
                return Err(FrameDecodeErr::UnknownMethod);
            } else {
                return Ok(Method::AccessMethod(method));
            }
        }
        Class::Exchange => {
            let method = ExchangeMethod::from(method_id);
            if let ExchangeMethod::Unknown = method {
                return Err(FrameDecodeErr::UnknownMethod);
            } else {
                return Ok(Method::ExchangeMethod(method));
            }
        }
        Class::Queue => {
            let method = QueueMethod::from(method_id);
            if let QueueMethod::Unknown = method {
                return Err(FrameDecodeErr::UnknownMethod);
            } else {
                return Ok(Method::QueueMethod(method));
            }
        }
        Class::Basic => {
            let method = BasicMethod::from(method_id);
            if let BasicMethod::Unknown = method {
                return Err(FrameDecodeErr::UnknownMethod);
            } else {
                return Ok(Method::BasicMethod(method));
            }
        }
        Class::Tx => {
            let method = TxMethod::from(method_id);
            if let TxMethod::Unknown = method {
                return Err(FrameDecodeErr::UnknownMethod);
            } else {
                return Ok(Method::TxMethod(method));
            }
        }
        Class::Unknown => return Err(FrameDecodeErr::UnknownClassType)
    }
}

pub(crate) fn parse_short_string(buffer: &[u8]) -> Result<(&[u8], ShortStr), FrameDecodeErr> {
    let (buffer, length) = match be_u8(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, data) = match take(length)(buffer) {
        Ok(data) => data,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    Ok((buffer, ShortStr::with_bytes(data).unwrap()))
}

pub(crate) fn parse_field_array(buffer: &[u8]) -> Result<(&[u8], FieldArray), FrameDecodeErr> {
    // array bytes length
    let (buffer, length) = match be_u32(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, data) = match take(length)(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let mut arr: Vec<FieldValue> = Vec::new();

    while data.len() != 0 {
        let (data, value) = match parse_field_value(data) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
        };
        arr.push(value);
    }

    Ok((buffer, arr))
}

pub(crate) fn parse_field_table(buffer: &[u8]) -> Result<(&[u8], FieldTable), FrameDecodeErr> {
    let (buffer, length) = match be_u32(buffer) {
        Ok(ret) => ret,
        Err(err) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, data) = match take(length)(buffer) {
        Ok(ret) => ret,
        Err(err) => return Err(FrameDecodeErr::ParseFrameFailed)
    };

    let mut table = FieldTable::new();

    while data.len() != 0 {
        let (data, name) = match parse_field_name(data) {
            Ok(ret) => ret,
            Err(err) => return Err(FrameDecodeErr::ParseFrameFailed)
        };
        let (data, value) = match parse_field_value(data) {
            Ok(ret) => ret,
            Err(err) => return Err(FrameDecodeErr::ParseFrameFailed)
        };
        table.insert(name, value);
    }

    Ok((buffer, table))
}

pub(crate) fn parse_field_value(buffer: &[u8]) -> Result<(&[u8], FieldValue), FrameDecodeErr> {
    let (buffer, field_value_type) = match be_u8(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let field_value_type = FieldValueKind::from(field_value_type);
    if let FieldValueKind::Unknown = field_value_type {
        return Err(FrameDecodeErr::ParseFrameFailed);
    }
    match field_value_type {
        FieldValueKind::I8 => {
            if let Ok((buffer, value)) = be_i8(buffer) {
                return Ok((buffer, FieldValue::from_i8(value as i8)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::U8 => {
            if let Ok((buffer, value)) = be_u8(buffer) {
                return Ok((buffer, FieldValue::from_u8(value as u8)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::Boolean => {
            if let Ok((buffer, value)) = be_u8(buffer) {
                return Ok((buffer, FieldValue::from_bool(value != 0)))
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::I16 => {
            if let Ok((buffer, value)) = be_i16(buffer) {
                return Ok((buffer, FieldValue::from_i16(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::U16 => {
            if let Ok((buffer, value)) = be_u16(buffer) {
                return Ok((buffer, FieldValue::from_u16(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::I32 => {
            if let Ok((buffer, value)) = be_i32(buffer) {
                return Ok((buffer, FieldValue::from_i32(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::U32 => {
            if let Ok((buffer, value)) = be_u32(buffer) {
                return Ok((buffer, FieldValue::from_u32(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::I64 => {
            if let Ok((buffer, value)) = be_i64(buffer) {
                return Ok((buffer, FieldValue::from_i64(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::U64 => {
            if let Ok((buffer, value)) = be_u64(buffer) {
                return Ok((buffer, FieldValue::from_u64(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::F32 => {
            if let Ok((buffer, value)) = be_f32(buffer) {
                return Ok((buffer, FieldValue::from_f32(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::F64 => {
            if let Ok((buffer, value)) = be_f64(buffer) {
                return Ok((buffer, FieldValue::from_f64(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::Timestamp => {
            if let Ok((buffer, value)) = be_u64(buffer) {
                return Ok((buffer, FieldValue::from_timestamp(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::Decimal => {
            let scale = be_u8(buffer);
            let (buffer, scale) = if scale.is_ok() { scale.unwrap() } else { return Err(FrameDecodeErr::ParseFrameFailed); };
            if let Ok((buffer, value)) = be_u32(buffer) {
                return Ok((buffer, FieldValue::from_decimal(Decimal::new(scale, value))));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::LongStr => {
            match parse_long_string(buffer) {
                Ok((buffer, value)) => return Ok((buffer, FieldValue::from_long_string(value))),
                Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
        FieldValueKind::ByteArray => {
            match parse_long_string(buffer) {
                Ok((buffer, value)) => return Ok((buffer, FieldValue::from_bytes_array(value))),
                Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
        FieldValueKind::Void => return Ok((buffer, FieldValue::from_void())),
        FieldValueKind::FieldArray => {
            match parse_field_array(buffer) {
                Ok((buffer, value)) => return Ok((buffer, FieldValue::from_field_array(value))),
                Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
        FieldValueKind::FieldTable => {
            match parse_field_table(buffer) {
                Ok((buffer, value)) => return Ok((buffer, FieldValue::from_field_table(value))),
                Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
        _ => return Err(FrameDecodeErr::ParseFrameFailed)
    }

}

pub(crate) fn parse_field_name(buffer: &[u8]) -> Result<(&[u8], FieldName), FrameDecodeErr> {
    let (buffer, length) = match be_u8(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, data) = match take(length)(buffer) {
        Ok(data) => data,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let name = match FieldName::with_bytes(data) {
        Ok(name) => name,
        Err(e) => return Err(FrameDecodeErr::Amqp(e))
    };
    Ok((buffer, name))
}

pub(crate) fn parse_long_string(buffer: &[u8]) -> Result<(&[u8], LongStr), FrameDecodeErr> {
    let (buffer, length) = match be_u32(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, data) = match take(length) {
        Ok(data) => data,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    Ok((buffer, LongStr::with_bytes(data).unwrap()))
}

pub(crate) fn parse_connection_start(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, major_version) = match be_u8(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, minor_version) = match be_u8(buffer) {
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
    connection_start.set_major_version(major_version);
    connection_start.set_minor_version(minor_version);
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
    let (buffer, mechanism) = match parse_long_string(buffer) {
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
    let (buffer, channel_max) = match be_u16(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, frame_max) = match be_u32(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, heartbeat) = match be_u16(buffer) {
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
    let (buffer, channel_max) = match be_u16(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, frame_max) = match be_u32(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, heartbeat) = match be_u16(buffer) {
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
    let (buffer, insist) = match be_u8(buffer) {
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
    let (buffer, reply_code) = match be_u16(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, reply_text) = match parse_short_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, class_id) = match be_u16(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let class_type = Class::from(class_id);
    if let Class::Unknown = class_type {
        return Err(FrameDecodeErr::ParseFrameFailed);
    }
    let (buffer, method_id) = match be_u16(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let method_type = match get_method_type(class_type, method_id) {
        Ok(method_type) => method_type,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let connection_close = ConnectionClose::default();
    // Ok(Arguments::ConnectionClose(connection_close))
}

pub(crate) fn parse_connection_close_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, dummy) = match be_u8(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let connection_close_ok = ConnectionCloseOk::default();
    Ok(Arguments::ConnectionCloseOk(connection_close_ok))
}

pub(crate) fn parse_channel_open(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, out_of_band) = match parse_short_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let channel_open = ChannelOpen::default();
    Ok(Arguments::ChannelOpen(channel_open))
}

pub(crate) fn parse_channel_open_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, channel_id) = match parse_short_string(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let channel_open_ok = ChannelOpenOk::default();
    Ok(Arguments::ChannelOpenOk(channel_open_ok))
}

pub(crate) fn parse_channel_flow(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, active) = match be_u8(buffer) {
        Ok((buffer, value)) => (buffer, if let 0 = value { false } else { true }),
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let channel_flow = ChannelFlow::default();
    Ok(Arguments::ChannelFlow(channel_flow))
}

pub(crate) fn parse_channel_flow_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, active) = match be_u8(buffer) {

    }
}

pub(crate) fn parse_channel_close(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {

}

pub(crate) fn parse_method_frame(buffer: &[u8]) -> Result<(u32, Frame), FrameDecodeErr> {
    // skip frame type
    let (buffer, _) = match be_u8(buffer) {
        Ok(ret) => ret,
        Err(e) => {
            match e {
                nom::Err::Incomplete(_) => return Err(FrameDecodeErr::Incomplete),
                _ => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
    };
    let (buffer, channel_id, length) = match parse_channel_id_and_length(buffer) {
        Ok(ret) => ret,
        Err(e) => {
            match e {
                FrameDecodeErr::Incomplete => return Err(e),
                _ => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
    };
    let frame_length = FRAME_PREFIX_LENGTH + length as usize;

    // check length
    if buffer.len() < frame_length {
        return Err(FrameDecodeErr::Incomplete)
    }

    let (buffer, class_id) = match be_u16(buffer) {
        Ok(ret) => ret,
        _ => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, method_id) = match be_u16(buffer) {
        Ok(ret) => ret,
        _ => return Err(FrameDecodeErr::ParseFrameFailed)
    };

    // check class type
    let class_type = Class::from(class_id);
    if let Class::Unknown = class_type {
        return Err(FrameDecodeErr::UnknownClassType);
    }

    let method_type = match get_method_type(class_type.clone(), method_id) {
        Ok(method_type) => method_type,
        Err(e) => return Err(e)
    };

    let frame = Frame::default();
    frame.set_frame_type(FrameType::METHOD);
    frame.set_channel(channel_id);
    frame.set_length(length);
    frame.set_class(class_type);
    frame.set_method(method_type);

}

pub(crate) fn parse_content_header_frame(buffer: &[u8]) -> Result<(u32, Frame), FrameDecodeErr> {
    let frame = Frame::default();
    frame.set_frame_type(FrameType::HEADER);
}

pub(crate) fn parse_content_body_frame(buffer: &[u8]) -> Result<(u32, Frame), FrameDecodeErr> {
    let frame = Frame::default();
    frame.set_frame_type(FrameType::BODY);
}

pub(crate) fn parse_heartbeat_frame(buffer: &[u8]) -> Result<(u32, Frame), FrameDecodeErr> {
    let frame = Frame::default();
    frame.set_frame_type(FrameType::HEARTBEAT);
}

pub(crate) fn parse_args(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {

}