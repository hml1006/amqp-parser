use amqp_types::frame::{Arguments, QueueDeclare, QueueDeclareOk, QueueBind, QueueBindOk, QueuePurge, QueuePurgeOk, QueueDelete, QueueDeleteOk, QueueUnbind, QueueUnbindOk};
use crate::error::FrameDecodeErr;
use nom::number::complete::{be_u16, be_u8, be_u32};
use nom::error::ErrorKind;
use crate::common::{parse_short_string, parse_field_table};

pub(crate) fn parse_queue_declare(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, ticket) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, queue_name) = parse_short_string(buffer)?;
    let (buffer, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (_, args) = parse_field_table(buffer)?;
    let queue_declare = QueueDeclare::default();
    queue_declare.set_ticket(ticket);
    queue_declare.set_queue_name(queue_name);
    queue_declare.set_args(args);
    if 1 == (flags & (1 << 0)) {
        queue_declare.set_passive(true);
    }
    if 1 == (flags & (1 << 1)) {
        queue_declare.set_durable(true);
    }
    if 1 == (flags & (1 << 2)) {
        queue_declare.set_exclusive(true);
    }
    if 1 == (flags & (1 << 3)) {
        queue_declare.set_auto_delete(true);
    }
    if 1 == (flags & (1 << 4)) {
        queue_declare.set_no_wait(true);
    }
    Ok(Arguments::QueueDeclare(queue_declare))
}

pub(crate) fn parse_queue_declare_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, queue_name) = parse_short_string(buffer)?;
    let (buffer, message_count) = match be_u32::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (_, consumer_count) = match be_u32::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let queue_declare_ok = QueueDeclareOk::default();
    queue_declare_ok.set_queue_name(queue_name);
    queue_declare_ok.set_message_count(message_count);
    queue_declare_ok.set_consumer_count(consumer_count);
    Ok(Arguments::QueueDeclareOk(queue_declare_ok))
}

pub(crate) fn parse_queue_bind(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, ticket) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, queue_name) = parse_short_string(buffer)?;
    let (buffer, exchange_name) = parse_short_string(buffer)?;
    let (buffer, routing_key) = parse_short_string(buffer)?;
    let (buffer, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (_, args) = parse_field_table(buffer)?;
    let queue_bind = QueueBind::default();
    queue_bind.set_ticket(ticket);
    queue_bind.set_queue_name(queue_name);
    queue_bind.set_exchange_name(exchange_name);
    queue_bind.set_routing_key(routing_key);
    if 1 == (flags & (1 << 0)) {
        queue_bind.set_no_wait(true);
    }
    queue_bind.set_args(args);
    Ok(Arguments::QueueBind(queue_bind))
}

pub(crate) fn parse_queue_bind_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let queue_bind_ok = QueueBindOk::default();
    queue_bind_ok.set_dummy(dummy);
    Ok(Arguments::QueueBindOk(queue_bind_ok))
}

pub(crate) fn parse_queue_purge(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, ticket) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, queue_name) = parse_short_string(buffer)?;
    let (_, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let queue_purge = QueuePurge::default();
    queue_purge.set_ticket(ticket);
    queue_purge.set_queue_name(queue_name);
    if 1 == (flags & (1 << 0)) {
        queue_purge.set_no_wait(true);
    }
    Ok(Arguments::QueuePurge(queue_purge))
}

pub(crate) fn parse_queue_purge_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, message_count) = match be_u32::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let queue_purge_ok = QueuePurgeOk::default();
    queue_purge_ok.set_message_count(message_count);
    Ok(Arguments::QueuePurgeOk(queue_purge_ok))
}

pub(crate) fn parse_queue_delete(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, ticket) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, queue_name) = parse_short_string(buffer)?;
    let (_, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let queue_delete = QueueDelete::default();
    queue_delete.set_ticket(ticket);
    queue_delete.set_queue_name(queue_name);
    if 1 == (flags & (1 << 0)) {
        queue_delete.set_if_unused(true);
    }
    if 1 == (flags & (1 << 1)) {
        queue_delete.set_if_empty(true);
    }
    if 1 == (flags & (1 << 2)) {
        queue_delete.set_no_wait(true);
    }
    Ok(Arguments::QueueDelete(queue_delete))
}

pub(crate) fn parse_queue_delete_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, message_count) = match be_u32::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let queue_delete_ok = QueueDeleteOk::default();
    queue_delete_ok.set_message_count(message_count);
    Ok(Arguments::QueueDeleteOk(queue_delete_ok))
}

pub(crate) fn parse_queue_unbind(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, ticket) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, queue_name) = parse_short_string(buffer)?;
    let (buffer, exchange_name) = parse_short_string(buffer)?;
    let (buffer, routing_key) = parse_short_string(buffer)?;
    let (_, args) = parse_field_table(buffer)?;
    let queue_unbind = QueueUnbind::default();
    queue_unbind.set_ticket(ticket);
    queue_unbind.set_queue_name(queue_name);
    queue_unbind.set_exchange_name(exchange_name);
    queue_unbind.set_routing_key(routing_key);
    queue_unbind.set_args(args);
    Ok(Arguments::QueueUnbind(queue_unbind))
}

pub(crate) fn parse_queue_unbind_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let queue_unbind_ok = QueueUnbindOk::default();
    queue_unbind_ok.set_dummy(dummy);
    Ok(Arguments::QueueUnbindOk(queue_unbind_ok))
}