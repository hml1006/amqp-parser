mod util;
pub mod error;
pub mod parser;

#[cfg(test)]
mod tests {
    use crate::parser::parse_amqp_protocal_header;

    #[test]
    fn test_protocol_header() {
        let header = [0x41u8, 0x4d, 0x51, 0x50, 0x00, 0x00, 0x09, 0x01];
        let protocol_header = parse_amqp_protocal_header(&header).unwrap();
        assert_eq!(protocol_header.major_version(), 0x9u8);
    }
}
