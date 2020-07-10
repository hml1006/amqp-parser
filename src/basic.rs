use amqp_types::frame::{Arguments, BasicQos, BasicQosOk, BasicConsume, BasicConsumeOk, BasicCancel, BasicCancelOk, BasicPublish, BasicReturn, BasicDeliver, BasicGet, BasicGetOk, BasicGetEmpty, BasicAck, BasicReject, BasicRecoverAsync, BasicRecover, BasicRecoverOk, BasicNack};
use crate::error::FrameDecodeErr;
use nom::number::complete::{be_u32, be_u16, be_u8, be_u64};
use nom::error::ErrorKind;
use crate::common::{parse_short_string, parse_field_table};

pub(crate) fn parse_basic_qos(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, prefetch_size) = match be_u32::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, prefetch_count) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (_, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let basic_qos = BasicQos::default();
    basic_qos.set_prefetch_size(prefetch_size);
    basic_qos.set_prefetch_count(prefetch_count);
    if 1 == (flags & (1 << 0)) {
        basic_qos.set_global(true);
    }
    Ok(Arguments::BasicQos(basic_qos))
}

pub(crate) fn parse_basic_qos_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let basic_qos_ok = BasicQosOk::default();
    basic_qos_ok.set_dummy(dummy);
    Ok(Arguments::BasicQosOk(basic_qos_ok))
}

pub(crate) fn parse_basic_consume(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, ticket) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, queue_name) = parse_short_string(buffer)?;
    let (buffer, consumer_tag) = parse_short_string(buffer)?;
    let (buffer, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (_, args) = parse_field_table(buffer)?;
    let basic_consume = BasicConsume::default();
    basic_consume.set_ticket(ticket);
    basic_consume.set_queue_name(queue_name);
    basic_consume.set_consumer_tag(consumer_tag);
    basic_consume.set_args(args);
    if 1 == (flags & (1 << 0)) {
        basic_consume.set_no_local(true);
    }
    if 1 == (flags & (1 << 1)) {
        basic_consume.set_no_ack(true);
    }
    if 1 == (flags & (1 << 2)) {
        basic_consume.set_exclusive(true);
    }
    if 1 == (flags & (1 << 3)) {
        basic_consume.set_no_wait(true);
    }
    Ok(Arguments::BasicConsume(basic_consume))
}

pub(crate) fn parse_basic_consume_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, consumer_tag) = parse_short_string(buffer)?;
    let basic_consume_ok = BasicConsumeOk::default();
    basic_consume_ok.set_consumer_tag(consumer_tag);
    Ok(Arguments::BasicConsumeOk(basic_consume_ok))
}

pub(crate) fn parse_basic_cancel(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, consumer_tag) = parse_short_string(buffer)?;
    let (_, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let basic_cancel = BasicCancel::default();
    basic_cancel.set_consumer_tag(consumer_tag);
    if 1 == (flags & (1 << 0)) {
        basic_cancel.set_no_wait(true);
    }
    Ok(Arguments::BasicCancel(basic_cancel))
}

pub(crate) fn parse_basic_cancel_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, consumer_tag) = parse_short_string(buffer)?;
    let basic_cancel_ok = BasicCancelOk::default();
    basic_cancel_ok.set_consumer_tag(consumer_tag);
    Ok(Arguments::BasicCancelOk(basic_cancel_ok))
}

pub(crate) fn parse_basic_publish(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, ticket) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, exchange_name) = parse_short_string(buffer)?;
    let (buffer, routing_key) = parse_short_string(buffer)?;
    let (_, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let basic_publish = BasicPublish::default();
    basic_publish.set_ticket(ticket);
    basic_publish.set_exchange_name(exchange_name);
    basic_publish.set_routing_key(routing_key);
    if 1 == (flags & (1 << 0)) {
        basic_publish.set_mandatory(true);
    }
    if 1 == (flags & (1 << 1)) {
        basic_publish.set_immediate(true);
    }
    Ok(Arguments::BasicPublish(basic_publish))
}

pub(crate) fn parse_basic_return(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, reply_code) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, reply_text) = parse_short_string(buffer)?;
    let (buffer, exchange_name) = parse_short_string(buffer)?;
    let (_, routing_key) = parse_short_string(buffer)?;
    let basic_return = BasicReturn::default();
    basic_return.set_reply_code(reply_code);
    basic_return.set_reply_text(reply_text);
    basic_return.set_exchange_name(exchange_name);
    basic_return.set_routing_key(routing_key);
    Ok(Arguments::BasicReturn(basic_return))
}

