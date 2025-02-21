use std::fs::File;
use std::io::{BufReader, ErrorKind, Write};
use std::path::PathBuf;

use anyhow::{anyhow, Result};

use boya_g192::bitvec::prelude::*;
use boya_g192::{next_frame_bits, Error};
use boya_g729::dec::G729Decoder;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let input = PathBuf::from(args.get(1).ok_or(anyhow!("No input file specified"))?);
    let output = PathBuf::from(args.get(2).ok_or(anyhow!("No output file specified"))?);

    let mut file = BufReader::new(File::open(&input).unwrap());
    let mut exit = false;
    let mut buf = [0u8; 10];
    let mut decoded = vec![];
    let mut bits = BitVec::<u8, Msb0>::new();
    while !exit {
        buf.fill(0);
        bits.clear();
        match next_frame_bits(&mut file, &mut bits) {
            Ok(bfi) => bfi,
            Err(Error::IoError(e)) => match e.kind() {
                ErrorKind::UnexpectedEof => {
                    exit = true;
                    break;
                }
                _ => panic!("{e}"),
            },
            Err(e) => panic!("{e}"),
        };

        let mut bits = bits.as_bitslice();
        for b in buf.iter_mut() {
            let (byte, rem) = bits.split_at(8);
            bits = rem;
            let byte = byte.load::<u8>();
            *b = byte;
        }

        let mut decoder = G729Decoder::default();
        decoder.decode(&buf, &mut decoded)?;
    }
    Ok(())
}
