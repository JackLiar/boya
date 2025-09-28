use crate::Decoder;
use crate::consts::FAC;
use crate::utils::sasr;

impl Decoder {
    pub fn apcm_quantization_xmaxc_to_exp_mant(xmaxc: i16) -> (i16, i16) {
        let mut exp = 0;
        let mut mant = 0;
        if xmaxc > 15 {
            exp = sasr(xmaxc, 3).wrapping_sub(1);
        }
        mant = xmaxc - (exp << 3);

        if mant == 0 {
            exp = -4;
            mant = 7;
        } else {
            while mant <= 7 {
                mant = mant << 1 | 1;
                exp -= 1;
            }
            mant -= 8;
        }

        debug_assert!((-4..=6).contains(&exp));
        debug_assert!((0..=7).contains(&mant));
        (exp, mant)
    }

    pub fn apcm_inverse_quantization(xmc: &[i16; 13], mant: i16, exp: i16, xmp: &mut [i16; 13]) {
        debug_assert!((0..=7).contains(&mant));

        let tmp1 = FAC[mant as usize];
        let tmp2 = 0;
        let tmp3 = 0;

        for (xmc, xmp) in xmc.iter().zip(xmp) {
            debug_assert!((0..=7).contains(xmc));
            let mut tmp = (*xmc << 1).wrapping_sub(7);
            debug_assert!((-7..=7).contains(&tmp));
            tmp <<= 12;
            tmp = tmp1 * tmp;
            tmp = tmp + tmp3;
            *xmp = todo!();
        }
    }

    pub fn rpe_grid_positioning(mc: i16, xmp: &[i16; 13], ep: &mut [i16; 40]) {
        let mut i = 13;
        debug_assert!((0..=3).contains(&mc));
        // while i > 0 {
        //     match mc {
        //         0 => {}
        //         1 => {}
        //         2 => {}
        //         3 => {}
        //         _ => {}
        //     }
        // }
    }

    pub fn rpe_decode(&mut self, xmaxcr: i16, mcr: i16, xmcr: &[i16; 13], erp: &mut [i16; 40]) {
        let mut exp = 0i16;
        let mut mant = 0i16;
        let mut xmp = [0i16; 13];
        let (exp, mant) = Self::apcm_quantization_xmaxc_to_exp_mant(xmaxcr);
        Self::apcm_inverse_quantization(xmcr, mant, exp, &mut xmp);
    }
}
