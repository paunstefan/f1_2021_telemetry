use bytes::{Buf, BytesMut};
use std::io::Cursor;

use crate::error::F1Error;

pub const HEADER_SIZE: usize = 24;

pub fn parse_header(buf: &mut Cursor<&mut BytesMut>) -> Result<Header, F1Error> {
    if buf.remaining() < HEADER_SIZE {
        return Err(F1Error::IncompleteData);
    }

    let format = buf.get_u16_le();
    let version = (buf.get_u8(), buf.get_u8());
    let packet_version = buf.get_u8();
    let packet_id = buf.get_u8().try_into()?;
    let session_uid = buf.get_u64_le();
    let session_time = buf.get_f32_le();
    let frame_identifier = buf.get_u32_le();
    let player_car_index = buf.get_u8();
    let secondary_player_car_index = buf.get_u8();

    let header = Header {
        format,
        version,
        packet_version,
        packet_id,
        session_uid,
        session_time,
        frame_identifier,
        player_car_index,
        secondary_player_car_index,
    };

    Ok(header)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub format: u16,
    pub version: (u8, u8),
    pub packet_version: u8,
    pub packet_id: PacketId,
    pub session_uid: u64,
    pub session_time: f32,
    pub frame_identifier: u32,
    pub player_car_index: u8,
    pub secondary_player_car_index: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PacketId {
    Motion,
    Session,
    LapData,
    Event,
    Participants,
    CarSetups,
    CarTelemetry,
    CarStatus,
    FinalClassification,
    LobbyInfo,
    CarDamage,
    SessionHistory,
}

impl TryFrom<u8> for PacketId {
    type Error = F1Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PacketId::Motion),
            1 => Ok(PacketId::Session),
            2 => Ok(PacketId::LapData),
            3 => Ok(PacketId::Event),
            4 => Ok(PacketId::Participants),
            5 => Ok(PacketId::CarSetups),
            6 => Ok(PacketId::CarTelemetry),
            7 => Ok(PacketId::CarStatus),
            8 => Ok(PacketId::FinalClassification),
            9 => Ok(PacketId::LobbyInfo),
            10 => Ok(PacketId::CarDamage),
            11 => Ok(PacketId::SessionHistory),
            _ => Err(F1Error::ConversionError),
        }
    }
}
