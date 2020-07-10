use amqp_types::{FieldValue, FieldTable, FieldArray, ShortStr, Decimal, FieldName, LongStr};
use crate::error::FrameDecodeErr;
use amqp_types::basic_types::FieldValueKind;
use nom::error::ErrorKind;
use nom::number::complete::{be_i64, be_u32, be_i32, be_u16, be_i16, be_u8, be_i8, be_u64, be_f32, be_f64};
use amqp_types::frame::{Method, Class, BasicMethod, TxMethod, QueueMethod, ExchangeMethod, AccessMethod, ChannelMethod, ConnectionMethod};
use nom::bytes::complete::take;

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
                return Err(FrameDecodeErr::UnknownMethodType);
            } else {
                return Ok(Method::ChannelMethod(method));
            }
        }
        Class::Access => {
            let method = AccessMethod::from(method_id);
            if let AccessMethod::Unknown = method {
                return Err(FrameDecodeErr::UnknownMethodType);
            } else {
                return Ok(Method::AccessMethod(method));
            }
        }
        Class::Exchange => {
            let method = ExchangeMethod::from(method_id);
            if let ExchangeMethod::Unknown = method {
                return Err(FrameDecodeErr::UnknownMethodType);
            } else {
                return Ok(Method::ExchangeMethod(method));
            }
        }
        Class::Queue => {
            let method = QueueMethod::from(method_id);
            if let QueueMethod::Unknown = method {
                return Err(FrameDecodeErr::UnknownMethodType);
            } else {
                return Ok(Method::QueueMethod(method));
            }
        }
        Class::Basic => {
            let method = BasicMethod::from(method_id);
            if let BasicMethod::Unknown = method {
                return Err(FrameDecodeErr::UnknownMethodType);
            } else {
                return Ok(Method::BasicMethod(method));
            }
        }
        Class::Tx => {
            let method = TxMethod::from(method_id);
            if let TxMethod::Unknown = method {
                return Err(FrameDecodeErr::UnknownMethodType);
            } else {
                return Ok(Method::TxMethod(method));
            }
        }
        Class::Unknown => return Err(FrameDecodeErr::UnknownClassType)
    }
}

pub(crate) fn parse_short_string(buffer: &[u8]) -> Result<(&[u8], ShortStr), FrameDecodeErr> {
    let (buffer, length) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, data) = match take::<_,_,(_, ErrorKind)>(length)(buffer) {
        Ok(data) => data,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    Ok((buffer, ShortStr::with_bytes(data).unwrap()))
}

pub(crate) fn parse_field_array(buffer: &[u8]) -> Result<(&[u8], FieldArray), FrameDecodeErr> {
    // array bytes length
    let (buffer, length) = match be_u32::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, data) = match take::<_,_,(_, ErrorKind)>(length)(buffer) {
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
    let (buffer, length) = match be_u32::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(err) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, data) = match take::<_,_,(_, ErrorKind)>(length)(buffer) {
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
    let (buffer, field_value_type) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let field_value_type = FieldValueKind::from(field_value_type);
    if let FieldValueKind::Unknown = field_value_type {
        return Err(FrameDecodeErr::ParseFrameFailed);
    }
    match field_value_type {
        FieldValueKind::I8 => {
            if let Ok((buffer, value)) = be_i8::<(_, ErrorKind)>(buffer) {
                return Ok((buffer, FieldValue::from_i8(value as i8)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::U8 => {
            if let Ok((buffer, value)) = be_u8::<(_, ErrorKind)>(buffer) {
                return Ok((buffer, FieldValue::from_u8(value as u8)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::Boolean => {
            if let Ok((buffer, value)) = be_u8::<(_, ErrorKind)>(buffer) {
                return Ok((buffer, FieldValue::from_bool(value != 0)))
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::I16 => {
            if let Ok((buffer, value)) = be_i16::<(_, ErrorKind)>(buffer) {
                return Ok((buffer, FieldValue::from_i16(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::U16 => {
            if let Ok((buffer, value)) = be_u16::<(_, ErrorKind)>(buffer) {
                return Ok((buffer, FieldValue::from_u16(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::I32 => {
            if let Ok((buffer, value)) = be_i32::<(_, ErrorKind)>(buffer) {
                return Ok((buffer, FieldValue::from_i32(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::U32 => {
            if let Ok((buffer, value)) = be_u32::<(_, ErrorKind)>(buffer) {
                return Ok((buffer, FieldValue::from_u32(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::I64 => {
            if let Ok((buffer, value)) = be_i64::<(_, ErrorKind)>(buffer) {
                return Ok((buffer, FieldValue::from_i64(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::U64 => {
            if let Ok((buffer, value)) = be_u64::<(_, ErrorKind)>(buffer) {
                return Ok((buffer, FieldValue::from_u64(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::F32 => {
            if let Ok((buffer, value)) = be_f32::<(_, ErrorKind)>(buffer) {
                return Ok((buffer, FieldValue::from_f32(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::F64 => {
            if let Ok((buffer, value)) = be_f64::<(_, ErrorKind)>(buffer) {
                return Ok((buffer, FieldValue::from_f64(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::Timestamp => {
            if let Ok((buffer, value)) = be_u64::<(_, ErrorKind)>(buffer) {
                return Ok((buffer, FieldValue::from_timestamp(value)));
            } else {
                return Err(FrameDecodeErr::ParseFrameFailed);
            }
        }
        FieldValueKind::Decimal => {
            let scale = be_u8::<(_, ErrorKind)>(buffer);
            let (buffer, scale) = if scale.is_ok() { scale.unwrap() } else { return Err(FrameDecodeErr::ParseFrameFailed); };
            if let Ok((buffer, value)) = be_u32::<(_, ErrorKind)>(buffer) {
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
    let (buffer, length) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, data) = match take::<_,_,(_, ErrorKind)>(length)(buffer) {
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
    let (buffer, length) = match be_u32::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, data) = match take::<_,_,(_, ErrorKind)>(length)(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    Ok((buffer, LongStr::with_bytes(data).unwrap()))
}


pub(crate) fn parse_channel_id_and_length(buffer: &[u8]) -> Result<(&[u8], u16, u32), FrameDecodeErr> {
    let (buffer, channel_id) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, length) = match be_u32::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    Ok((buffer, channel_id, length))
}