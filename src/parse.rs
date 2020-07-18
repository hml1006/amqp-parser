use std::result::Result;
use amqp_types::frame::{ProtocolHeader, Arguments, Class, Method, ConnectionMethod, ChannelMethod, AccessMethod, ExchangeMethod, QueueMethod, BasicMethod, TxMethod, ConfirmMethod, MethodPayload, Payload};
use amqp_types::{Frame, FrameType};
use nom::number::complete::{be_u16, be_u8, be_u64};
use nom::bytes::streaming::{tag, take};
use crate::error::FrameDecodeErr;
use nom::error::ErrorKind;
use crate::common::{get_method_type, parse_channel_id_and_length};
use crate::connection::{parse_connection_start, parse_connection_start_ok, parse_connection_tune, parse_connection_tune_ok, parse_connection_secure, parse_connection_secure_ok, parse_connection_open, parse_connection_open_ok, parse_connection_close, parse_connection_close_ok};
use crate::channel::{parse_channel_open, parse_channel_open_ok, parse_channel_flow, parse_channel_flow_ok, parse_channel_close, parse_channel_close_ok};
use crate::access::{parse_access_request, parse_access_request_ok};
use crate::exchange::{parse_exchange_declare, parse_exchange_declare_ok, parse_exchange_bind, parse_exchange_bind_ok, parse_exchange_unbind, parse_exchange_unbind_ok, parse_exchange_delete, parse_exchange_delete_ok};
use crate::queue::{parse_queue_delete, parse_queue_declare, parse_queue_declare_ok, parse_queue_bind, parse_queue_bind_ok, parse_queue_unbind, parse_queue_unbind_ok, parse_queue_purge, parse_queue_purge_ok, parse_queue_delete_ok};
use crate::basic::{parse_basic_delivery, parse_basic_qos_ok, parse_basic_consume, parse_basic_consume_ok, parse_basic_cancel, parse_basic_cancel_ok, parse_basic_publish, parse_basic_return, parse_basic_get, parse_basic_get_ok, parse_basic_reject, parse_basic_recover_async, parse_basic_recover, parse_basic_recover_ok, parse_basic_ack, parse_basic_nack, parse_basic_qos, parse_basic_get_empty};
use crate::tx::{parse_tx_select, parse_tx_select_ok, parse_tx_commit, parse_tx_commit_ok, parse_tx_rollback, parse_tx_rollback_ok};
use crate::confirm::{parse_confirm_select, parse_confirm_select_ok};
use nom::{Err, Needed};

pub const PROTOCOL_HEADER_SIZE: usize = 8;

// +-frame type: u8-+---channel id: u16---+-----length: u32-----+----payload---+--frame end--+
// |   1|2|3|4      |       0x0000        |     payload length  |              |  0xce       |
// +----------------+---------------------+---------------------+--------------+-------------+
// size_of(frame_type + channel_id + length)
pub const FRAME_PREFIX_LENGTH: u32 = 7;

// parse protocol header
pub fn parse_amqp_protocal_header(buffer: &[u8]) -> Result<ProtocolHeader, FrameDecodeErr> {
    if buffer.len() < PROTOCOL_HEADER_SIZE {
        return Err(FrameDecodeErr::Incomplete);
    }

    let mut header = ProtocolHeader::default();
    let (buffer, protocol) = match tag::<_,_, (_,ErrorKind)>("AMQP")(buffer){
        Ok(ret) => ret,
        Err(_) => return Err(FrameDecodeErr::ParseAmqpHeaderFailed)
    };
    header.set_protocol(Vec::from(protocol));
    let (buffer, major_id) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(_) => return Err(FrameDecodeErr::ParseAmqpHeaderFailed)
    };
    header.set_major_id(major_id);
    let (buffer, minor_id) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(_) => return Err(FrameDecodeErr::ParseAmqpHeaderFailed)
    };
    header.set_minor_id(minor_id);
    let (buffer, major_version) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(_) => return Err(FrameDecodeErr::ParseAmqpHeaderFailed)
    };
    header.set_major_version(major_version);
    let (_, minor_version) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseAmqpHeaderFailed)
    };
    header.set_minor_version(minor_version);
    Ok(header)
}

