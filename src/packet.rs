use std::io::Cursor;

use crate::error::F1Error;

use self::header::Header;

pub mod event;
pub mod header;
pub mod motion;

pub fn parse_packet(buf: &mut Cursor<&[u8]>) -> Result<Header, F1Error> {
    header::parse_header(buf)
}
