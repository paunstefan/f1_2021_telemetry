use std::io::Cursor;

use bytes::{Buf, BytesMut};
use enum_iterator::IntoEnumIterator;

use crate::error::F1Error;

const EVENT_CODE_SIZE: usize = 4;

pub fn parse_event_packet(buf: &mut Cursor<&mut BytesMut>) -> Result<EventData, F1Error> {
    use EventDataDetails::*;

    if buf.remaining() < EVENT_CODE_SIZE {
        return Err(F1Error::IncompleteData);
    }

    let event_string_code = parse_event_code(buf)?;

    let event_details = match event_string_code {
        EventCode::SessionStarted => SessionStarted,
        EventCode::SessionEnded => SessionEnded,
        EventCode::FastestLap => parse_fastest_lap(buf)?,
        EventCode::Retirement => parse_retirement(buf)?,
        EventCode::DRSEnabled => DRSEnabled,
        EventCode::DRSDisabled => DRSDisabled,
        EventCode::TeamMateInPits => parse_teammate_in_pits(buf)?,
        EventCode::ChequeredFlag => ChequeredFlag,
        EventCode::RaceWinner => parse_race_winner(buf)?,
        EventCode::PenaltyIssued => parse_penalty(buf)?,
        EventCode::SpeedTrapTriggered => parse_speed_trap(buf)?,
        EventCode::StartLights => parse_start_lights(buf)?,
        EventCode::LightsOut => LightsOut,
        EventCode::DriveThroughServed => parse_drive_penalty(buf)?,
        EventCode::StopGoServed => parse_stop_go_penalty(buf)?,
        EventCode::Flashback => parse_flashback(buf)?,
        EventCode::ButtonStatus => parse_buttons(buf)?,
    };

    Ok(EventData {
        event_string_code,
        event_details,
    })
}

fn parse_fastest_lap(buf: &mut Cursor<&mut BytesMut>) -> Result<EventDataDetails, F1Error> {
    if buf.remaining() < 5 {
        return Err(F1Error::IncompleteData);
    }

    Ok(EventDataDetails::FastestLap {
        vehicle_idx: buf.get_u8(),
        lap_time: buf.get_f32_le(),
    })
}

fn parse_retirement(buf: &mut Cursor<&mut BytesMut>) -> Result<EventDataDetails, F1Error> {
    if buf.remaining() < 1 {
        return Err(F1Error::IncompleteData);
    }

    Ok(EventDataDetails::Retirement {
        vehicle_idx: buf.get_u8(),
    })
}

fn parse_teammate_in_pits(buf: &mut Cursor<&mut BytesMut>) -> Result<EventDataDetails, F1Error> {
    if buf.remaining() < 1 {
        return Err(F1Error::IncompleteData);
    }

    Ok(EventDataDetails::TeamMateInPits {
        vehicle_idx: buf.get_u8(),
    })
}

fn parse_race_winner(buf: &mut Cursor<&mut BytesMut>) -> Result<EventDataDetails, F1Error> {
    if buf.remaining() < 1 {
        return Err(F1Error::IncompleteData);
    }

    Ok(EventDataDetails::RaceWinner {
        vehicle_idx: buf.get_u8(),
    })
}

fn parse_penalty(buf: &mut Cursor<&mut BytesMut>) -> Result<EventDataDetails, F1Error> {
    if buf.remaining() < 7 {
        return Err(F1Error::IncompleteData);
    }

    Ok(EventDataDetails::Penalty {
        penalty_type: buf.get_u8(),
        infringement_type: buf.get_u8(),
        vehicle_idx: buf.get_u8(),
        other_vehicle_idx: buf.get_u8(),
        time: buf.get_u8(),
        lap_num: buf.get_u8(),
        places_gained: buf.get_u8(),
    })
}

fn parse_speed_trap(buf: &mut Cursor<&mut BytesMut>) -> Result<EventDataDetails, F1Error> {
    if buf.remaining() < 7 {
        return Err(F1Error::IncompleteData);
    }

    Ok(EventDataDetails::SpeedTrap {
        vehicle_idx: buf.get_u8(),
        speed: buf.get_f32_le(),
        overall_fastest_in_session: buf.get_u8(),
        driver_fastest_in_session: buf.get_u8(),
    })
}

fn parse_start_lights(buf: &mut Cursor<&mut BytesMut>) -> Result<EventDataDetails, F1Error> {
    if buf.remaining() < 1 {
        return Err(F1Error::IncompleteData);
    }

    Ok(EventDataDetails::StartLights {
        num_lights: buf.get_u8(),
    })
}

fn parse_drive_penalty(buf: &mut Cursor<&mut BytesMut>) -> Result<EventDataDetails, F1Error> {
    if buf.remaining() < 1 {
        return Err(F1Error::IncompleteData);
    }

    Ok(EventDataDetails::DriveThroughPenaltyServed {
        vehicle_idx: buf.get_u8(),
    })
}