pub(crate) fn parse_arguments(method: Method, buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    match method {
        Method::ConnectionMethod(method_type) => {
            match method_type {
                ConnectionMethod::Start => parse_connection_start(buffer),
                ConnectionMethod::StartOk => parse_connection_start_ok(buffer),
                ConnectionMethod::Tune => parse_connection_tune(buffer),
                ConnectionMethod::TuneOk => parse_connection_tune_ok(buffer),
                ConnectionMethod::Secure => parse_connection_secure(buffer),
                ConnectionMethod::SecureOk => parse_connection_secure_ok(buffer),
                ConnectionMethod::Open => parse_connection_open(buffer),
                ConnectionMethod::OpenOk => parse_connection_open_ok(buffer),
                ConnectionMethod::Close => parse_connection_close(buffer),
                ConnectionMethod::CloseOk => parse_connection_close_ok(buffer),
                ConnectionMethod::Unknown => return Err(FrameDecodeErr::UnknownMethodType)
            }
        },
        Method::ChannelMethod(method_type) => {
            match method_type {
                ChannelMethod::Open => parse_channel_open(buffer),
                ChannelMethod::OpenOk => parse_channel_open_ok(buffer),
                ChannelMethod::Flow => parse_channel_flow(buffer),
                ChannelMethod::FlowOk => parse_channel_flow_ok(buffer),
                ChannelMethod::Close => parse_channel_close(buffer),
                ChannelMethod::CloseOk => parse_channel_close_ok(buffer),
                ChannelMethod::Unknown => return Err(FrameDecodeErr::UnknownMethodType)
            }
        },
        Method::AccessMethod(method_type) => {
            match method_type {
                AccessMethod::Request => parse_access_request(buffer),
                AccessMethod::RequestOk => parse_access_request_ok(buffer),
                AccessMethod::Unknown => return Err(FrameDecodeErr::UnknownMethodType)
            }
        },
        Method::ExchangeMethod(method_type) => {
            match method_type {
                ExchangeMethod::Declare => parse_exchange_declare(buffer),
                ExchangeMethod::DeclareOk => parse_exchange_declare_ok(buffer),
                ExchangeMethod::Bind => parse_exchange_bind(buffer),
                ExchangeMethod::BindOk => parse_exchange_bind_ok(buffer),
                ExchangeMethod::Unbind => parse_exchange_unbind(buffer),
                ExchangeMethod::UnbindOk => parse_exchange_unbind_ok(buffer),
                ExchangeMethod::Delete => parse_exchange_delete(buffer),
                ExchangeMethod::DeleteOk => parse_exchange_delete_ok(buffer),
                ExchangeMethod::Unknown => return Err(FrameDecodeErr::UnknownMethodType)
            }
        },
        Method::QueueMethod(method_type) => {
            match method_type {
                QueueMethod::Declare => parse_queue_declare(buffer),
                QueueMethod::DeclareOk => parse_queue_declare_ok(buffer),
                QueueMethod::Bind => parse_queue_bind(buffer),
                QueueMethod::BindOk => parse_queue_bind_ok(buffer),
                QueueMethod::Unbind => parse_queue_unbind(buffer),
                QueueMethod::UnbindOk => parse_queue_unbind_ok(buffer),
                QueueMethod::Purge => parse_queue_purge(buffer),
                QueueMethod::PurgeOk => parse_queue_purge_ok(buffer),
                QueueMethod::Delete => parse_queue_delete(buffer),
                QueueMethod::DeleteOk => parse_queue_delete_ok(buffer),
                QueueMethod::Unknown => return Err(FrameDecodeErr::UnknownMethodType)
            }
        },
        Method::BasicMethod(method_type) => {
            match method_type {
                BasicMethod::Qos => parse_basic_qos(buffer),
                BasicMethod::QosOk => parse_basic_qos_ok(buffer),
                BasicMethod::Consume => parse_basic_consume(buffer),
                BasicMethod::ConsumeOk => parse_basic_consume_ok(buffer),
                BasicMethod::Cancel => parse_basic_cancel(buffer),
                BasicMethod::CancelOk => parse_basic_cancel_ok(buffer),
                BasicMethod::Publish => parse_basic_publish(buffer),
                BasicMethod::Return => parse_basic_return(buffer),
                BasicMethod::Deliver => parse_basic_delivery(buffer),
                BasicMethod::Get => parse_basic_get(buffer),
                BasicMethod::GetEmpty => parse_basic_get_empty(buffer),
                BasicMethod::GetOk => parse_basic_get_ok(buffer),
                BasicMethod::Reject => parse_basic_reject(buffer),
                BasicMethod::RecoverAsync => parse_basic_recover_async(buffer),
                BasicMethod::Recover => parse_basic_recover(buffer),
                BasicMethod::RecoverOk => parse_basic_recover_ok(buffer),
                BasicMethod::Ack => parse_basic_ack(buffer),
                BasicMethod::Nack => parse_basic_nack(buffer),
                BasicMethod::Unknown => return Err(FrameDecodeErr::UnknownMethodType)
            }
        },
        Method::TxMethod(method_type) => {
            match method_type {
                TxMethod::Select => parse_tx_select(buffer),
                TxMethod::SelectOk => parse_tx_select_ok(buffer),
                TxMethod::Commit => parse_tx_commit(buffer),
                TxMethod::CommitOk => parse_tx_commit_ok(buffer),
                TxMethod::Rollback => parse_tx_rollback(buffer),
                TxMethod::RollbackOk => parse_tx_rollback_ok(buffer),
                TxMethod::Unknown => return Err(FrameDecodeErr::UnknownMethodType)
            }
        },
        Method::ConfirmMethod(method_type) => {
            match method_type {
                ConfirmMethod::Select => parse_confirm_select(buffer),
                ConfirmMethod::SelectOk => parse_confirm_select_ok(buffer),
                ConfirmMethod::Unknown => return Err(FrameDecodeErr::UnknownMethodType)
            }
        }
    }
}


