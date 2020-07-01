use tokio_util::codec::Decoder;
use amqp_types::{Frame, FrameType};
use bytes::BytesMut;
use nom::bytes::streaming::take;
use nom::number::streaming::{be_u16, be_u32, be_u8};
use std::fmt::Display;
use nom::lib::std::fmt::Formatter;
use std::io;
use amqp_types::frame::ProtocolHeader;
use crate::error::FrameDecodeErr;
use crate::parse::{parse_amqp_protocal_header, parse_method_frame, parse_content_header_frame, parse_content_body_frame, parse_heartbeat_frame};
use crate::frame_codec::DecodedFrame::AmqpFrame;

pub enum DecodedFrame {
    ProtocolHeader(ProtocolHeader),
    AmqpFrame(Frame)
}

pub struct FrameCodec {
    header_received: bool,
}

impl FrameCodec {
    pub fn new() -> Self {
        FrameCodec {
            header_received: false,
        }
    }
}

impl Decoder for FrameCodec {
    type Item = DecodedFrame;
    type Error = FrameDecodeErr;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // parse amqp header
        if !self.header_received {
            match parse_amqp_protocal_header(src) {
                Ok(header) => {
                    src.split_to(crate::parse::PROTOCOL_HEADER_SIZE);
                    return Ok(Some(DecodedFrame::ProtocolHeader(header)))
                },
                Err(e) => {
                    match e {
                        FrameDecodeErr::Incomplete => return Ok(None),
                        _ => return Err(e)
                    }
                }
            }
        }

        // +-frame type: u8-+---channel id: u16---+-----length: u32-----+----payload---+--frame end--+
        // |   1|2|3|4      |       0x0000        |     payload length  |              |  0xce       |
        // +----------------+---------------------+---------------------+--------------+-------------+
        let (buffer, frame_type_id) = match be_u8(src) {
            Ok(ret) => ret,
            Err(e) => {
                match e {
                    nom::Err::Incomplete(_) => return Ok(None),
                    _ => return Err(FrameDecodeErr::ParseFrameFailed)
                }
            }
        };
        let frame_type: FrameType = FrameType::from(frame_type_id);
        let (frame_length, frame) = match frame_type {
            FrameType::METHOD => {
                match parse_method_frame(buffer) {
                    Ok(ret) => ret,
                    Err(e) => {
                        match e {
                            FrameDecodeErr::Incomplete => return Ok(None),
                            _ => return Err(FrameDecodeErr::ParseFrameFailed)
                        }
                    }
                }
            },
            FrameType::HEADER => {
                match parse_content_header_frame(buffer) {
                    Ok(ret) => ret,
                    Err(e) => {
                        match e {
                            FrameDecodeErr::Incomplete => return Ok(None),
                            _ => return Err(FrameDecodeErr::ParseFrameFailed)
                        }
                    }
                }
            },
            FrameType::BODY => {
                match parse_content_body_frame(buffer) {
                    Ok(ret) => ret,
                    Err(e) => {
                        match e {
                            FrameDecodeErr::Incomplete => return Ok(None),
                            _ => return Err(FrameDecodeErr::ParseFrameFailed)
                        }
                    }
                }
            },
            FrameType::HEARTBEAT => {
                match parse_heartbeat_frame(buffer) {
                    Ok(ret) => ret,
                    Err(e) => {
                        match e {
                            FrameDecodeErr::Incomplete => return Ok(None),
                            _ => return Err(FrameDecodeErr::ParseFrameFailed)
                        }
                    }
                }
            },
            FrameType::UNKNOWN => return Err(FrameDecodeErr::UnknowFrameType)
        };
        src.split_to(frame_length as usize);
        Ok(Some(AmqpFrame(frame)))
    }
}
