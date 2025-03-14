use crate::M;

#[derive(Clone, Copy, Debug)]
pub struct PostFilter {
    pub res2: [f64; 192],
    pub mem_stp: [f64; M],
    pub apond2: [f64; 20],
    pub gain_prec: f64,
}

impl Default for PostFilter {
    fn default() -> Self {
        Self {
            res2: [0.0; 192],
            mem_stp: [0.0; M],
            apond2: [0.0; 20],
            gain_prec: 0.0,
        }
    }
}

impl PostFilter {
    pub fn post(&mut self) {}
}
