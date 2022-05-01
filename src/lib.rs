#![allow(non_camel_case_types)]
use std::{io::Cursor, net::SocketAddr};

use tokio_stream::{Stream, StreamExt};
use tokio_util::{codec::Decoder, udp::UdpFramed};

pub mod error;
pub mod packet;
pub mod utils;

pub struct F1_2021;

impl F1_2021 {
    /// Creates an async Stream of decoded packets
    pub fn telemetry(
        socket_address: SocketAddr,
    ) -> Result<impl Stream<Item = packet::Packet>, error::F1Error> {
        let socket = std::net::UdpSocket::bind(&socket_address)?;
        socket.set_nonblocking(true)?;
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

    /// This method is called after an UDP datagram is received
    /// It will try to parse a packet from the data
    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let len = src.len();

        if len < packet::header::HEADER_SIZE {
            return Ok(None);
        }
        // Buffer needs to be advanced, otherwise same frame will be processed
        let mut useful_buf = src.split_to(len);
        let mut cursor = Cursor::new(&mut useful_buf);

        let packet = packet::parse_packet(&mut cursor);

        match packet {
            Ok(pack) => Ok(Some(pack)),
            Err(err) => Err(err),
        }
    }
}
