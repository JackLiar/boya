use std::{
    fs::File,
    io::{BufReader, ErrorKind, Read},
};

use anyhow::{bail, Result};
use byteorder::{BigEndian, ReadBytesExt};
use log::{error, info};

use boya_opus::dec::OpusDecoder;

use crate::Config;

pub fn decode(cfg: Config) -> Result<()> {
    let mut decoder = OpusDecoder::new(cfg.sample_rate, cfg.channels);
    decoder.complexity = cfg.complexity;

    info!(
        "Decoding with {} Hz ({:?} channels)",
        cfg.sample_rate as u16, cfg.channels
    );

    let mut infile = BufReader::new(File::open(&cfg.input_file)?);
    let mut outfile = BufReader::new(File::create(&cfg.output_file)?);
    let mut pcm = vec![];

    loop {
        let len = match infile.read_u32::<BigEndian>() {
            Ok(l) => l,
            Err(e) => {
                if e.kind() == ErrorKind::UnexpectedEof {
                    break;
                } else {
                    error!("{}", e);
                    bail!(e);
                }
            }
        };
        let enc_final_range = infile.read_u32::<BigEndian>()?;

        let mut data = vec![0; len as usize];
        infile.read_exact(&mut data)?;

        decoder.decode(&data, &mut pcm)?;
    }

    println!("average bitrate: ");
    println!("maxmium bitrate: ");
    println!("bitrate standard deviation: ");
    Ok(())
}
