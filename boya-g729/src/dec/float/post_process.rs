use super::consts::{A100, B100};

#[derive(Clone, Copy, Debug, Default)]
pub struct PostProcess {
    pub x0: f64,
    pub x1: f64,
    pub y1: f64,
    pub y2: f64,
}

impl PostProcess {
    pub fn post_process(&mut self, signal: &mut [f64; 80]) {
        for s in signal.iter_mut() {
            let x2 = self.x1;
            self.x1 = self.x0;
            self.x0 = *s;
            let y0 = self.y1 * A100[1]
                + self.y2 * A100[1]
                + self.x0 * B100[0]
                + self.x1 * B100[1]
                + x2 * B100[2];
            *s = y0;
            self.y2 = self.y1;
            self.y1 = y0;
        }
    }
}