fn parse_stop_go_penalty(buf: &mut Cursor<&mut BytesMut>) -> Result<EventDataDetails, F1Error> {
    if buf.remaining() < 1 {
        return Err(F1Error::IncompleteData);
    }

    Ok(EventDataDetails::StopGoPenaltyServed {
        vehicle_idx: buf.get_u8(),
    })
}

fn parse_flashback(buf: &mut Cursor<&mut BytesMut>) -> Result<EventDataDetails, F1Error> {
    if buf.remaining() < 8 {
        return Err(F1Error::IncompleteData);
    }

    Ok(EventDataDetails::Flashback {
        flashback_frame_identifier: buf.get_u32_le(),
        flashback_session_time: buf.get_f32_le(),
    })
}

fn parse_buttons(buf: &mut Cursor<&mut BytesMut>) -> Result<EventDataDetails, F1Error> {
    if buf.remaining() < 4 {
        return Err(F1Error::IncompleteData);
    }

    let flags = buf.get_u32_le();

    todo!()
}

fn parse_event_code(buf: &mut Cursor<&mut BytesMut>) -> Result<EventCode, F1Error> {
    let code_chars = [buf.get_u8(), buf.get_u8(), buf.get_u8(), buf.get_u8()];
    let code_string = std::str::from_utf8(&code_chars)?.to_string();

    code_string.try_into()
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventData {
    pub event_string_code: EventCode,
    pub event_details: EventDataDetails,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventDataDetails {
    SessionStarted,
    SessionEnded,
    FastestLap {
        vehicle_idx: u8,
        lap_time: f32,
    },
    Retirement {
        vehicle_idx: u8,
    },
    DRSEnabled,
    DRSDisabled,
    TeamMateInPits {
        vehicle_idx: u8,
    },
    ChequeredFlag,
    RaceWinner {
        vehicle_idx: u8,
    },
    Penalty {
        penalty_type: u8,
        infringement_type: u8,
        vehicle_idx: u8,
        other_vehicle_idx: u8,
        time: u8,
        lap_num: u8,
        places_gained: u8,
    },

    SpeedTrap {
        vehicle_idx: u8,
        speed: f32,
        overall_fastest_in_session: u8,
        driver_fastest_in_session: u8,
    },

    StartLights {
        num_lights: u8,
    },
    LightsOut,
    DriveThroughPenaltyServed {
        vehicle_idx: u8,
    },
    StopGoPenaltyServed {
        vehicle_idx: u8,
    },

    Flashback {
        flashback_frame_identifier: u32,
        flashback_session_time: f32,
    },

    Buttons {
        button_status: std::collections::HashSet<ButtonFlags>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventCode {
    SessionStarted,
    SessionEnded,
    FastestLap,
    Retirement,
    DRSEnabled,
    DRSDisabled,
    TeamMateInPits,
    ChequeredFlag,
    RaceWinner,
    PenaltyIssued,
    SpeedTrapTriggered,
    StartLights,
    LightsOut,
    DriveThroughServed,
    StopGoServed,
    Flashback,
    ButtonStatus,
}

impl TryFrom<String> for EventCode {
    type Error = F1Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "SSTA" => Ok(EventCode::SessionStarted),
            "SEND" => Ok(EventCode::SessionEnded),
            "FTLP" => Ok(EventCode::FastestLap),
            "RTMT" => Ok(EventCode::Retirement),
            "DRSE" => Ok(EventCode::DRSEnabled),
            "DRSD" => Ok(EventCode::DRSDisabled),
            "TMPT" => Ok(EventCode::TeamMateInPits),
            "CHQF" => Ok(EventCode::ChequeredFlag),
            "RCWN" => Ok(EventCode::RaceWinner),
            "PENA" => Ok(EventCode::PenaltyIssued),
            "SPTP" => Ok(EventCode::SpeedTrapTriggered),
            "STLG" => Ok(EventCode::StartLights),
            "LGOT" => Ok(EventCode::LightsOut),
            "DTSV" => Ok(EventCode::DriveThroughServed),
            "SGSV" => Ok(EventCode::StopGoServed),
            "FLBK" => Ok(EventCode::Flashback),
            "BUTN" => Ok(EventCode::ButtonStatus),
            _ => Err(F1Error::ConversionError),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, PartialEq, IntoEnumIterator, Eq, Hash)]
pub enum ButtonFlags {
    A = 0x00000001,
    Y = 0x00000002,
    B = 0x00000004,
    X = 0x00000008,
    DpadLeft = 0x00000010,
    DpadRight = 0x00000020,
    DpadUp = 0x00000040,
    DpadDown = 0x00000080,
    Options = 0x00000100,
    LB = 0x00000200,
    RB = 0x00000400,
    LT = 0x00000800,
    RT = 0x00001000,
    LeftStickClick = 0x00002000,
    RightStickClick = 0x00004000,
    RightStickLeft = 0x000008000,
    RightStickRight = 0x00010000,
    RightStickUp = 0x00020000,
    RightStickDown = 0x00040000,
    Special = 0x00080000,
}
