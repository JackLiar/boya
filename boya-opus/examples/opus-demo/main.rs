use std::path::PathBuf;

use anyhow::{bail, Result};
use boya_opus::{Channels, Complexity, SampleRate};

mod cli;
mod dec;

#[derive(Clone, Debug, Default)]
struct Config {
    channels: Channels,
    complexity: Complexity,
    dtx: bool,
    encode_only: bool,
    frame_size: usize,
    input_file: PathBuf,
    max_frame_size: usize,
    output_file: PathBuf,
    packet_loss: u8,
    packet_loss_file: Option<PathBuf>,
    sample_rate: SampleRate,
    sweep: u32,
    sweep_min: u32,
    sweep_max: u32,
    use_inbandfec: bool,
    use_vbr: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    let cmd = cli::cli();
    let matches = cmd.get_matches();

    let mut cfg = Config::default();

    if matches.get_flag("encode") {
        cfg.encode_only = true;
    }
    if matches.get_flag("decode") {
        cfg.encode_only = false;
    }

    cfg.sample_rate = match matches.get_one::<String>("sample-rate").unwrap().as_str() {
        "8000" => SampleRate::Fs8000,
        "12000" => SampleRate::Fs12000,
        "16000" => SampleRate::Fs16000,
        "24000" => SampleRate::Fs24000,
        "48000" => SampleRate::Fs48000,
        _ => bail!("Invalid sample rate value"),
    };
    cfg.frame_size = cfg.sample_rate as usize / 50;
    cfg.channels = match matches.get_one::<String>("channels").unwrap().as_str() {
        "1" => Channels::Mono,
        "2" => Channels::Stereo,
        _ => bail!("Invalid channelds"),
    };

    cfg.use_vbr = matches.get_flag("constant-bitrate");
    let mut max_payload_bytes = 1500usize;
    cfg.complexity = match matches.get_one::<u8>("complexity") {
        None => bail!("Invalid complexity"),
        Some(c) => match c {
            0 => Complexity::C0,
            1 => Complexity::C1,
            2 => Complexity::C2,
            3 => Complexity::C3,
            4 => Complexity::C4,
            5 => Complexity::C5,
            6 => Complexity::C6,
            7 => Complexity::C7,
            8 => Complexity::C8,
            9 => Complexity::C9,
            10 => Complexity::C10,
            _ => unreachable!("Shoud never happends"),
        },
    };
    cfg.use_inbandfec = matches.get_flag("inbandfec");
    // forcechannels = OPUS_AUTO;
    cfg.dtx = matches.get_flag("dtx");
    if let Some(sweep) = matches.get_one::<u32>("sweep") {
        cfg.sweep = *sweep;
    }
    if let Some(max) = matches.get_one::<u32>("sweep-max") {
        cfg.sweep_max = *max;
    }
    cfg.packet_loss = *matches.get_one::<u8>("loss").unwrap();
    cfg.packet_loss_file = matches.get_one::<PathBuf>("loss-file").cloned();
    cfg.input_file = matches.get_one::<PathBuf>("infile").unwrap().clone();
    cfg.output_file = matches.get_one::<PathBuf>("outfile").unwrap().clone();

    cfg.max_frame_size = 48000 * cfg.channels as usize;
    println!("cfg: {:?}", cfg);

    if !cfg.encode_only {
        dec::decode(cfg)
    } else {
        Ok(())
    }
}
