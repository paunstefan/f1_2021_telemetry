use bytes::BytesMut;

use crate::error::F1Error;
use std::io::Cursor;

pub mod event;
pub mod header;
pub mod motion;

use self::event::*;
use self::header::*;
use self::motion::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PacketType {
    Motion(MotionData),
    Event(),
    Unimplemented,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Packet {
    pub header: Header,
    pub data: PacketType,
}

pub fn parse_packet(buf: &mut Cursor<&mut BytesMut>) -> Result<Packet, F1Error> {
    let header = header::parse_header(buf)?;

    let data = match header.packet_id {
        PacketId::Motion => PacketType::Motion(parse_motion_packet(buf)?),

        PacketId::Event => todo!(),
        _ => PacketType::Unimplemented,
    };

    Ok(Packet { header, data })
}
