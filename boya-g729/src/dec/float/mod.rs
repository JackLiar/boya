use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};
use lp::LinearPrediction;

use super::param::Parameter;
use crate::M;

mod consts;
mod lp;

#[derive(Clone, Debug)]
pub struct G729Decoder {
    pub param: Parameter,
    pub voicing: i16,
    pub lp: LinearPrediction,
}

impl Default for G729Decoder {
    fn default() -> Self {
        Self {
            param: Parameter::default(),
            voicing: 0,
            lp: LinearPrediction::new(),
        }
    }
}

impl G729Decoder {
    pub fn decode<W: Write>(&mut self, data: &[u8; 10], w: &mut W) -> std::io::Result<usize> {
        let mut pst_out = [0i16; 80];

        self.decode_ld8k(data);

        self.voicing = 0;
        self.post_filter();

        self.post_process();

        for x in pst_out {
            w.write_i16::<BigEndian>(x)?;
        }

        Ok(0)
    }

    fn decode_ld8k(&mut self, data: &[u8; 10]) {
        let mut lsp_new = [0.0f64; M];
        self.param = Parameter::from(data);

        // Decode the LSPs
        self.lp.decode_lsp(&self.param, &mut lsp_new);

        todo!("Interpolation of LPC for the 2 subframes");

        // self.lsp_old = lsp_new;

        // todo!("Loop for every subframe in the analysis frame");

        // todo!("Find the adaptive codebook vector");

        // todo!("Decode innovative codebook");

        // todo!("Add the fixed-gain pitch contribution to code[]");

        // todo!("Decode pitch and codebook gains");
    }

    fn post_filter(&mut self) {}

    fn post_process(&mut self) {}
}
