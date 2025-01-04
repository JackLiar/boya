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

#[derive(Clone, Debug, Default)]
pub struct Context {
    pub buf: Vec<u8>,
    pub end_offs: usize,
    pub end_window: u32,
    pub n_end_bits: i32,
    pub n_bits_total: u32,
    pub offs: usize,
    pub rng: u32,
    pub val: u32,
    pub ext: u32,
    pub rem: u8,
    pub err: i32,
}

impl Context {
    pub fn new() -> Self {
        let mut ctx = Self::default();
        ctx.n_bits_total =
            EC_CODE_BITS + 1 - ((EC_CODE_BITS - EC_CODE_EXTRA) / EC_SYM_BITS) * EC_SYM_BITS;
        ctx.rng = 1 << EC_CODE_EXTRA;
        ctx
    }

    pub fn read_byte(&mut self) -> u8 {
        if self.offs < self.buf.len() {
            let ret = self.buf[self.offs];
            self.offs += 1;
            ret
        } else {
            0
        }
    }

    pub fn normalize(&mut self) {
        while self.rng <= EC_CODE_BOT {
            self.n_bits_total += EC_SYM_BITS;
            self.rng <<= EC_SYM_BITS;
            let mut sym = self.rem as u32;
            self.rem = self.read_byte();
            sym = (sym << EC_SYM_BITS | self.rem as u32) >> (EC_SYM_BITS - EC_CODE_EXTRA);
            self.val = ((self.val << EC_SYM_BITS) + (EC_SYM_MAX & !sym)) & (EC_CODE_TOP - 1);
        }
    }

    pub fn decode_bit_logp(&mut self, logp: u8) -> bool {
        let mut r = self.rng;
        let mut d = self.val;
        let mut s = r >> logp;
        let ret = d < s;
        if !ret {
            self.val = d - s;
            self.rng = r - s;
        } else {
            self.rng = s;
        }
        ret
    }
}

pub struct RangeDecoder<'a> {
    range: u32,
    value: u32,
    stream: &'a [u8],
    stream_pos: usize,
}

impl<'a> RangeDecoder<'a> {
    /// 创建一个新的 Range Decoder
    pub fn new(stream: &'a [u8]) -> Self {
        let mut decoder = RangeDecoder {
            range: 0xFFFFFFFF, // 初始化区间为全范围
            value: 0,
            stream,
            stream_pos: 0,
        };
        // 读取前4个字节初始化 value
        for _ in 0..4 {
            decoder.value = (decoder.value << 8) | decoder.read_byte();
        }
        decoder
    }

    /// 读取压缩流中的下一个字节
    fn read_byte(&mut self) -> u32 {
        if self.stream_pos < self.stream.len() {
            let byte = self.stream[self.stream_pos];
            self.stream_pos += 1;
            byte as u32
        } else {
            0 // 若比特流结束则返回 0
        }
    }

    /// 解码给定符号的区间
    pub fn decode(&mut self, cum_prob_low: u32, cum_prob_high: u32, total: u32) -> bool {
        // 计算当前的区间宽度
        let range_div = self.range / total;

        // 计算新的 upper 和 lower 界限
        let low = cum_prob_low * range_div;
        let high = cum_prob_high * range_div;

        let target = self.value - low;
        if target < high - low {
            self.range = high - low;
            self.normalize();
            true
        } else {
            false
        }
    }

    /// 归一化区间，确保 range 足够大，读取更多比特流来调整 value
    fn normalize(&mut self) {
        while self.range <= 0x00FFFFFF {
            self.range <<= 8;
            self.value = (self.value << 8) | self.read_byte();
        }
    }

    /// 返回当前区间中的值，方便外部逻辑推导解码的符号
    pub fn get_scaled_value(&self, total: u32) -> u32 {
        self.value / (self.range / total)
    }
}
