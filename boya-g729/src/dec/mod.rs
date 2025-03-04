use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};

use crate::{L_FRAME, L_INTERPOL, M, PIT_MAX};

pub mod float;
mod lpc;
mod lsp;
pub mod param;
mod post_filter;
mod post_process;

use param::Parameter;
use post_filter::PostFilter;
use post_process::PostProcess;

pub const SHARP_MAX: i16 = 13017;
pub const SHARP_MIN: i16 = 3277;
pub const LSP: [i16; 10] = [
    30000, 26000, 21000, 15000, 8000, 0, -8000, -15000, -21000, -26000,
];

#[derive(Clone, Debug)]
pub struct G729Decoder {
    /// Decoder parameters
    pub param: Parameter,
    /// Excitation vector
    pub exc: [i16; L_FRAME + PIT_MAX + L_INTERPOL],
    pub mem_syn: [i16; 10],
    pub sharp: i16,
    pub old_t0: i16,
    pub gain_code: i16,
    pub gain_pitch: i16,
    pub freq_prev_reset: [i16; 10],
    pub freq_prev: [[i16; 10]; 4],
    /// Previous MA prediction coef
    pub prev_ma: i16,
    pub prev_lsp: [i16; 10],
    pub lsp_old: [i16; 10],
    pub voicing: i16,
    pub postf: PostFilter,
    pub postp: PostProcess,
}

impl Default for G729Decoder {
    fn default() -> Self {
        let mut dec = Self {
            param: Default::default(),
            exc: [0; L_FRAME + PIT_MAX + L_INTERPOL],
            mem_syn: [0; 10],
            sharp: SHARP_MIN,
            old_t0: 60,
            gain_code: 0,
            gain_pitch: 0,
            freq_prev_reset: [
                2339, 4679, 7018, 9358, 11698, 14037, 16377, 18717, 21056, 23396,
            ],
            freq_prev: [[0; 10]; 4],
            prev_ma: 0,
            prev_lsp: [0; 10],
            lsp_old: [
                30000, 26000, 21000, 15000, 8000, 0, -8000, -15000, -21000, -26000,
            ],
            voicing: 60,
            postf: PostFilter::default(),
            postp: PostProcess::default(),
        };
        dec.lsp_decw_reset();

        dec
    }
}

impl G729Decoder {
    pub fn decode<W: Write>(&mut self, data: &[u8; M], w: &mut W) -> std::io::Result<usize> {
        let mut pst_out = [0i16; L_FRAME];

        self.decode_ld8k(data);

        self.voicing = 0;
        self.post_filter();

        self.post_process();

        for x in pst_out {
            w.write_i16::<BigEndian>(x)?;
        }

        Ok(0)
    }

    fn decode_ld8k(&mut self, data: &[u8; M]) {
        let mut lsp_new = [0i16; M];
        self.param = Parameter::from(data);

        // Decode the LSPs
        self.lsp_decode(&mut lsp_new);

        todo!("Interpolation of LPC for the 2 subframes");

        self.lsp_old = lsp_new;

        todo!("Loop for every subframe in the analysis frame");

        todo!("Find the adaptive codebook vector");

        todo!("Decode innovative codebook");

        todo!("Add the fixed-gain pitch contribution to code[]");

        todo!("Decode pitch and codebook gains");
    }

    fn post_filter(&mut self) {}

    fn post_process(&mut self) {}
}
