#![allow(non_camel_case_types)]
use tokio_util::codec::Decoder;

pub mod error;
pub mod packet;

pub struct F1_2021;

impl F1_2021 {}

pub struct F1_2021_Decoder;

impl Decoder for F1_2021_Decoder {
    type Item = packet::Packet;

    type Error = error::F1Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}
