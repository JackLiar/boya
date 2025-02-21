use super::param::Parameter;
use super::G729Decoder;
use crate::basic_ops::{
    add_i16, extract_l_i32, mac_l_i16, msu_l_i16, mul_i16, mul_l_i16, shl_l_i16, shr_i16,
    shr_l_i16, sub_i16, sub_i32,
};
use crate::{FG, FG_SUM, FG_SUM_INV, GAP1, GAP2, GAP3, LSPCB1, LSPCB2, M, NC, SLOPE_COS, TABLE2};

fn lsp_expand_1_2(buf: &mut [i16; M], gap: i16) {
    for j in 1..(M - 1) {
        let diff = sub_i16(buf[j - 1], buf[j]);
        let tmp = shr_i16(add_i16(diff, gap), 1);
        if tmp > 0 {
            buf[j - 1] = sub_i16(buf[j - 1], tmp);
            buf[j] = add_i16(buf[j], tmp);
        }
    }
}

/// Compose LSP parameter from elementary LSP with previous LSP.
fn lsp_prev_compose(p: &Parameter, buf: &[i16; M], freq_prev: &[[i16; M]; 4], lspq: &mut [i16]) {
    let mode_idx = p.l0 as usize;
    let fg = &FG[mode_idx];
    let fg_sum = &FG_SUM[mode_idx];

    for (j, (b, lsp)) in buf.iter().copied().zip(lspq).enumerate() {
        let mut l_acc = mul_l_i16(b, fg_sum[j]);
        for (freqs, fg) in freq_prev.iter().zip(fg) {
            l_acc = mac_l_i16(freqs[j], fg[j], l_acc);
        }
        *lsp = (l_acc >> 16) as i16;
    }
}

fn lsf_lsp2(lsf: &[i16; M], lspq: &mut [i16; M]) {
    for (lsf, lspq) in lsf.iter().copied().zip(lspq) {
        let freq = mul_i16(lsf, 20861);
        let mut ind = shr_i16(freq, 8);
        let offset = freq & 0x00ff;

        if sub_i16(ind, 63) > 0 {
            ind = 63;
        }

        let tmp = mul_l_i16(SLOPE_COS[ind as usize], offset);
        *lspq = add_i16(TABLE2[ind as usize], extract_l_i32(shr_l_i16(tmp, 13)));
    }
}

impl G729Decoder {
    pub fn lsp_decw_reset(&mut self) {
        for freq in &mut self.freq_prev {
            *freq = self.freq_prev_reset;
        }
        self.prev_ma = 0;
        self.prev_lsp = self.freq_prev_reset;
    }

    pub fn lsp_decode(&mut self, lsp_q: &mut [i16; M]) {
        let mut lsf_q = [0i16; M];
        self.lsp_iqua_cs(&mut lsf_q);
        lsf_lsp2(&lsf_q, lsp_q);
    }

    pub fn lsp_iqua_cs(&mut self, lspq: &mut [i16; M]) {
        if self.param.bfi {
            let mut buf = [0i16; M];
            *lspq = self.prev_lsp;
            self.lsp_prev_extract(&mut buf);
            self.lsp_prev_update(&buf);
        } else {
            self.lsp_get_quant(lspq);
            self.prev_lsp = *lspq;
            self.prev_ma = self.param.l0 as i16;
        }
    }

    fn lsp_get_quant(&mut self, lspq: &mut [i16]) {
        let p = &self.param;
        let l1 = p.l1 as usize;
        let l2 = p.l2 as usize;
        let l3 = p.l3 as usize;
        let mut buf = [0i16; M];

        for (j, b) in buf.iter_mut().enumerate().take(NC) {
            *b = add_i16(LSPCB1[l1][j], LSPCB2[l2][j]);
        }

        for (j, b) in buf.iter_mut().enumerate().take(M).skip(NC) {
            *b = add_i16(LSPCB1[l1][j], LSPCB2[l3][j]);
        }

        lsp_expand_1_2(&mut buf, GAP1);
        lsp_expand_1_2(&mut buf, GAP2);

        lsp_prev_compose(p, &buf, &self.freq_prev, lspq);

        self.lsp_stability(&mut buf);
    }

    pub fn lsp_prev_update(&mut self, buf: &[i16; 10]) {
        self.freq_prev[3] = self.freq_prev[2];
        self.freq_prev[2] = self.freq_prev[1];
        self.freq_prev[1] = self.freq_prev[0];
        self.freq_prev[0] = *buf;
    }

    pub fn lsp_stability(&mut self, buf: &mut [i16; 10]) {
        for j in 0..(buf.len() - 1) {
            let l_acc = buf[j + 1] as i32;
            let l_accb = buf[j] as i32;
            let l_diff = sub_i32(l_acc, l_accb);
            if l_diff < 0 {
                buf.swap(j + 1, j);
            }
        }

        if buf[0] < 40 {
            buf[0] = 40;
        }

        for j in 0..(buf.len() - 1) {
            let l_acc = buf[j + 1] as i32;
            let l_accb = buf[j] as i32;
            let l_diff = sub_i32(l_acc, l_accb);
            if sub_i32(l_diff, GAP3 as i32) < 0 {
                buf[j + 1] = add_i16(buf[j], GAP3);
            }
        }

        if buf[9] > 25681 {
            buf[9] = 25681;
        }
    }

    pub fn lsp_prev_extract(&mut self, buf: &mut [i16; 10]) {
        for (j, (b, prev)) in buf.iter_mut().zip(self.prev_lsp).enumerate() {
            let mut temp = (prev as i32) << 16;
            for (freq_prev, fg) in self.freq_prev.iter().zip(FG[self.prev_ma as usize]) {
                temp = msu_l_i16(freq_prev[j], fg[j], temp);
            }

            let temp = (temp >> 16) as i16;
            let temp = mul_l_i16(temp, FG_SUM_INV[self.prev_ma as usize][j]);
            *b = (shl_l_i16(temp, 3) >> 16) as i16;
        }
    }
}
