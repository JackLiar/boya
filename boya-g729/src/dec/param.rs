use bitvec::slice::BitSlice;
use bitvec::view::AsBits;
use bitvec::{field::BitField, order::Msb0};

use crate::basic_ops::{add_i16, shr_i16};

#[derive(Clone, Copy, Debug, Default)]
pub struct ParameterPacked([u8; 11]);

impl ParameterPacked {
    pub fn new(raw: [u8; 11]) -> Self {
        Self(raw)
    }

    pub fn copy_from_slice(raw: &[u8]) -> Self {
        let mut p = Self::default();
        p.0[1..].copy_from_slice(raw);
        p
    }

    pub fn raw(&self) -> &[u8; 11] {
        &self.0
    }

    pub fn bfi(&self) -> bool {
        self.0[0] == 0
    }

    pub fn set_bfi(&mut self, bfi: bool) {
        self.0[0] = bfi as u8
    }

    /// Switched MA predictor of LSP quantizer, 1 bit
    pub fn l0(&self) -> u8 {
        self.0[1] >> 7
    }

    /// First stage vector of quantizer, 7 bits
    pub fn l1(&self) -> u8 {
        self.0[1] & 0b111_1111
    }

    /// Second stage lower vector of LSP quantizer, 5 bits
    pub fn l2(&self) -> u8 {
        self.0[2] >> 3
    }

    /// Second stage higher vector of LSP quantizer, 5 bits
    pub fn l3(&self) -> u8 {
        ((self.0[2] & 0b111) << 2) | (self.0[3] >> 6)
    }

    /// Pitch delay first subframe, 8 bits
    pub fn p1(&self) -> u8 {
        ((self.0[3] & 0b11_1111) << 2) | self.0[4] >> 6
    }

    /// Parity bit for pitch delay, 1 bit
    pub fn p0(&self) -> u8 {
        (self.0[4] >> 5) & 0b1
    }

    pub fn set_p0(&mut self, p0: bool) {
        if p0 {
            self.0[4] |= 1 << 3
        } else {
            self.0[4] &= !(1 << 3)
        }
    }

    /// Fixed codebook first subframe, 13 bits
    pub fn c1(&self) -> u16 {
        (((self.0[4] as u16) & 0b1_1111) << 8) | (self.0[5] as u16)
    }

    /// Signs of fixed-codebook pulses 1st subframe, 4 bits
    pub fn s1(&self) -> u8 {
        self.0[6] >> 4
    }

    /// Gain codebook (stage 1) 1st subframe, 3 bits
    pub fn ga1(&self) -> u8 {
        (self.0[6] >> 1) & 0b111
    }

    /// Gain codebook (stage 2) 1st subframe, 4 bits
    pub fn gb1(&self) -> u8 {
        ((self.0[6] & 0b1) << 3) | (self.0[7] >> 5)
    }

    /// Pitch delay second subframe, 5 bits
    pub fn p2(&self) -> u8 {
        self.0[7] & 0b1_1111
    }

    /// Fixed codebook 2nd subframe, 13 bits
    pub fn c2(&self) -> u16 {
        ((self.0[8] as u16) << 5) | ((self.0[9] as u16) >> 3)
    }

    /// Signs of fixed-codebook pulses 2nd subframe, 4 bits
    pub fn s2(&self) -> u8 {
        ((self.0[9] & 0b111) << 1) | (self.0[10] >> 7)
    }

    /// Gain codebook (stage 1) 2nd subframe
    pub fn ga2(&self) -> u8 {
        (self.0[10] >> 4) & 0b111
    }

    /// Gain codebook (stage 2) 2nd subframe
    pub fn gb2(&self) -> u8 {
        self.0[10] & 0b1111
    }

