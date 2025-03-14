use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};
use pitch::dec_lag3;

use super::param::Parameter;
use crate::{M, PIT_MAX};

mod consts;
mod lp;
mod pitch;
mod post_filter;
mod post_process;

use lp::LinearPrediction;
use post_process::PostProcess;

#[derive(Clone, Debug)]
pub struct G729Decoder {
    pub param: Parameter,
    pub voicing: i16,
    pub old_t0: i32,
    pub t0_first: i32,
    pub lp: LinearPrediction,
    pub post_process: PostProcess,
}

impl Default for G729Decoder {
    fn default() -> Self {
        Self {
            param: Parameter::default(),
            voicing: 0,
            old_t0: 60,
            t0_first: 0,
            lp: LinearPrediction::new(),
            post_process: Default::default(),
        }
    }
}

impl G729Decoder {
    pub fn decode<W: Write>(&mut self, data: &[u8; M], w: &mut W) -> std::io::Result<usize> {
        let mut pst_out = [0.0f64; 80];

        self.decode_ld8k(data);

        self.voicing = 0;
        self.post_filter();

        self.post_process(&mut pst_out);

        for mut x in pst_out {
            if x >= 0.0 {
                x += 0.5
            } else {
                x -= 0.5;
            }
            x = x.clamp(-32768.0, 32767.0);
            w.write_i16::<BigEndian>(x as i16)?;
        }

        Ok(0)
    }

    fn decode_ld8k(&mut self, data: &[u8; M]) {
        let mut lsp_new = [0.0f64; M];
        let mut az = [[0.0f64; M + 1]; 2];
        self.param = Parameter::from(data);

        // Decode the LSPs
        self.lp.decode_lsp(&self.param, &mut lsp_new);

        self.lp.int_qlpc(&lsp_new, &mut az);

        self.lp.lsp_old = lsp_new;

        let mut t0 = 0;
        let mut t0frac = 0;
        for i in (0..80).step_by(40) {
            if i == 0 {
                let bad_pitch = self.param.bfi || self.param.p0;
                if !bad_pitch {
                    dec_lag3(&self.param, i == 0, &mut t0, &mut t0frac);
                    self.old_t0 = t0;
                } else {
                    t0 = self.old_t0;
                    t0frac = 0;
                    self.old_t0 += 1;
                    if self.old_t0 > PIT_MAX {
                        self.old_t0 = PIT_MAX;
                    }
                }
                self.t0_first = t0;
            } else if !self.param.bfi {
                dec_lag3(&self.param, i == 0, &mut t0, &mut t0frac);
                self.old_t0 = t0;
            } else {
                t0 = self.old_t0;
                t0frac = 0;
                self.old_t0 += 1;

                if self.old_t0 > PIT_MAX {
                    self.old_t0 = PIT_MAX;
                }
            }
        }

        // todo!("Find the adaptive codebook vector");

        // todo!("Decode innovative codebook");

        // todo!("Add the fixed-gain pitch contribution to code[]");

        // todo!("Decode pitch and codebook gains");
    }

    fn post_filter(&mut self) {}

    fn post_process(&mut self, signal: &mut [f64; 80]) {
        self.post_process.post_process(signal)
    }
}
