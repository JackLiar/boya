use std::path::PathBuf;

use clap::{value_parser, Arg, ArgAction, ArgGroup, Command};

pub fn cli() -> Command {
    let mut cmd = Command::new("opus-demo")
        .arg(
            Arg::new("decode")
                .short('d')
                .action(ArgAction::SetTrue)
                .conflicts_with("encode")
                .help("only runs the encoder (output the bit-stream)"),
        )
        .arg(
            Arg::new("encode")
                .short('e')
                .action(ArgAction::SetTrue)
                .conflicts_with("decode")
                .help("only runs the decoder (reads the bit-stream as input)"),
        )
        .arg(
            Arg::new("sample-rate")
                .required(true)
                .value_name("sample rate (Hz)")
                .value_parser(clap::builder::PossibleValuesParser::new([
                    "8000", "12000", "16000", "24000", "48000",
                ])),
        )
        .arg(
            Arg::new("channels")
                .required(true)
                .value_name("channels (1/2)")
                .value_parser(clap::builder::PossibleValuesParser::new(["1", "2"])),
        )
        .arg(
            Arg::new("constant-bitrate")
                .long("cbr")
                .help("enable constant bitrate")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("constrained-variable-bitrate")
                .long("cvbr")
                .help("enable constrained variable bitrate")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("bandwidth")
                .long("bandwidth")
                .value_name("audio bandwidth (from narrowband to fullband)")
                .value_parser(clap::builder::PossibleValuesParser::new([
                    "NB", "MB", "WB", "SWB", "FB",
                ])),
        )
        .arg(
            Arg::new("framesize")
                .long("framesize")
                .value_name("frame size in ms")
                .value_parser(clap::builder::PossibleValuesParser::new([
                    "2.5", "5", "10", "20", "40", "60", "80", "100", "120",
                ]))
                .default_value("20"),
        )
        .arg(
            Arg::new("max-payload")
                .long("max-payload")
                .value_name("maximum payload size in bytes")
                .value_parser(value_parser!(usize))
                .default_value("1024"),
        )
        .arg(
            Arg::new("complexity")
                .long("complexity")
                .value_name("encoder/decoder complexity, 0 (lowest) ... 10 (highest)")
                .value_parser(value_parser!(u8).range(0..=10))
                .default_value("0"),
        )
        .arg(
            Arg::new("inbandfec")
                .long("inbandfec")
                .help("enable SILK inband FEC")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("forcemono")
                .long("forcemono")
                .help("force mono encoding, even for stereo input")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("delayed-decision")
                .long("delayed-decision")
                .help("use look-ahead for speech/music detection (experts only)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("dtx")
                .long("dtx")
                .help("enable SILK DTX")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("loss")
                .long("loss")
                .help("optimize for loss percentage and simulate packet loss, in percent (0-100)")
                .value_parser(value_parser!(u8).range(0..=100))
                .default_value("0"),
        )
        .arg(
            Arg::new("loss-file")
                .long("lossfile")
                .help("simulate packet loss, reading loss from file")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("dred")
                .long("dred")
                .help("add Deep REDundancy (in units of 10-ms frames)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("sweep")
                .long("sweep")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("random-framesize")
                .long("random-framesize")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("sweep-max")
                .long("sweep-max")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("random-fec")
                .long("random-fec")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("infile")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("outfile")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .group(
            ArgGroup::new("encode-group")
                .arg("constant-bitrate")
                .arg("constant-bitrate")
                .arg("bandwidth")
                .arg("framesize")
                .arg("max-payload")
                .arg("forcemono")
                .arg("delayed-decision")
                .arg("dtx")
                .arg("sweep")
                .arg("random-framesize")
                .arg("sweep-max")
                .arg("random-fec"),
        )
        .group(ArgGroup::new("decode-group").arg("forcemono"));
    cmd.build();
    cmd
}
