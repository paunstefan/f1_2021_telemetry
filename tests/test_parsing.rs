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

    let mut cursor = Cursor::new(&buf[..]);

    let header = parse_packet(&mut cursor).unwrap();

    assert_eq!(header, pack);
}
