use super::consts::{
    FG, FG_SUM, FG_SUM_INV, GAP1, GAP2, GAP3, INIT_FREQ_PREV, LSPCB1, LSPCB2, L_LIMIT, M_LIMIT,
};
use crate::dec::param::Parameter;
use crate::M;

#[derive(Clone, Debug, Default)]
pub struct LinearPrediction {
    pub freq_prev: [[f64; M]; 4],
    /// Previous LSP vector
    pub prev_lsp: [f64; M],
    pub prev_ma: bool,
}

impl LinearPrediction {
    pub fn new() -> Self {
        Self {
            freq_prev: [INIT_FREQ_PREV; 4],
            prev_ma: false,
            prev_lsp: INIT_FREQ_PREV,
        }
    }

    fn lsp_expand_1_2(buf: &mut [f64; M], gap: f64) {
        for j in 1..(M - 1) {
            let diff = buf[j - 1] - buf[j];
            let tmp = (diff + gap) * 0.5;
            if tmp > 0.0 {
                buf[j - 1] -= tmp;
                buf[j] += tmp;
            }
        }
    }

    /// Compose LSP parameter from elementary LSP with previous LSP.
    fn lsp_prev_compose(&mut self, p: &Parameter, buf: &[f64; M], lspq: &mut [f64]) {
        let mode_idx = p.l0 as usize;
        let fg = &FG[mode_idx];
        let fg_sum = &FG_SUM[mode_idx];

        for (j, (b, lsp)) in buf.iter().zip(lspq).enumerate() {
            *lsp = *b * fg_sum[j];
            for (freqs, fg) in self.freq_prev.iter().zip(fg) {
                *lsp += freqs[j] * fg[j];
            }
        }
    }

    fn lsp_prev_update(&mut self, buf: &[f64; 10]) {
        self.freq_prev[3] = self.freq_prev[2];
        self.freq_prev[2] = self.freq_prev[1];
        self.freq_prev[1] = self.freq_prev[0];
        self.freq_prev[0] = *buf;
    }

    fn lsp_stability(buf: &mut [f64; 10]) {
        for j in 0..(buf.len() - 1) {
            let diff = buf[j + 1] - buf[j];
            if diff < 0.0 {
                buf.swap(j + 1, j);
            }
        }

        if buf[0] < L_LIMIT {
            buf[0] = L_LIMIT;
        }

        for j in 0..(buf.len() - 1) {
            let diff = buf[j + 1] - buf[j];
            if (diff - GAP3) < 0.0 {
                buf[j + 1] = buf[j] + GAP3;
            }
        }

        if buf[9] > M_LIMIT {
            buf[9] = M_LIMIT;
        }
    }

    fn lsp_get_quant(&mut self, p: &Parameter, lsp_q: &mut [f64; M]) {
        let l1 = p.l1 as usize;
        let l2 = p.l2 as usize;
        let l3 = p.l3 as usize;
        // LSPCB1
        let mut buf = [0.0f64; M];
        for (j, b) in buf.iter_mut().enumerate().take(5) {
            *b = LSPCB1[l1][j] + LSPCB2[l2][j];
        }
        for (j, b) in buf.iter_mut().enumerate().take(M).skip(5) {
            *b = LSPCB1[l1][j] + LSPCB2[l3][j];
        }

        Self::lsp_expand_1_2(&mut buf, GAP1);
        Self::lsp_expand_1_2(&mut buf, GAP2);

        self.lsp_prev_compose(p, &buf, lsp_q);

        self.lsp_prev_update(&buf);

        Self::lsp_stability(&mut buf);
    }

    pub fn lsp_prev_extract(&mut self, buf: &mut [f64; M]) {
        for (j, (b, prev)) in buf.iter_mut().zip(self.prev_lsp).enumerate() {
            *b = prev;
            for (freq_prev, fg) in self.freq_prev.iter().zip(FG[self.prev_ma as usize]) {
                *b -= freq_prev[j] * fg[j];
            }
            *b *= FG_SUM_INV[self.prev_ma as usize][j]
        }
    }

    /// LSP main quantization routine
    fn lsp_iqua_cs(&mut self, p: &Parameter, lsp_q: &mut [f64; M]) {
        if p.bfi {
            let mut buf = [0.0f64; M];
            *lsp_q = self.prev_lsp;
            self.lsp_prev_extract(&mut buf);
            self.lsp_prev_update(&buf);
        } else {
            self.lsp_get_quant(p, lsp_q);
            self.prev_lsp = *lsp_q;
            self.prev_ma = p.l0;
        }
    }

    pub fn decode_lsp(&mut self, p: &Parameter, lsp_q: &mut [f64; M]) {
        self.lsp_iqua_cs(p, lsp_q);
        for lsp in lsp_q.iter_mut() {
            *lsp = lsp.cos();
        }
    }
}