pub(crate) fn parse_method_frame(buffer: &[u8]) -> Result<(u32, Frame), FrameDecodeErr> {
    // skip frame type
    let (buffer, _) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => {
            match e {
                nom::Err::Incomplete(_) => return Err(FrameDecodeErr::Incomplete),
                _ => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
    };
    let (buffer, channel_id, payload_length) = match parse_channel_id_and_length(buffer) {
        Ok(ret) => ret,
        Err(e) => {
            match e {
                FrameDecodeErr::Incomplete => return Err(e),
                _ => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
    };
    let frame_length = FRAME_PREFIX_LENGTH + payload_length as u32;

    let (buffer, payload) = match take::<_,_,(_, ErrorKind)>(payload_length)(buffer) {
        Ok(ret) => ret,
        Err(e) => {
            match e {
                Err::Incomplete(Needed::Size(_)) => return Err(FrameDecodeErr::Incomplete),
                _ => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
    };

    // check length
    if buffer.len() < frame_length as usize {
        return Err(FrameDecodeErr::Incomplete)
    }

    let (buffer, class_id) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        _ => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, method_id) = match be_u16::<(_, ErrorKind)>(buffer) {
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

    let args = match parse_arguments(method_type, payload) {
        Ok(args) => args,
        Err(e) => return Err(e)
    };
    let payload = MethodPayload::default();
    payload.set_class(class_type);
    payload.set_method(method_type);
    payload.set_args(args);
    let payload = Payload::Method(payload);

    let frame = Frame::default();
    frame.set_frame_type(FrameType::METHOD);
    frame.set_channel(channel_id);
    frame.set_length(payload_length);
    frame.set_payload(payload);

    Ok((frame_length, frame))
}

pub(crate) fn parse_content_header_frame(buffer: &[u8]) -> Result<(u32, Frame), FrameDecodeErr> {
    // skip frame type
    let (buffer, _) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => {
            match e {
                nom::Err::Incomplete(_) => return Err(FrameDecodeErr::Incomplete),
                _ => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
    };
    let (buffer, channel_id, payload_length) = match parse_channel_id_and_length(buffer) {
        Ok(ret) => ret,
        Err(e) => {
            match e {
                FrameDecodeErr::Incomplete => return Err(e),
                _ => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
    };

    // take payload
    let (buffer, payload) = match take::<_,_,(_, ErrorKind)>(payload_length)(buffer) {
        Ok(ret) => ret,
        Err(e) => {
            match e {
                Err::Incomplete(Needed::Size(_)) => return Err(FrameDecodeErr::Incomplete),
                _ => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
    };

    // pase payload
    let (remain, class_id) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let class_type = Class::from(class_id);
    if let Class::Unknown = class_type {
        return Err(FrameDecodeErr::UnknownClassType);
    }
    let (remain, weight) = match be_u16::<(_, ErrorKind)>(remain) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (remain, body_size) = match be_u64::<(_, ErrorKind)>(remain) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };


    let frame_length = FRAME_PREFIX_LENGTH + payload_length as u32;
    let frame = Frame::default();
    frame.set_frame_type(FrameType::HEADER);
    frame.set_channel(channel_id);
    frame.set_length(payload_length);
}

pub(crate) fn parse_content_body_frame(buffer: &[u8]) -> Result<(u32, Frame), FrameDecodeErr> {
    let frame = Frame::default();
    frame.set_frame_type(FrameType::BODY);
}

pub(crate) fn parse_heartbeat_frame(buffer: &[u8]) -> Result<(u32, Frame), FrameDecodeErr> {
    let frame = Frame::default();
    frame.set_frame_type(FrameType::HEARTBEAT);
}

pub fn parse_frame(buffer: &[u8]) -> Result<(u32, Frame), FrameDecodeErr> {
    let (buffer, frame_type) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => {
            match e {
                nom::Err::Incomplete(_) => return Err(FrameDecodeErr::Incomplete),
                _ => return Err(FrameDecodeErr::ParseFrameFailed)
            }
        }
    };
    let frame_type = FrameType::from(frame_type);
    match frame_type {
        FrameType::HEARTBEAT => parse_heartbeat_frame(buffer),
        FrameType::METHOD => parse_method_frame(buffer),
        FrameType::HEADER => parse_content_header_frame(buffer),
        FrameType::BODY => parse_content_body_frame(buffer),
        FrameType::UNKNOWN => return Err(FrameDecodeErr::UnknowFrameType)
    }
}