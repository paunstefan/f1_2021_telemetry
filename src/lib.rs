#![allow(non_camel_case_types)]
use std::{io::Cursor, net::SocketAddr};

use bytes::Buf;
use tokio_stream::{Stream, StreamExt};
use tokio_util::{codec::Decoder, udp::UdpFramed};

pub mod error;
pub mod packet;

pub struct F1_2021;

impl F1_2021 {
    pub fn telemetry(
        socket_address: SocketAddr,
    ) -> Result<impl Stream<Item = packet::Packet>, error::F1Error> {
        let socket = std::net::UdpSocket::bind(&socket_address)?;
        let socket = tokio::net::UdpSocket::from_std(socket)?;

        Ok(UdpFramed::new(socket, F1_2021_Decoder)
            .map(|result| result.unwrap())
            .map(|(packet, _addr)| packet))
    }
}

pub struct F1_2021_Decoder;

impl Decoder for F1_2021_Decoder {
    type Item = packet::Packet;

    type Error = error::F1Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let mut cursor = Cursor::new(src);

        if cursor.remaining() < packet::header::HEADER_SIZE {
            return Ok(None);
        }

        let packet = packet::parse_packet(&mut cursor);

        match packet {
            Ok(pack) => Ok(Some(pack)),
            Err(err) => Err(err),
        }
    }
}
