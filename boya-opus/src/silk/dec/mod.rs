use std::io::Write;

use super::{Channels, FrameDuration, InternalSampleRate, SampleRate};

pub struct CNG {
    pub exc_buf_q14: [i32; 320],
    pub smth_nlsf_q15: [i16; 16],
    pub synth_state: [i32; 16],
    pub smth_gain_q16: i32,
    pub rand_seed: i32,
    pub fs_khz: u8,
}

impl CNG {
    pub fn reset(&mut self, cnt: usize) {
        let nlsf_step = (0x7FFF) / (cnt as i16 + 1);
        let mut nlsf_acc_q15 = 0;
        for i in 0..cnt {
            nlsf_acc_q15 += nlsf_step;
            self.smth_nlsf_q15[i] = nlsf_acc_q15;
        }
        self.smth_gain_q16 = 0;
        self.rand_seed = 3176576;
    }
}

/// Decoder control parameters
#[derive(Clone, Copy, Debug)]
pub struct Resampler {
    pub siir: [i32; 6],
    pub delay_buf: [i16; 48],
    pub resampler_function: i32,
    pub batch_size: usize,
    pub inv_ratio_q16: i32,
    pub fir_order: i32,
    pub fir_fracs: i32,
    pub fs_in_khz: u16,
    pub fs_out_khz: u16,
    pub input_delay: i32,
}

impl Default for Resampler {
    fn default() -> Self {
        Self {
            siir: [0; 6],
            delay_buf: [0; 48],
            resampler_function: 0,
            batch_size: 0,
            inv_ratio_q16: 0,
            fir_order: 0,
            fir_fracs: 0,
            fs_in_khz: 0,
            fs_out_khz: 0,
            input_delay: 0,
        }
    }
}

/// Decoder control parameters
#[derive(Clone, Copy, Debug, Default)]
pub struct ControlParameter {
    pub channels: Channels,
    pub channels_internal: Channels,
    pub sample_rate: SampleRate,
    pub sample_rate_internal: InternalSampleRate,
    pub frame_duration: FrameDuration,
    pub prev_pitch_lag: i32,
    pub enable_deep_plc: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Channel {
    pub prev_gain_q16: i32,
    // pub exc_q145: [i32; 320],
    pub s_lpc_q14_buf: [i32; 16],
    // pub out_buf: [i16; 480],
    pub lag_prev: i32,
    pub last_gain_idx: i8,
    pub sample_rate: SampleRate,
    pub fs_api_hz: i32,
    pub nb_subfr: usize,
    pub frame_len: i32,
    pub subfr_len: i32,
    pub ltp_mem_length: i32,
    pub lpc_order: i32,
    pub prev_nlsf_q15: [i16; 16],
    pub first_frame_after_reset: bool,
    // pub pitch_lag_low_bits_icdf:
    // pub pitch_contour_icdf:
    pub decoded_frames_num: i32,
    pub frames_per_pkt: usize,
    pub ec_prev_signal_type: i32,
    pub ec_prev_lag_index: i32,
    pub vad_flags: [i32; 3],
    pub lbrr_flag: i32,
    pub lbrr_flags: [i32; 3],
    pub resampler: Resampler,

    pub loss_cnt: u32,
    pub prev_signal_type: i32,
    pub arch: i32,
}

impl Channel {
    pub fn reset(&mut self) {
        self.first_frame_after_reset = false;
        self.prev_gain_q16 = 65536;
    }

    fn set_sample_rate(&mut self, fs_khz: u16, fs_api_hz: u32) {}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Decoder {
    pub channels: [Channel; 2],
    // stereo fields
    pub pred_prev_q13: [i16; 2],
    pub smid: [i16; 2],
    pub sside: [i16; 2],

    pub prev_decode_only_middle: bool,
    pub parameters: ControlParameter,
}

impl Decoder {
    pub fn init(&mut self) {}

    pub fn reset(&mut self) {
        self.pred_prev_q13 = [0; 2];
        self.smid = [0; 2];
        self.sside = [0; 2];
        self.prev_decode_only_middle = false;
    }

    fn channels_mut(&mut self, channels: Channels) -> impl Iterator<Item = &mut Channel> {
        self.channels.iter_mut().take(channels as usize)
    }

    pub fn decode<W: Write>(
        &mut self,
        ctl: &ControlParameter,
        lost: bool,
        first_pkt: bool,
        w: &mut W,
    ) {
        if first_pkt {
            for chl in self.channels_mut(ctl.channels) {
                chl.decoded_frames_num = 0;
            }
        }

        if ctl.channels_internal > self.parameters.channels_internal {
            todo!("init self.channels[1]");
        }

        let stereo2mono = ctl.channels_internal == Channels::Mono
            && self.parameters.channels_internal == Channels::Stereo
            && (ctl.sample_rate_internal as u32 == self.channels[0].sample_rate as u32);

        if self.channels[0].decoded_frames_num == 0 {
            for chl in self.channels_mut(ctl.channels) {
                match ctl.frame_duration {
                    FrameDuration::Loss | FrameDuration::Ms10 => {
                        chl.frames_per_pkt = 1;
                        chl.nb_subfr = 2;
                    }
                    FrameDuration::Ms20 => {
                        chl.frames_per_pkt = 1;
                        chl.nb_subfr = 4;
                    }
                    FrameDuration::Ms40 => {
                        chl.frames_per_pkt = 2;
                        chl.nb_subfr = 4;
                    }
                    FrameDuration::Ms60 => {
                        chl.frames_per_pkt = 3;
                        chl.nb_subfr = 4;
                    }
                }
                let fs_kHz_dec = (ctl.sample_rate_internal as u16 >> 10) + 1;
                chl.set_sample_rate(fs_kHz_dec, 0);
            }
        }

        if ctl.channels == Channels::Stereo
            && ctl.channels_internal == Channels::Stereo
            && self.parameters.channels == Channels::Mono
            && self.parameters.channels_internal == Channels::Mono
        {
            self.pred_prev_q13 = [0; 2];
            self.sside = [0; 2];
        }

        self.parameters.channels = ctl.channels;
        self.parameters.channels_internal = ctl.channels_internal;

        if !lost && self.channels[0].decoded_frames_num == 0 {
            for chl in self.channels_mut(ctl.channels) {
                for flag in chl.vad_flags.iter_mut().take(chl.frames_per_pkt) {}
            }
        }
    }
}
