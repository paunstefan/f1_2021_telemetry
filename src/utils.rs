use std::io::Cursor;

use bytes::{Buf, BytesMut};

pub const NUMBER_OF_CARS: usize = 22;

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Coordinates3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct WheelsData<T> {
    pub rear_left: T,
    pub rear_right: T,
    pub front_left: T,
    pub front_right: T,
}

pub(crate) fn parse_coordinates_3d_f32(buf: &mut Cursor<&mut BytesMut>) -> Coordinates3D<f32> {
    Coordinates3D {
        x: buf.get_f32_le(),
        y: buf.get_f32_le(),
        z: buf.get_f32_le(),
    }
}

pub(crate) fn parse_coordinates_3d_i16(buf: &mut Cursor<&mut BytesMut>) -> Coordinates3D<i16> {
    Coordinates3D {
        x: buf.get_i16_le(),
        y: buf.get_i16_le(),
        z: buf.get_i16_le(),
    }
}

pub(crate) fn parse_wheels_data_f32(buf: &mut Cursor<&mut BytesMut>) -> WheelsData<f32> {
    WheelsData {
        rear_left: buf.get_f32_le(),
        rear_right: buf.get_f32_le(),
        front_left: buf.get_f32_le(),
        front_right: buf.get_f32_le(),
    }
}

pub(crate) fn parse_wheels_data_u16(buf: &mut Cursor<&mut BytesMut>) -> WheelsData<u16> {
    WheelsData {
        rear_left: buf.get_u16_le(),
        rear_right: buf.get_u16_le(),
        front_left: buf.get_u16_le(),
        front_right: buf.get_u16_le(),
    }
}

pub(crate) fn parse_wheels_data_u8(buf: &mut Cursor<&mut BytesMut>) -> WheelsData<u8> {
    WheelsData {
        rear_left: buf.get_u8(),
        rear_right: buf.get_u8(),
        front_left: buf.get_u8(),
        front_right: buf.get_u8(),
    }
}
