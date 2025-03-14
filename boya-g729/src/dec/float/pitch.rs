use crate::dec::param::Parameter;
use crate::{PIT_MAX, PIT_MIN};

pub fn dec_lag3(param: &Parameter, first: bool, t0: &mut i32, t0frac: &mut i32) {
    if first {
        if param.p1 < 197 {
            *t0 = (param.p1 as i32 + 2) / 3 + 19;
            *t0frac = param.p1 as i32 - *t0 * 3 + 58;
        } else {
            *t0 = param.p1 as i32 - 112;
            *t0frac = 0;
        }
    } else {
        let mut t0min = *t0 - 5;
        if t0min < PIT_MIN {
            t0min = PIT_MIN;
        }

        let mut t0max = t0min + 9;
        if t0max > PIT_MAX {
            t0max = PIT_MAX;
            t0min = t0max - 9;
        }

        let i = (param.p1 + 2) / 3 - 1;
        *t0 = i as i32 + t0min;
        *t0frac = param.p1 as i32 - 2 - i as i32 * 3;
    }
}
