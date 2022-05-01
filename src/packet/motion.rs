#![allow(clippy::needless_range_loop)]
use bytes::{Buf, BytesMut};
use std::io::Cursor;

use crate::error::F1Error;
use crate::packet;
use crate::utils::*;

pub const MOTION_PACKET_SIZE: usize = 1464 - packet::header::HEADER_SIZE;

pub fn parse_motion_packet(buf: &mut Cursor<&mut BytesMut>) -> Result<MotionData, F1Error> {
    if buf.remaining() < MOTION_PACKET_SIZE {
        return Err(F1Error::IncompleteData);
    }

    let mut car_motion_data: [CarMotionData; NUMBER_OF_CARS] =
        [CarMotionData::default(); NUMBER_OF_CARS];

    for i in 0..NUMBER_OF_CARS {
        let world_positon = parse_coordinates_3d_f32(buf);

        let world_velocity = parse_coordinates_3d_f32(buf);

        let world_forward_dir = parse_coordinates_3d_i16(buf);

        let world_right_dir = parse_coordinates_3d_i16(buf);

        let g_force_lateral = buf.get_f32_le();
        let g_force_longitudinal = buf.get_f32_le();
        let g_force_vertical = buf.get_f32_le();

        let yaw = buf.get_f32_le();
        let pitch = buf.get_f32_le();
        let roll = buf.get_f32_le();

        car_motion_data[i] = CarMotionData {
            world_positon,
            world_velocity,
            world_forward_dir,
            world_right_dir,
            g_force_lateral,
            g_force_longitudinal,
            g_force_vertical,
            yaw,
            pitch,
            roll,
        }
    }

    let suspension_position = parse_wheels_data_f32(buf);

    let suspension_velocity = parse_wheels_data_f32(buf);

    let suspension_acceleration = parse_wheels_data_f32(buf);

    let wheel_speed = parse_wheels_data_f32(buf);

    let wheel_slip = parse_wheels_data_f32(buf);

    let local_velocity = parse_coordinates_3d_f32(buf);

    let angular_velocity = parse_coordinates_3d_f32(buf);

    let angular_acceleration = parse_coordinates_3d_f32(buf);

    let front_wheels_angle = buf.get_f32_le();

    let packet = MotionData {
        car_motion_data,
        suspension_position,
        suspension_velocity,
        suspension_acceleration,
        wheel_speed,
        wheel_slip,
        local_velocity,
        angular_velocity,
        angular_acceleration,
        front_wheels_angle,
    };

    Ok(packet)
}

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct CarMotionData {
    pub world_positon: Coordinates3D<f32>,
    pub world_velocity: Coordinates3D<f32>,
    pub world_forward_dir: Coordinates3D<i16>,
    pub world_right_dir: Coordinates3D<i16>,
    pub g_force_lateral: f32,
    pub g_force_longitudinal: f32,
    pub g_force_vertical: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MotionData {
    pub car_motion_data: [CarMotionData; NUMBER_OF_CARS],

    pub suspension_position: WheelsData<f32>,
    pub suspension_velocity: WheelsData<f32>,
    pub suspension_acceleration: WheelsData<f32>,
    pub wheel_speed: WheelsData<f32>,
    pub wheel_slip: WheelsData<f32>,
    pub local_velocity: Coordinates3D<f32>,
    pub angular_velocity: Coordinates3D<f32>,
    pub angular_acceleration: Coordinates3D<f32>,
    pub front_wheels_angle: f32,
}
