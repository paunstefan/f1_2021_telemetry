#![allow(clippy::needless_range_loop)]

use bytes::{Buf, BytesMut};
use std::io::Cursor;

use crate::error::F1Error;
use crate::packet;
use crate::utils::*;

pub const CAR_TELEMETRY_SIZE: usize = 1347 - packet::header::HEADER_SIZE;

pub fn parse_car_telemetry_packet(
    buf: &mut Cursor<&mut BytesMut>,
) -> Result<TelemetryData, F1Error> {
    if buf.remaining() < CAR_TELEMETRY_SIZE {
        return Err(F1Error::IncompleteData);
    }

    let mut car_telemetry_data: [CarTelemetryData; NUMBER_OF_CARS] =
        [CarTelemetryData::default(); NUMBER_OF_CARS];

    for i in 0..NUMBER_OF_CARS {
        let speed = buf.get_u16_le();
        let throttle = buf.get_f32_le();
        let steer = buf.get_f32_le();
        let brake = buf.get_f32_le();
        let clutch = buf.get_u8();
        let gear = buf.get_i8();
        let engine_rpm = buf.get_u16_le();
        let drs = buf.get_u8() == 1;
        let rev_lights_percent = buf.get_u8();
        let rev_lights_bit = buf.get_u16_le();
        let brakes_temp = parse_wheels_data_u16(buf);
        let tyres_surface_temp = parse_wheels_data_u8(buf);
        let tyres_inner_temp = parse_wheels_data_u8(buf);
        let engine_temp = buf.get_u16_le();
        let tyres_pressure = parse_wheels_data_f32(buf);
        let surface_type = parse_wheels_data_u8(buf);

        car_telemetry_data[i] = CarTelemetryData {
            speed,
            throttle,
            steer,
            brake,
            clutch,
            gear,
            engine_rpm,
            drs,
            rev_lights_percent,
            rev_lights_bit,
            brakes_temp,
            tyres_surface_temp,
            tyres_inner_temp,
            engine_temp,
            tyres_pressure,
            surface_type,
        };
    }

    let mfd_panel_index = buf.get_u8();
    let mfd_panel_index_secondary = buf.get_u8();
    let suggested_gear = buf.get_i8();

    let packet = TelemetryData {
        car_telemetry_data,
        mfd_panel_index,
        mfd_panel_index_secondary,
        suggested_gear,
    };

    Ok(packet)
}

#[derive(Debug, Clone, PartialEq)]
pub struct TelemetryData {
    pub car_telemetry_data: [CarTelemetryData; NUMBER_OF_CARS],
    pub mfd_panel_index: u8,
    pub mfd_panel_index_secondary: u8,
    pub suggested_gear: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CarTelemetryData {
    pub speed: u16,
    pub throttle: f32,
    pub steer: f32,
    pub brake: f32,
    pub clutch: u8,
    pub gear: i8,
    pub engine_rpm: u16,
    pub drs: bool,
    pub rev_lights_percent: u8,
    pub rev_lights_bit: u16,
    pub brakes_temp: WheelsData<u16>,
    pub tyres_surface_temp: WheelsData<u8>,
    pub tyres_inner_temp: WheelsData<u8>,
    pub engine_temp: u16,
    pub tyres_pressure: WheelsData<f32>,
    pub surface_type: WheelsData<u8>,
}
