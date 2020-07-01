use bytes::Bytes;
use amqp_types::frame::{ProtocolHeader, Arguments, Class, Method, ConnectionMethod, ChannelMethod, AccessMethod, ExchangeMethod, QueueMethod, BasicMethod, TxMethod};
use nom::{Err, Needed, IResult, tag};
use crate::{error};
use amqp_types::{Frame, FrameType, ShortStr, LongStr, FieldArray, FieldTable, FieldValue, FieldName, Decimal};
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


pub(crate) fn parse_channel_id_and_length(buffer: &[u8]) -> Result<(u16, u32, &[u8]), FrameDecodeErr> {
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
    Ok((channel_id, length, buffer))
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

pub(crate) fn parse_short_string(buffer: &[u8]) -> Result<(ShortStr, &[u8]), FrameDecodeErr> {
    let (buffer, length) = match be_u8(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, data) = match take(length)(buffer) {
        Ok(data) => data,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    Ok((ShortStr::with_bytes(data).unwrap(), buffer))
}

pub(crate) fn parse_field_array(buffer: &[u8]) -> Result<(FieldArray, &[u8]), FrameDecodeErr> {
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
        let (value, data) = match parse_field_value(data) {
            Ok(ret) => ret,
            Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
        };
        arr.push(value);
    }

    Ok((arr, buffer))
}

pub(crate) fn parse_field_table(buffer: &[u8]) -> Result<(FieldTable, &[u8]), FrameDecodeErr> {
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
        let (name, data) = match parse_field_name(data) {
            Ok(ret) => ret,
            Err(err) => return Err(FrameDecodeErr::ParseFrameFailed)
        };
        let (value, data) = match parse_field_value(data) {
            Ok(ret) => ret,
            Err(err) => return Err(FrameDecodeErr::ParseFrameFailed)
        };
        table.insert(name, value);
    }

    Ok((table, buffer))
}

pub(crate) fn parse_field_value(buffer: &[u8]) -> Result<(FieldValue, &[u8]), FrameDecodeErr> {
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
                return Ok((FieldValue::from_i8(value as i8), buffer));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::U8 => {
            if let Ok((buffer, value)) = be_u8(buffer) {
                return Ok((FieldValue::from_u8(value as u8), buffer));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::Boolean => {
            if let Ok((buffer, value)) = be_u8(buffer) {
                return Ok((FieldValue::from_bool(value != 0), buffer))
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::I16 => {
            if let Ok((buffer, value)) = be_i16(buffer) {
                return Ok((FieldValue::from_i16(value), buffer));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::U16 => {
            if let Ok((buffer, value)) = be_u16(buffer) {
                return Ok((FieldValue::from_u16(value), buffer));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::I32 => {
            if let Ok((buffer, value)) = be_i32(buffer) {
                return Ok((FieldValue::from_i32(value), buffer));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::U32 => {
            if let Ok((buffer, value)) = be_u32(buffer) {
                return Ok((FieldValue::from_u32(value), buffer));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::I64 => {
            if let Ok((buffer, value)) = be_i64(buffer) {
                return Ok((FieldValue::from_i64(value), buffer));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::U64 => {
            if let Ok((buffer, value)) = be_u64(buffer) {
                return Ok((FieldValue::from_u64(value), buffer));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::F32 => {
            if let Ok((buffer, value)) = be_f32(buffer) {
                return Ok((FieldValue::from_f32(value), buffer));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::F64 => {
            if let Ok((buffer, value)) = be_f64(buffer) {
                return Ok((FieldValue::from_f64(value), buffer));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::Timestamp => {
            if let Ok((buffer, value)) = be_u64(buffer) {
                return Ok((FieldValue::from_timestamp(value), buffer));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::Decimal => {
            let scale = be_u8(buffer);
            let (buffer, scale) = if scale.is_ok() { scale.unwrap() } else { return Err(FrameDecodeErr::ParseFrameFailed); };
            if let Ok((buffer, value)) = be_u32(buffer) {
                return Ok((FieldValue::from_decimal(Decimal::new(scale, value)), buffer));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::LongStr => {
            match parse_long_string(buffer) {
                Ok((value, buffer)) => return Ok((FieldValue::from_long_string(value), buffer)),
                Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
        FieldValueKind::ByteArray => {
            match parse_long_string(buffer) {
                Ok((value, buffer)) => return Ok((FieldValue::from_bytes_array(value), buffer)),
                Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
        FieldValueKind::Void => return Ok((FieldValue::from_void(), buffer)),
        FieldValueKind::FieldArray => {
            match parse_field_array(buffer) {
                Ok((value, buffer)) => return Ok((FieldValue::from_field_array(value), buffer)),
                Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
        FieldValueKind::FieldTable => {
            match parse_field_table(buffer) {
                Ok((value, buffer)) => return Ok((FieldValue::from_field_table(value), buffer)),
                Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
    }

}

pub(crate) fn parse_field_name(buffer: &[u8]) -> Result<(FieldName, &[u8]), FrameDecodeErr> {
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
    Ok((name, buffer))
}

pub(crate) fn parse_long_string(buffer: &[u8]) -> Result<(LongStr, &[u8]), FrameDecodeErr> {
    let (buffer, length) = match be_u32(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, data) = match take(length) {
        Ok(data) => data,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    Ok((LongStr::with_bytes(data).unwrap(), buffer))
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
    let (channel_id, length, buffer) = match parse_channel_id_and_length(buffer) {
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