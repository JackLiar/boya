use crate::Result;

/// The number of bits to output at a time.
pub const EC_SYM_BITS: u32 = 8;
/// The total number of bits in each of the state registers.
pub const EC_CODE_BITS: u32 = 32;
/// The maximum symbol value.
pub const EC_SYM_MAX: u32 = (1 << EC_SYM_BITS) - 1;
/// Bits to shift by to move a symbol into the high-order position.
pub const EC_CODE_SHIFT: u32 = EC_CODE_BITS - EC_SYM_BITS - 1;
/// Carry bit of the high-order range symbol.
pub const EC_CODE_TOP: u32 = 1 << (EC_CODE_BITS - 1);
/// Low-order bit of the high-order range symbol.
pub const EC_CODE_BOT: u32 = EC_CODE_TOP >> EC_SYM_BITS;
/// The number of bits available for the last, partial symbol in the code field.
pub const EC_CODE_EXTRA: u32 = (EC_CODE_BITS - 2) % EC_SYM_BITS + 1;

pub struct RangeDecoder<'a> {
    /// Size of the current range
    range: u32,
    /// Difference between the high end of the current range and the actual coded value, minus one
    val: u32,
    stream: &'a [u8],
    bits_read: usize,
}

impl<'a> RangeDecoder<'a> {
    pub fn try_new(stream: &'a [u8]) -> Result<Self> {
        let mut dec = match stream.first() {
            None => todo!(),
            Some(b0) => RangeDecoder {
                range: 128,
                val: (127 - (b0 >> 1)) as u32,
                stream,
                bits_read: 33,
            },
        };
        dec.normalize();
        Ok(dec)
    }

    pub fn tell(&self) -> usize {
        println!(
            "{} {} {} {}",
            self.bits_read,
            32,
            self.range,
            self.range.leading_zeros()
        );
        self.bits_read - (32 - self.range.leading_zeros() as usize)
    }

    pub fn tell_frac(self) -> usize {
        unimplemented!()
    }

    pub fn tot_bits(&self) -> usize {
        self.stream.len() * 8
    }

    pub fn decode(&self, ft: u32) -> u16 {
        (ft - ((self.val / (self.range / ft)) + 1).min(ft)) as u16
    }

    pub fn decode_bin(&self, bits: usize) -> u16 {
        ((1 << bits) - (self.val / (self.range >> bits) + 1).min(1 << bits)) as u16
    }

    pub fn decode_bit_logp(&mut self, logp: u32) -> u8 {
        let s = self.range >> logp;
        let ret = self.val < s;
        if !ret {
            self.val -= s;
            self.range -= s;
        } else {
            self.range = s;
        }
        self.normalize();
        ret as u8
    }

    pub fn decode_icdf(&self) {}

    pub fn update(&mut self, fl: u32, fh: u32, ft: u32) {
        let s = (self.range / ft) * (ft - fh);
        self.val -= s;
        if fl > 0 {
            self.range = (self.range / ft) * (fh - fl)
        } else {
            self.range -= s
        }
        self.normalize();
    }

    pub fn normalize(&mut self) {
        while self.range <= 2u32.pow(23) {
            self.bits_read += 8;
            self.range <<= 8;
            let sym = match self.read_byte() {
                None => 0,
                Some(prev) => match self.stream.first() {
                    Some(next) => prev << 7 | (next | 0b0111_1111),
                    None => 0,
                },
            };

            self.val = ((self.val << 8) + (255 - sym) as u32) & 0x7FFFFFFF;
        }
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        match self.stream.split_first() {
            None => None,
            Some((b, rem)) => {
                self.stream = rem;
                Some(*b)
            }
        }
    }
}