    /// Check parity of index with transmitted parity
    pub fn check_parity_pitch(&mut self) -> bool {
        let pidx = self.p1();

        let mut temp = shr_i16(pidx as i16, 1);
        let mut sum = 1;
        let mut bit;
        for _ in 0..=5 {
            temp = shr_i16(temp, 1);
            bit = temp & 1;
            sum = add_i16(sum, bit);
        }
        sum = add_i16(sum, self.p0() as i16);
        sum &= 1;
        sum == 1
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Parameter {
    pub bfi: bool,
    /// Switched MA predictor of LSP quantizer, 1 bit
    pub l0: bool,
    /// First stage vector of quantizer, 7 bits
    pub l1: u8,
    /// Second stage lower vector of LSP quantizer, 5 bits
    pub l2: u8,
    /// Second stage higher vector of LSP quantizer, 5 bits
    pub l3: u8,
    /// Pitch delay first subframe, 8 bits
    pub p1: u8,
    /// Parity bit for pitch delay, 1 bit
    pub p0: bool,
    /// Fixed codebook first subframe, 13 bits
    pub c1: u16,
    /// Signs of fixed-codebook pulses 1st subframe, 4 bits
    pub s1: u8,
    /// Gain codebook (stage 1) 1st subframe, 3 bits
    pub ga1: u8,
    /// Gain codebook (stage 2) 1st subframe, 4 bits
    pub gb1: u8,
    /// Pitch delay second subframe, 5 bits
    pub p2: u8,
    /// Fixed codebook 2nd subframe, 13 bits
    pub c2: u16,
    /// Signs of fixed-codebook pulses 2nd subframe, 4 bits
    pub s2: u8,
    /// Gain codebook (stage 1) 2nd subframe
    pub ga2: u8,
    /// Gain codebook (stage 2) 2nd subframe
    pub gb2: u8,
}

impl From<&[u8; 10]> for Parameter {
    fn from(value: &[u8; 10]) -> Self {
        Self::new(value.as_bits())
    }
}

impl Parameter {
    pub fn new(data: &BitSlice<u8, Msb0>) -> Self {
        let mut param = Self {
            bfi: data.not_any(),
            ..Default::default()
        };

        let (p, rem) = data.split_at(1);
        param.l0 = p.load::<u8>() == 1;

        let (p, rem) = rem.split_at(7);
        param.l1 = p.load::<u8>();

        let (p, rem) = rem.split_at(5);
        param.l2 = p.load::<u8>();

        let (p, rem) = rem.split_at(5);
        param.l3 = p.load_be::<u8>();

        let (p, rem) = rem.split_at(8);
        param.p1 = p.load::<u8>();

        let (p, rem) = rem.split_at(1);
        param.p0 = p.load::<u8>() == 0;

        let (p, rem) = rem.split_at(13);
        param.c1 = p.load::<u16>();

        let (p, rem) = rem.split_at(4);
        param.s1 = p.load::<u8>();

        let (p, rem) = rem.split_at(3);
        param.ga1 = p.load::<u8>();

        let (p, rem) = rem.split_at(4);
        param.gb1 = p.load::<u8>();

        let (p, rem) = rem.split_at(5);
        param.p2 = p.load::<u8>();

        let (p, rem) = rem.split_at(13);
        param.c2 = p.load::<u16>();

        let (p, rem) = rem.split_at(4);
        param.s2 = p.load::<u8>();

        let (p, rem) = rem.split_at(3);
        param.ga2 = p.load::<u8>();

        let (p, _) = rem.split_at(4);
        param.gb2 = p.load::<u8>();

        param.p0 = param.check_parity_pitch();

        param
    }

    /// Check parity of index with transmitted parity
    pub fn check_parity_pitch(&mut self) -> bool {
        let pidx = self.p1;

        let mut temp = shr_i16(pidx as i16, 1);
        let mut sum = 1;
        let mut bit;
        for _ in 0..=5 {
            temp = shr_i16(temp, 1);
            bit = temp & 1;
            sum = add_i16(sum, bit);
        }
        sum = add_i16(sum, self.p0 as i16);
        sum &= 1;
        sum == 1
    }
}