pub(crate) fn parse_basic_delivery(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, consumer_tag) = parse_short_string(buffer)?;
    let (buffer, delivery_tag) = match be_u64::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, exchange_name) = parse_short_string(buffer)?;
    let (_, routing_key) = parse_short_string(buffer)?;
    let basic_delivery = BasicDeliver::default();
    basic_delivery.set_consumer_tag(consumer_tag);
    basic_delivery.set_delivery_tag(delivery_tag);
    if 1 == (flags & (1 << 0)) {
        basic_delivery.set_redelivered(true);
    }
    basic_delivery.set_exchange_name(exchange_name);
    basic_delivery.set_routing_key(routing_key);
    Ok(Arguments::BasicDeliver(basic_delivery))
}

pub(crate) fn parse_basic_get(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, ticket) = match be_u16::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(_) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, queue_name) = parse_short_string(buffer)?;
    let (_, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(_) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let basic_get = BasicGet::default();
    basic_get.set_ticket(ticket);
    basic_get.set_queue_name(queue_name);
    if 1 == (flags & (1 << 0)) {
        basic_get.set_no_ack(true);
    }
    Ok(Arguments::BasicGet(basic_get))
}

pub(crate) fn parse_basic_get_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, delivery_tag) = match be_u64::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (buffer, exchange_name) = parse_short_string(buffer)?;
    let (buffer, routing_key) = parse_short_string(buffer)?;
    let (_, message_count) = match be_u32::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let basic_get_ok = BasicGetOk::default();
    basic_get_ok.set_delivery_tag(delivery_tag);
    if 1 == (flags & (1 << 0)) {
        basic_get_ok.set_redelivered(true);
    }
    basic_get_ok.set_exchange_name(exchange_name);
    basic_get_ok.set_routing_key(routing_key);
    basic_get_ok.set_message_count(message_count);
    Ok(Arguments::BasicGetOk(basic_get_ok))
}

pub(crate) fn parse_basic_get_empty(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, cluster_id) = parse_short_string(buffer)?;
    let basic_get_empty = BasicGetEmpty::default();
    basic_get_empty.set_cluster_id(cluster_id);
    Ok(Arguments::BasicGetEmpty(basic_get_empty))
}

pub(crate) fn parse_basic_ack(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, delivery_tag) = match be_u64::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (_, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let basic_ack = BasicAck::default();
    basic_ack.set_delivery_tag(delivery_tag);
    if 1 == (flags & (1 << 0)) {
        basic_ack.set_multiple(true);
    }
    Ok(Arguments::BasicAck(basic_ack))
}

pub(crate) fn parse_basic_reject(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, delivery_tag) = match be_u64::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (_, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let basic_reject = BasicReject::default();
    basic_reject.set_delivery_tag(delivery_tag);
    if 1 == (flags & (1 << 0)) {
        basic_reject.set_requeue(true);
    }
    Ok(Arguments::BasicReject(basic_reject))
}

pub(crate) fn parse_basic_recover_async(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let basic_recover_async = BasicRecoverAsync::default();
    if 1 == (flags & (1 << 0)) {
        basic_recover_async.set_requeue(true);
    }
    Ok(Arguments::BasicRecoverAsync(basic_recover_async))
}

pub(crate) fn parse_basic_recover(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let basic_recover = BasicRecover::default();
    if 1 == (flags & (1 << 0)) {
        basic_recover.set_requeue(true);
    }
    Ok(Arguments::BasicRecover(basic_recover))
}

pub(crate) fn parse_basic_recover_ok(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (_, dummy) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let basic_recover_ok = BasicRecoverOk::default();
    basic_recover_ok.set_dummy(dummy);
    Ok(Arguments::BasicRecoverOk(basic_recover_ok))
}

pub(crate) fn parse_basic_nack(buffer: &[u8]) -> Result<Arguments, FrameDecodeErr> {
    let (buffer, delivery_tag) = match be_u64::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(_) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let (_, flags) = match be_u8::<(_, ErrorKind)>(buffer) {
        Ok(ret) => ret,
        Err(e) => return Err(FrameDecodeErr::ParseFrameFailed)
    };
    let basic_nack = BasicNack::default();
    basic_nack.set_delivery_tag(delivery_tag);
    if 1 == (flags & (1 << 0)) {
        basic_nack.set_multiple(true);
    }
    if 1 == (flags & (1 << 1)) {
        basic_nack.set_requeue(true);
    }
    Ok(Arguments::BasicNack(basic_nack))
}
