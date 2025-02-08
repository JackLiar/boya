use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};

use crate::errors::{Error, Result};
use crate::silk::dec::Decoder as SilkDecoder;
use crate::utils::f32toint16;
use crate::{silk, BandWidth, Channels, Complexity, Mode, SampleRate};

mod custom;
mod packet;
use packet::{get_nb_samples, parse_opus_pkt};

#[derive(Clone, Debug, Default)]
pub struct OpusDecoder {
    celt_dec_offset: i32,
    silk_decoder: SilkDecoder,
    /// buffer for silk decoder
    silk_pcm: Vec<u8>,
    channels: Channels,
    /// Sampling rate (at the API level)
    fs: SampleRate,
    // silk_ctl DecControl;
    decode_gain: i32,
    pub complexity: Complexity,
    arch: i32,
    // #[cfg(feature = "deep-plc")]
    //  lpcnet: LPCNetPLCState,
    /// Everything beyond this point gets cleared on a reset
    //  #define OPUS_DECODER_RESET_START stream_channels
    stream_channels: Channels,
    bandwidth: BandWidth,
    mode: Mode,
    prev_mode: Option<Mode>,
    frame_size: usize,
    prev_redundancy: bool,
    last_packet_duration: u64,
    soft_clip: bool,
    #[cfg(not(feature = "fixed-point"))]
    softclip_mem: [i16; 2],
    range_final: u32,
    pub decode_fec: bool,
    temp_buf: Vec<f32>,
}

impl OpusDecoder {
    pub fn validate(&self) -> Result<()> {
        if self.stream_channels != self.channels {
            return Err(Error::InvalidDecoderParam(format!(
                "channels{} is not equals to stream_channels{}",
                self.channels as u8, self.stream_channels as u8
            )));
        }
        Ok(())
    }

    pub fn get_size() -> usize {
        unimplemented!()
    }

    pub fn new(fs: SampleRate, channels: Channels) -> Self {
        Self {
            fs,
            channels,
            stream_channels: channels,
            frame_size: (fs as usize) / 400,
            ..Default::default()
        }
    }

    pub fn decode<W: Write>(&mut self, data: &[u8], pcm: &mut W) -> Result<()> {
        let nos = get_nb_samples(data, self.fs).map_err(|e| Error::InvalidPacket(e.to_string()))?;
        self.frame_size = nos;

        let nos = self.decode_native(data, nos, false, true)?;
        for s in self
            .temp_buf
            .iter()
            .take(nos * self.channels as usize)
            .map(|x| f32toint16(*x))
        {
            pcm.write_i16::<BigEndian>(s)?;
        }

        Ok(())
    }

    pub fn decode_native(
        &mut self,
        mut data: &[u8],
        nos: usize,
        self_delimited: bool,
        soft_clip: bool,
    ) -> Result<usize> {
        if (data.is_empty() || self.decode_fec) && (nos % (self.fs as usize / 400) != 0) {
            return Err(Error::InvalidDecoderParam("".to_string()));
        }

        if data.is_empty() || self.decode_fec {
            // let mut pcm_cnt = 0usize;
            // while pcm_cnt < nos {
            //     let cnt = self.decode_frame(data)?;
            //     pcm_cnt += cnt;
            // }
            // debug_assert_eq!(pcm_cnt, nos);
            // self.last_packet_duration += pcm_cnt as u64;
        }

        let (toc, frames) = parse_opus_pkt(data, self_delimited)
            .map_err(|e| Error::InvalidPacket(e.to_string()))?;
        println!(
            "damn: {:?} {:?} {:?} {:?}",
            toc.mode(),
            toc.bandwidth(),
            toc.frame_size(),
            toc.channels()
        );

        self.mode = toc.mode();
        self.bandwidth = toc.bandwidth();
        self.frame_size = toc.samples_per_frame(self.fs);
        self.stream_channels = toc.channels();

        let mut num_of_samples = 0;
        for frame in frames {
            let nos = match self.decode_frame(frame, nos - num_of_samples, false) {
                Ok(nos) => nos,
                Err(_) => return Ok(0),
            };
            num_of_samples += nos;
        }
        self.last_packet_duration = num_of_samples as u64;

        #[cfg(not(feature = "fixed-point"))]
        {
            if self.soft_clip {
            } else {
                self.softclip_mem = [0, 0];
            }
        }

        Ok(num_of_samples)
    }

