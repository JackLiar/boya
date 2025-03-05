use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};
use lp::LinearPrediction;
use post_process::PostProcess;

use super::param::Parameter;
use crate::M;

mod consts;
mod lp;
mod post_process;

#[derive(Clone, Debug)]
pub struct G729Decoder {
    pub param: Parameter,
    pub voicing: i16,
    pub lp: LinearPrediction,
    pub post_process: PostProcess,
}

impl Default for G729Decoder {
    fn default() -> Self {
        Self {
            param: Parameter::default(),
            voicing: 0,
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

        // todo!("Loop for every subframe in the analysis frame");

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
