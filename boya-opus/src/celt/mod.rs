use std::ops::Range;

use crate::{Channels, Complexity, Result, SampleRate, MAX_PERIOD};

pub const PLC_PITCH_LAG_MAX: isize = 720;
pub const PLC_PITCH_LAG_MIN: isize = 100;

pub const COMBFILTER_MAXPERIOD: i32 = 1024;
pub const COMBFILTER_MINPERIOD: i32 = 15;

#[rustfmt::skip]
pub const EBAND_5MS: [i16; 22] = [
//  0  200 400 600 800  1k 1.2 1.4 1.6  2k 2.4 2.8 3.2  4k 4.8 5.6 6.8  8k 9.6 12k 15.6 */
    0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 14, 16, 20, 24, 28, 34, 40, 48, 60, 78, 100,
];

/// 0  200 400 600 800  1k 1.2 1.4 1.6  2k 2.4 2.8 3.2  4k 4.8 5.6 6.8  8k 9.6 12k 15.6
#[rustfmt::skip]
pub const BAND_ALLOCATION: [u8; 231] = [
      0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
     90, 80, 75, 69, 63, 56, 49, 40, 34, 29, 20, 18, 10,  0,  0,  0,  0,  0,  0,  0,  0,
    110,100, 90, 84, 78, 71, 65, 58, 51, 45, 39, 32, 26, 20, 12,  0,  0,  0,  0,  0,  0,
    118,110,103, 93, 86, 80, 75, 70, 65, 59, 53, 47, 40, 31, 23, 15,  4,  0,  0,  0,  0,
    126,119,112,104, 95, 89, 83, 78, 72, 66, 60, 54, 47, 39, 32, 25, 17, 12,  1,  0,  0,
    134,127,120,114,103, 97, 91, 85, 78, 72, 66, 60, 54, 47, 41, 35, 29, 23, 16, 10,  1,
    144,137,130,124,113,107,101, 95, 88, 82, 76, 70, 64, 57, 51, 45, 39, 33, 26, 15,  1,
    152,145,138,132,123,117,111,105, 98, 92, 86, 80, 74, 67, 61, 55, 49, 43, 36, 20,  1,
    162,155,148,142,133,127,121,115,108,102, 96, 90, 84, 77, 71, 65, 59, 53, 46, 30,  1,
    172,165,158,152,143,137,131,125,118,112,106,100, 94, 87, 81, 75, 69, 63, 56, 45, 20,
    200,200,200,200,200,200,200,200,198,193,188,183,178,173,168,163,158,153,148,129,104,
];

pub const LOG_N_400: [i16; 21] = [
    0, 0, 0, 0, 0, 0, 0, 0, 8, 8, 8, 8, 16, 16, 16, 21, 21, 24, 29, 34, 36,
];

#[rustfmt::skip]
pub const WINDOW120: [f32; 120] = [
    6.7286966e-05, 0.00060551348, 0.0016815970, 0.0032947962, 0.0054439943,
    0.0081276923, 0.011344001, 0.015090633, 0.019364886, 0.024163635,
    0.029483315, 0.035319905, 0.041668911, 0.048525347, 0.055883718,
    0.063737999, 0.072081616, 0.080907428, 0.090207705, 0.099974111,
    0.11019769, 0.12086883, 0.13197729, 0.14351214, 0.15546177,
    0.16781389, 0.18055550, 0.19367290, 0.20715171, 0.22097682,
    0.23513243, 0.24960208, 0.26436860, 0.27941419, 0.29472040,
    0.31026818, 0.32603788, 0.34200931, 0.35816177, 0.37447407,
    0.39092462, 0.40749142, 0.42415215, 0.44088423, 0.45766484,
    0.47447104, 0.49127978, 0.50806798, 0.52481261, 0.54149077,
    0.55807973, 0.57455701, 0.59090049, 0.60708841, 0.62309951,
    0.63891306, 0.65450896, 0.66986776, 0.68497077, 0.69980010,
    0.71433873, 0.72857055, 0.74248043, 0.75605424, 0.76927895,
    0.78214257, 0.79463430, 0.80674445, 0.81846456, 0.82978733,
    0.84070669, 0.85121779, 0.86131698, 0.87100183, 0.88027111,
    0.88912479, 0.89756398, 0.90559094, 0.91320904, 0.92042270,
    0.92723738, 0.93365955, 0.93969656, 0.94535671, 0.95064907,
    0.95558353, 0.96017067, 0.96442171, 0.96834849, 0.97196334,
    0.97527906, 0.97830883, 0.98106616, 0.98356480, 0.98581869,
    0.98784191, 0.98964856, 0.99125274, 0.99266849, 0.99390969,
    0.99499004, 0.99592297, 0.99672162, 0.99739874, 0.99796667,
    0.99843728, 0.99882195, 0.99913147, 0.99937606, 0.99956527,
    0.99970802, 0.99981248, 0.99988613, 0.99993565, 0.99996697,
    0.99998518, 0.99999457, 0.99999859, 0.99999982, 1.0000000,
];