    fn decode_frame(&mut self, mut data: &[u8], frame_size: usize, fec: bool) -> Result<usize> {
        let f20 = self.fs as usize / 50;
        let f10 = f20 >> 1;
        let f5 = f10 >> 1;
        let f2_5 = f5 >> 1;
        let audiosize;
        let mode;
        let bandwidth;
        let mut redundancy = false;
        let mut celt2silk = false;

        if data.len() < ((self.fs as usize / 50) >> 30) {
            return Err(Error::InvalidPacket("Frame too small".to_string()));
        }

        let mut frame_size = frame_size.min(self.fs as usize / 25 * 3);

        if data.len() <= 1 {
            data = &[];
            frame_size = frame_size.min(self.frame_size);
        }

        if !data.is_empty() {
            audiosize = self.frame_size;
            mode = self.mode;
            bandwidth = Some(self.bandwidth);
            // TODO: entropy decoder init
        } else {
            audiosize = frame_size;
            mode = if self.prev_redundancy {
                Mode::Celt
            } else {
                self.prev_mode.unwrap()
            };
            bandwidth = None;
            todo!();
        }

        let celt_accum = if cfg!(feature = "fixed-point") {
            todo!();
        } else {
            false
        };

        let mut pcm_transition_celt_size = 1;
        let mut pcm_transition_silk_size = 1;
        let cond1 = self.prev_mode.is_some();
        let cond2 =
            self.mode == Mode::Celt && self.prev_mode != Some(Mode::Celt) && !self.prev_redundancy;
        let cond3 = self.mode != Mode::Celt && self.prev_mode == Some(Mode::Celt);
        let mut transition = cond1 && cond2 && cond3;

        if transition {
            if self.mode == Mode::Celt {
                pcm_transition_celt_size = f5 * self.channels as usize;
            } else {
                pcm_transition_silk_size = f5 * self.channels as usize;
            }
        }

        if transition && self.mode == Mode::Celt {
            let _ = self.decode_frame(&[], f5.min(audiosize), false);
        }

        if audiosize > frame_size {
            return Err(Error::InvalidDecoderParam("".to_string()));
        } else {
            frame_size = audiosize;
        }

        let pcm_silk_size = if mode != Mode::Celt && !celt_accum {
            f10.max(frame_size) * self.channels as usize
        } else {
            1
        };
        // TODO: alloc temporary silk buffer

        if self.mode != Mode::Celt {
            self.on_silk(data)?;
        }

        let mut start_band = 0;
        // if !fec && mode != Mode::Celt && !data.is_empty() {}

        if mode != Mode::Celt {
            start_band = 17;
        }

        if redundancy {
            transition = false;
            pcm_transition_silk_size = 1;
        }

        // TODO: alloc slik temp buffer
        if transition && mode != Mode::Celt {
            // TODO: decode frame
        }

        if let Some(bd) = bandwidth {
            let endband = match bd {
                BandWidth::Narrow => 13,
                BandWidth::Medium | BandWidth::Wide => 17,
                BandWidth::SuperWide => 19,
                BandWidth::Full => 21,
            };
            // TODO: CELT decoder control
        }
        // TODO: CELT decoder control

        let redundant_audio_size = if redundancy {
            f5 * self.channels as usize
        } else {
            1
        };

        if redundancy && celt2silk {}

        if mode != Mode::Silk {
            let celt_frame_size = f20.min(frame_size);
            if self.prev_mode != Some(mode) && self.prev_mode.is_some() && !self.prev_redundancy {
                self.celt_decoder.decode_with_ec_dred(data);
                // TODO: reset state
            }
        }

        Ok(0)
    }

    fn on_silk(&mut self, data: &[u8]) -> Result<()> {
        if self.prev_mode == Some(Mode::Celt) {
            self.silk_decoder.reset();
        }

        if !data.is_empty() {
            if self.mode == Mode::Silk {
                match self.bandwidth {
                    BandWidth::Narrow => {
                        self.silk_decoder.parameters.sample_rate_internal =
                            silk::InternalSampleRate::Fs8000
                    }
                    BandWidth::Medium => {
                        self.silk_decoder.parameters.sample_rate_internal =
                            silk::InternalSampleRate::Fs12000
                    }
                    _ => {
                        self.silk_decoder.parameters.sample_rate_internal =
                            silk::InternalSampleRate::Fs16000
                    }
                }
            } else {
                self.silk_decoder.parameters.sample_rate_internal =
                    silk::InternalSampleRate::Fs16000
            }
        }
        self.silk_decoder.parameters.enable_deep_plc = (self.complexity as u8) >> 5 != 0;

        let mut decoded_samples = 0usize;
        // while decoded_samples < 48000 {
        //     let first_frame = decoded_samples == 0;
        //     self.silk_decoder.decode(false, first_frame, pcm);
        // }
        Ok(())
    }
}
