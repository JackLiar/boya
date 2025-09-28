use std::io::{Error as IOError, Result as IOResult, Write};

use bitvec::{field::BitField, order::Msb0, view::BitView};
use derivative::Derivative;

mod consts;
mod rpe;
mod utils;

#[derive(Clone, Debug, Derivative)]
#[derivative(Default)]
pub struct Parameters {
    pub lar_c: [i16; 8],
    pub nc: [i16; 4],
    pub mc: [i16; 4],
    pub bc: [i16; 4],
    pub xmaxc: [i16; 4],
    #[derivative(Default(value = "[0i16; 52]"))]
    pub xmc: [i16; 52],
}

impl Parameters {
    pub fn decode(data: &[u8; 33]) -> IOResult<Self> {
        if (data[0] >> 4) & 0x0f != 0x0d {
            return Err(IOError::other("Invalid MAGIC number for GSM 6.10"));
        }

        let mut p = Self::default();
        let mut data = data.view_bits::<Msb0>();

        p.lar_c[0] = data[4..10].load_be::<i16>();
        p.lar_c[1] = data[10..16].load_be::<i16>();
        p.lar_c[2] = data[16..21].load_be::<i16>();
        p.lar_c[3] = data[21..26].load_be::<i16>();
        p.lar_c[4] = data[26..30].load_be::<i16>();
        p.lar_c[5] = data[30..34].load_be::<i16>();
        p.lar_c[6] = data[34..37].load_be::<i16>();
        p.lar_c[7] = data[37..40].load_be::<i16>();
        data = &data[40..];
        for i in 0..4 {
            p.nc[i] = data[0..7].load_be::<i16>();
            p.bc[i] = data[7..9].load_be::<i16>();
            p.mc[i] = data[9..11].load_be::<i16>();
            p.xmaxc[i] = data[11..17].load_be::<i16>();
            p.xmc[i * 13] = data[17..20].load_be::<i16>();
            p.xmc[1 + i * 13] = data[20..23].load_be::<i16>();
            p.xmc[2 + i * 13] = data[23..26].load_be::<i16>();
            p.xmc[3 + i * 13] = data[26..29].load_be::<i16>();
            p.xmc[4 + i * 13] = data[29..32].load_be::<i16>();
            p.xmc[5 + i * 13] = data[32..35].load_be::<i16>();
            p.xmc[6 + i * 13] = data[35..38].load_be::<i16>();
            p.xmc[7 + i * 13] = data[38..41].load_be::<i16>();
            p.xmc[8 + i * 13] = data[41..44].load_be::<i16>();
            p.xmc[9 + i * 13] = data[44..47].load_be::<i16>();
            p.xmc[10 + i * 13] = data[47..50].load_be::<i16>();
            p.xmc[11 + i * 13] = data[50..53].load_be::<i16>();
            p.xmc[12 + i * 13] = data[53..56].load_be::<i16>();
            data = &data[56..];
        }
        debug_assert!(data.is_empty());
        Ok(p)
    }

    pub fn get_param(&self, idx: usize) -> Option<()> {
        if idx >= 4 {
            return None;
        }
        todo!()
    }
}

#[derive(Clone, Debug, Derivative)]
#[derivative(Default)]
pub struct Decoder {
    #[derivative(Default(value = "[0i16; 280]"))]
    pub dp0: [i16; 280],
    #[derivative(Default(value = "[0i16; 50]"))]
    pub e: [i16; 50],
    pub z1: i16,
    pub z2: i32,
    pub mp: i32,
    pub u: [i16; 8],
    pub larpp: [[i16; 8]; 2],
    pub j: i16,
    pub ltp_cut: i16,
    #[derivative(Default(value = "40"))]
    pub nrp: i16,
    pub v: [i16; 9],
    pub msr: i16,
    pub verbose: bool,
    pub fast: bool,
    pub frame_idx: usize,
    pub frame_chain: usize,
}

impl Decoder {
    fn long_term_synthesis_filtering(&mut self, ncr: i16, bcr: i16, erp: &[i16; 40], drp: &[i16]) {}

    fn short_term_synthesis_filtering(&mut self, larcr: i16, wt: i16, s: &[i16; 40]) {}

    fn post_processing<W: Write>(&mut self, w: &mut W) {
        for i in 0..160 {
            // let tmp = self.msr * 28180; // TODO
            // let msr =
        }
    }

    pub fn decode<W: Write>(&mut self, data: &[u8; 33], w: &mut W) -> IOResult<usize> {
        let param = Parameters::decode(data)?;
        let mut erp = [0i16; 40];
        let mut wt = [0i16; 160];
        let mut drp = &mut self.dp0[120..];

        for i in 0..4 {
            // self.rpe_decode(xmaxcr, mcr, xmcr, erp);
            // self.long_term_synthesis_filtering(ncr, bcr, erp, drp);
            for j in 0..40 {
                wt[i * 40 + j] = drp[j];
            }
        }

        // self.short_term_synthesis_filtering(larcr, wt, s);
        // self.post_processing(w);

        Ok(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_parameters() {
        let data = &[
            0xd4, 0x33, 0xd9, 0xd7, 0x12, 0x77, 0x27, 0x0c, 0x9a, 0x92, 0xdc, 0x63, 0x76, 0xa5,
            0x25, 0x27, 0xe4, 0xc4, 0x95, 0xb1, 0x45, 0xed, 0x45, 0x6d, 0x47, 0x6a, 0xb1, 0xa3,
            0x8a, 0xa3, 0x8e, 0x53, 0x23,
        ];
        let p = Parameters::decode(data).unwrap();
    }
}