#[derive(Clone, Debug, Default)]
pub struct Mode {
    pub sample_rate: SampleRate,
    pub overlap: i32,
    pub nbEBands: i32,
    pub effEBands: i32,
    pub preemph: [f32; 4],
    pub eBands: Vec<i16>,
    pub maxLM: i32,
    pub nbShortMdcts: i32,
    pub shortMdctSize: i32,
    /// Number of lines in the matrix below
    pub nbAllocVectors: i32,
    // Number of bits in each band for several rates
    pub allocVectors: Vec<u8>,
    pub logN: Vec<i16>,
    pub window: Vec<f32>,
    // mdct_lookup mdct;
    // PulseCache cache;
}

pub fn mode_48000_960_120() -> Mode {
    Mode {
        sample_rate: SampleRate::Fs48000,
        overlap: 120,
        nbEBands: 21,
        effEBands: 21,
        preemph: [0.85000610, 0.0000000, 1.0000000, 1.0000000],
        eBands: EBAND_5MS.to_vec(),
        maxLM: 3,
        nbShortMdcts: 8,
        shortMdctSize: 120,
        nbAllocVectors: 11,
        allocVectors: BAND_ALLOCATION.to_vec(),
        logN: LOG_N_400.to_vec(),
        window: WINDOW120.to_vec(),
    }
}

#[derive(Debug, Default)]
pub struct CeltDecoder {
    pub mode: Mode,
    pub overlap: i32,
    pub channels: Channels,
    pub stream_channels: Channels,

    pub down_sample: i32,
    pub start_end: Range<i32>,
    pub signalling: bool,
    pub disable_inv: bool,
    pub complexity: Complexity,
    pub arch: i32,

    pub rng: u32,
    pub error: i32,
    pub last_pitch_index: isize,
    pub loss_duration: i32,
    pub skip_plc: bool,
    pub postfilter_period: i32,
    pub postfilter_period_old: i32,
    pub postfilter_gain: i32,
    pub postfilter_gain_old: i32,
    pub postfilter_tapset: i32,
    pub postfilter_tapset_old: i32,
    pub prefilter_and_fold: i32,

    pub preemph_memD: [f32; 2],
    #[cfg(feature = "deep-plc")]
    pub plc_pcm: [i16; 100],
    #[cfg(feature = "deep-plc")]
    pub plc_fill: i32,
    #[cfg(feature = "deep-plc")]
    pub plc_preemphasis_mem: f32,

    pub _decode_mem: f32,
}

macro_rules! celt_assert {
    ($e:expr) => {
        if !($e) {
            return false;
        }
    };
}

impl CeltDecoder {
    pub fn new(sr: SampleRate, chls: Channels) -> Self {
        Self::default()
    }

    pub fn validate(&self) -> bool {
        if cfg!(not(feature = "custom-mode")) {
            celt_assert!(self.overlap == 120);
            celt_assert!(self.start_end.end <= 21);
        } else {
            // TODO assert
        }

        celt_assert!(self.down_sample > 0);
        celt_assert!(self.start_end.start == 0 || self.start_end.start == 17);
        celt_assert!(self.start_end.start < self.start_end.end);

        celt_assert!(self.last_pitch_index <= PLC_PITCH_LAG_MAX);
        celt_assert!(self.last_pitch_index >= PLC_PITCH_LAG_MIN || self.last_pitch_index == 0);
        celt_assert!(self.postfilter_period < MAX_PERIOD);
        celt_assert!(self.postfilter_period < MAX_PERIOD);
        celt_assert!(self.postfilter_period >= COMBFILTER_MINPERIOD || self.postfilter_period == 0);
        celt_assert!(self.postfilter_period_old < MAX_PERIOD);
        celt_assert!(
            self.postfilter_period_old >= COMBFILTER_MINPERIOD || self.postfilter_period_old == 0
        );
        celt_assert!(self.postfilter_tapset <= 2);
        celt_assert!(self.postfilter_tapset >= 0);
        celt_assert!(self.postfilter_tapset_old <= 2);
        celt_assert!(self.postfilter_tapset_old >= 0);

        true
    }

    pub fn decode_with_ec_dred(&mut self) -> Result<()> {
        Ok(())
    }
}
