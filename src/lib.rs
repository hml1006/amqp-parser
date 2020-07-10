mod confirm;
mod tx;
mod basic;
mod queue;
mod exchange;
mod access;
mod channel;
mod connection;
mod common;
mod frame_codec;
pub mod error;
pub mod parse;

#[cfg(test)]
mod tests {
    use crate::parse::parse_amqp_protocal_header;

    #[test]
    fn test_protocol_header() {
        let header = [0x41u8, 0x4d, 0x51, 0x50, 0x00, 0x00, 0x09, 0x01];
        let protocol_header = parse_amqp_protocal_header(&header).unwrap();
        assert_eq!(protocol_header.major_version(), 0x9u8);
    }
}
