#[derive(Clone, Debug)]
pub struct PostFilter {
    /// A(gamma2) residual
    pub res2: [i16; 192],
    pub res2_idx: usize,
    /// A(gamma2) memory
    pub mem_stp: [i16; 10],
    pub mem_stp_idx: usize,
    pub apond: [i16; 20],
    pub mem_zero: [i16; 10],
    pub gain_prec: i16,
}

impl Default for PostFilter {
    fn default() -> Self {
        Self {
            res2: [0; 192],
            res2_idx: 152,
            mem_stp: [0; 10],
            mem_stp_idx: 9,
            apond: [0; 20],
            mem_zero: [0; 10],
            gain_prec: 16384,
        }
    }
}
