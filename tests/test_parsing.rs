use bytes::BytesMut;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;

use f1_2021_telemetry::packet;
use f1_2021_telemetry::packet::parse_packet;

#[test]
fn test_parse_header() {
    let pack = packet::header::Header {
        format: 2021,
        version: (1, 2),
        packet_version: 1,
        packet_id: packet::header::PacketId::Motion,
        session_uid: 1,
        session_time: 12.35,
        frame_identifier: 123,
        player_car_index: 1,
        secondary_player_car_index: 255,
    };

    let mut f = File::open("tests/packet_samples/header.pkt").expect("no file found");
    let mut buf = vec![0u8; packet::header::HEADER_SIZE];
    f.read(&mut buf).expect("buffer overflow");

    let mut buf = BytesMut::from(&buf[..]);

    let mut cursor = Cursor::new(&mut buf);

    let header = packet::header::parse_header(&mut cursor).unwrap();

    assert_eq!(header, pack);
}

#[test]
fn test_parse_motion() {
    use f1_2021_telemetry::utils::*;
    use packet::motion::*;

    let header = packet::header::Header {
        format: 2021,
        version: (1, 2),
        packet_version: 1,
        packet_id: packet::header::PacketId::Motion,
        session_uid: 1,
        session_time: 12.35,
        frame_identifier: 123,
        player_car_index: 1,
        secondary_player_car_index: 255,
    };

    let motion_data = {
        let mut car_motion_data: [CarMotionData; NUMBER_OF_CARS] = [CarMotionData::default(); 22];

        for i in 0..NUMBER_OF_CARS {
            let world_positon = Coordinates3D {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };

            let world_velocity = Coordinates3D {
                x: 10.0,
                y: 20.0,
                z: 30.0,
            };

            let world_forward_dir = Coordinates3D { x: 2, y: 2, z: 2 };

            let world_right_dir = Coordinates3D { x: 3, y: 3, z: 3 };

            let g_force_lateral = 0.0;
            let g_force_longitudinal = 1.0;
            let g_force_vertical = 1.0;

            let yaw = 0.0f32;
            let pitch = 0.0f32;
            let roll = 0.0f32;

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

        let suspension_position = WheelsData {
            rear_left: 0.0,
            rear_right: 0.0,
            front_left: 0.0,
            front_right: 0.0,
        };

        let suspension_velocity = WheelsData {
            rear_left: 0.0,
            rear_right: 0.0,
            front_left: 0.0,
            front_right: 0.0,
        };

        let suspension_acceleration = WheelsData {
            rear_left: 0.0,
            rear_right: 0.0,
            front_left: 0.0,
            front_right: 0.0,
        };

        let wheel_speed = WheelsData {
            rear_left: 0.0,
            rear_right: 0.0,
            front_left: 0.0,
            front_right: 0.0,
        };

        let wheel_slip = WheelsData {
            rear_left: 0.0,
            rear_right: 0.0,
            front_left: 0.0,
            front_right: 0.0,
        };

        let local_velocity = Coordinates3D {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        let angular_velocity = Coordinates3D {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let angular_acceleration = Coordinates3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let front_wheels_angle = 0.0;

        MotionData {
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
        }
    };

    let mut f = File::open("tests/packet_samples/motion.pkt").expect("no file found");
    let mut buf = vec![0u8; 2048];
    let size = f.read(&mut buf).expect("buffer overflow");

    let mut buf = BytesMut::from(&buf[..size]);

    let mut cursor = Cursor::new(&mut buf);

    let packet = parse_packet(&mut cursor).unwrap();

    assert_eq!(header, packet.header);
    assert_eq!(packet::PacketType::Motion(motion_data), packet.data);
}

#[test]
fn test_parse_event_SSTA() {
    let header = packet::header::Header {
        format: 2021,
        version: (1, 2),
        packet_version: 1,
        packet_id: packet::header::PacketId::Event,
        session_uid: 1,
        session_time: 12.35,
        frame_identifier: 123,
        player_car_index: 1,
        secondary_player_car_index: 255,
    };

    let event_data = packet::event::EventData {
        event_string_code: packet::event::EventCode::SessionStarted,
        event_details: packet::event::EventDataDetails::SessionStarted,
    };

    let mut f = File::open("tests/packet_samples/event_ssta.pkt").expect("no file found");
    let mut buf = vec![0u8; 2048];
    let size = f.read(&mut buf).expect("buffer overflow");

    let mut buf = BytesMut::from(&buf[..size]);

    let mut cursor = Cursor::new(&mut buf);

    let packet = parse_packet(&mut cursor).unwrap();

    assert_eq!(header, packet.header);
    assert_eq!(packet::PacketType::Event(event_data), packet.data);
}

#[test]
fn test_parse_event_ftlp() {
    let header = packet::header::Header {
        format: 2021,
        version: (1, 2),
        packet_version: 1,
        packet_id: packet::header::PacketId::Event,
        session_uid: 1,
        session_time: 12.35,
        frame_identifier: 123,
        player_car_index: 1,
        secondary_player_car_index: 255,
    };

    let event_data = packet::event::EventData {
        event_string_code: packet::event::EventCode::FastestLap,
        event_details: packet::event::EventDataDetails::FastestLap {
            vehicle_idx: 1,
            lap_time: 10.2,
        },
    };

    let mut f = File::open("tests/packet_samples/event_fltp.pkt").expect("no file found");
    let mut buf = vec![0u8; 2048];
    let size = f.read(&mut buf).expect("buffer overflow");

    let mut buf = BytesMut::from(&buf[..size]);

    let mut cursor = Cursor::new(&mut buf);

    let packet = parse_packet(&mut cursor).unwrap();

    assert_eq!(header, packet.header);
    assert_eq!(packet::PacketType::Event(event_data), packet.data);
}

#[test]
fn test_parse_event_butn() {
    let header = packet::header::Header {
        format: 2021,
        version: (1, 2),
        packet_version: 1,
        packet_id: packet::header::PacketId::Event,
        session_uid: 1,
        session_time: 12.35,
        frame_identifier: 123,
        player_car_index: 1,
        secondary_player_car_index: 255,
    };

    let mut buttons_set = std::collections::HashSet::new();

    buttons_set.insert(packet::event::ButtonFlags::A);
    buttons_set.insert(packet::event::ButtonFlags::RT);

    let event_data = packet::event::EventData {
        event_string_code: packet::event::EventCode::ButtonStatus,
        event_details: packet::event::EventDataDetails::Buttons {
            button_status: buttons_set,
        },
    };

    let mut f = File::open("tests/packet_samples/event_butn.pkt").expect("no file found");
    let mut buf = vec![0u8; 2048];
    let size = f.read(&mut buf).expect("buffer overflow");

    let mut buf = BytesMut::from(&buf[..size]);

    let mut cursor = Cursor::new(&mut buf);

    let packet = parse_packet(&mut cursor).unwrap();

    assert_eq!(header, packet.header);
    assert_eq!(packet::PacketType::Event(event_data), packet.data);
}

#[test]
fn test_parse_telemetry() {
    use f1_2021_telemetry::utils::*;
    use packet::car_telemetry::*;

    let header = packet::header::Header {
        format: 2021,
        version: (1, 2),
        packet_version: 1,
        packet_id: packet::header::PacketId::CarTelemetry,
        session_uid: 1,
        session_time: 12.35,
        frame_identifier: 123,
        player_car_index: 1,
        secondary_player_car_index: 255,
    };

    let telemetry_data = {
        let mut car_telemetry_data: [CarTelemetryData; NUMBER_OF_CARS] =
            [CarTelemetryData::default(); NUMBER_OF_CARS];

        for i in 0..NUMBER_OF_CARS {
            let speed = 123;
            let throttle = 1.0;
            let steer = 0.0;
            let brake = 0.0;
            let clutch = 0;
            let gear = 7;
            let engine_rpm = 1000;
            let drs = false;
            let rev_lights_percent = 50;
            let rev_lights_bit = 0;
            let brakes_temp = WheelsData {
                rear_left: 100,
                rear_right: 100,
                front_left: 100,
                front_right: 100,
            };
            let tyres_surface_temp = WheelsData {
                rear_left: 200,
                rear_right: 200,
                front_left: 200,
                front_right: 200,
            };
            let tyres_inner_temp = WheelsData {
                rear_left: 200,
                rear_right: 200,
                front_left: 200,
                front_right: 200,
            };
            let engine_temp = 1000;
            let tyres_pressure = WheelsData {
                rear_left: 50.0,
                rear_right: 50.0,
                front_left: 50.0,
                front_right: 50.0,
            };
            let surface_type = WheelsData {
                rear_left: 0,
                rear_right: 0,
                front_left: 0,
                front_right: 0,
            };

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

        let mfd_panel_index = 3;
        let mfd_panel_index_secondary = 4;
        let suggested_gear = 0;

        TelemetryData {
            car_telemetry_data,
            mfd_panel_index,
            mfd_panel_index_secondary,
            suggested_gear,
        }
    };

    let mut f = File::open("tests/packet_samples/car_telemetry.pkt").expect("no file found");
    let mut buf = vec![0u8; 2048];
    let size = f.read(&mut buf).expect("buffer overflow");

    let mut buf = BytesMut::from(&buf[..size]);

    let mut cursor = Cursor::new(&mut buf);

    let packet = parse_packet(&mut cursor).unwrap();

    assert_eq!(header, packet.header);
    assert_eq!(
        packet::PacketType::CarTelemetry(telemetry_data),
        packet.data
    );
}
