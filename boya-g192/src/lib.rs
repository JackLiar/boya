use std::io::Read;
use std::io::Write;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

#[rustfmt::skip]
const G192_MAGICS: [u16; 16] = [
    0x206b, 0x216b, 0x226b, 0x236b, 0x246b, 0x256b, 0x266b, 0x276b,
    0x286b, 0x296b, 0x2a6b, 0x2b6b, 0x2c6b, 0x2d6b, 0x2e6b, 0x2f6b,
];

#[repr(u16)]
#[derive(Clone, Copy, Debug, Default)]
pub enum Bit {
    #[default]
    B0 = 0x007f,
    B1 = 0x0081,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("not an G.192 bitstream")]
    NotG192,
    #[error("invalid bit value, expecting 0x007f/0x0081")]
    InvalidBit,
    #[error("IO error")]
    IoError(#[from] std::io::Error),
}

pub fn next_frame<R: Read, W: Write>(r: &mut R, w: &mut W) -> Result<Option<bool>, Error> {
    let sword = r.read_u16::<BigEndian>()?;

    if G192_MAGICS.iter().all(|m| *m != sword) {
        return Err(Error::NotG192);
    }

    // Bad frame indicator
    let bfi = sword == G192_MAGICS[0];

    let len = r.read_u16::<LittleEndian>().map(|l| l as usize)?;
    for _ in 0..len {
        let b = r.read_u16::<LittleEndian>()?;
        let b = match b {
            b if b == (Bit::B0 as u16) => 0,
            b if b == (Bit::B1 as u16) => 1,
            _ => return Err(Error::InvalidBit),
        };
        w.write_all(&[b])?;
    }

    Ok(Some(bfi))
}
