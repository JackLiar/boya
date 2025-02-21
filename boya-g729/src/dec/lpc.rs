use std::ops::{Shl, Shr};

use saturating_cast::SaturatingCast;

use crate::basic_ops::{add_i16, extract_l_i32, mul_i16, mul_l_i16, shr_i16, sub_i16};
use crate::{M, SLOPE_COS, TABLE2};

/// Convert LSFs to LSPs
pub fn lsf_lsp2(lsf: &[i16], lsp: &mut [i16]) {
    let mut freq: i16 = 0;
    let mut ind: i16 = 0;
    let mut offset = 0;
    let mut l_tmp = 0;
    for i in 0..M {
        freq = mul_i16(lsf[i], 20861);
        ind = freq.shr(8);
        offset = freq & 0x00ff;

        if ((ind as i32) - 63).saturating_cast::<i16>() > 0 {
            ind = 63;
        }

        let ind = ind as usize;
        l_tmp = mul_l_i16(SLOPE_COS[ind], offset);
        // lsp[i] = add_i16(TABLE2[ind], extract_l_i32(L_shr(l_tmp, 13)));
    }
}
