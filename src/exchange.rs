use amqp_types::frame::{Arguments, ExchangeDeclare, ExchangeDeclareOk, ExchangeDelete, ExchangeDeleteOk, ExchangeBind, ExchangeBindOk, ExchangeUnbind, ExchangeUnbindOk};
use crate::error::FrameDecodeErr;
use nom::error::ErrorKind;
use nom::number::complete::{be_u16, be_i8, be_u8};
use crate::common::{parse_short_string, parse_field_table};

pub(crate) fn parse_exchange_declare(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, ticket) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, exchange_name) = parse_short_string(buffer)?;
    let (buffer, exchange_type) = parse_short_string(buffer)?;
    let (buffer, flags) = match be_i8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(_) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (_, args) = parse_field_table(buffer)?;
    let exchange_declare = ExchangeDeclare::default();
    exchange_declare.set_ticket(ticket);
    exchange_declare.set_exchange_name(exchange_name);
    exchange_declare.set_exchange_type(exchange_type);
    if 1 == (flags & (1 << 0)) {
        exchange_declare.set_passive(true);
    }
    if 1 == (flags & (1 << 1)) {
        exchange_declare.set_durable(true);
    }
    if 1 == (flags & (1 << 2)) {
        exchange_declare.set_auto_delete(true);
    }
    if 1 == (flags & (1 << 3)) {
        exchange_declare.set_internal(true);
    }
    if 1 == (flags & (1 << 4)) {
        exchange_declare.set_no_wait(true);
    }
    exchange_declare.set_args(args);
    Ok(Arguments::ExchangeDeclare(exchange_declare))
}

pub(crate) fn parse_exchange_declare_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let exchange_declare_ok = ExchangeDeclareOk::default();
    exchange_declare_ok.set_dummy(dummy);
    Ok(Arguments::ExchangeDeclareOk(exchange_declare_ok))
}

pub(crate) fn parse_exchange_delete(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, ticket) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (_, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let exchange_delete = ExchangeDelete::default();
    exchange_delete.set_ticket(ticket);
    if 1 == (flags & (1 << 0)) {
        exchange_delete.set_if_unused(true);
    }
    if 1 == (flags & (1 << 1)) {
        exchange_delete.set_no_wait(true);
    }
    Ok(Arguments::ExchangeDelete(exchange_delete))
}

pub(crate) fn parse_exchange_delete_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let exchange_delete_ok = ExchangeDeleteOk::default();
    exchange_delete_ok.set_dummy(dummy);
    Ok(Arguments::ExchangeDeleteOk(exchange_delete_ok))
}

pub(crate) fn parse_exchange_bind(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, ticket) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, destination) = parse_short_string(buffer)?;
    let (buffer, source) = parse_short_string(buffer)?;
    let (buffer, routing_key) = parse_short_string(buffer)?;
    let (buffer, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (_, args) = parse_field_table(buffer)?;
    let exchange_bind = ExchangeBind::default();
    exchange_bind.set_ticket(ticket);
    exchange_bind.set_destination(destination);
    exchange_bind.set_source(source);
    exchange_bind.set_routing_key(routing_key);
    if 1 == (flags & (1 << 0)) {
        exchange_bind.set_no_wait(true);
    }
    exchange_bind.set_args(args);
    Ok(Arguments::ExchangeBind(exchange_bind))
}

pub(crate) fn parse_exchange_bind_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let exchange_bind_ok = ExchangeBindOk::default();
    exchange_bind_ok.set_dummy(dummy);
    Ok(Arguments::ExchangeBindOk(exchange_bind_ok))
}

pub(crate) fn parse_exchange_unbind(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, ticket) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, destination) = parse_short_string(buffer)?;
    let (buffer, source) = parse_short_string(buffer)?;
    let (buffer, routing_key) = parse_short_string(buffer)?;
    let (buffer, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (_, args) = parse_field_table(buffer)?;
    let exchange_unbind = ExchangeUnbind::default();
    exchange_unbind.set_ticket(ticket);
    exchange_unbind.set_destination(destination);
    exchange_unbind.set_source(source);
    exchange_unbind.set_routing_key(routing_key);
    if 1 == (flags & (1 << 0)) {
        exchange_unbind.set_no_wait(true);
    }
    exchange_unbind.set_args(args);
    Ok(Arguments::ExchangeUnbind(exchange_unbind))
}

pub(crate) fn parse_exchange_unbind_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let exchange_unbind_ok = ExchangeUnbindOk::default();
    exchange_unbind_ok.set_dummy(dummy);
    Ok(Arguments::ExchangeUnbindOk(exchange_unbind_ok))
}
