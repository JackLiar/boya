use saturating_cast::SaturatingCast;

pub fn mul_i16(x: i16, y: i16) -> i16 {
    (x as i32).wrapping_mul(y as i32).saturating_cast()
}

pub fn mul_l_i16(x: i16, y: i16) -> i32 {
    // println!("{:b}", (-1i32));
    // println!("{}", (0b10000000000000000000000000000000u32 as i32));
    // println!("{}", -32768i32.wrapping_mul(-32768));
    (x as i32).wrapping_mul(y as i32).saturating_mul(2)
}

pub fn mac_l_i16(x: i16, y: i16, z: i32) -> i32 {
    let produit = mul_l_i16(x, y);
    add_i32(produit, z)
}

pub fn msu_l_i16(x: i16, y: i16, z: i32) -> i32 {
    let produit = mul_l_i16(x, y);
    sub_i32(z, produit)
}

pub fn add_i16(x: i16, y: i16) -> i16 {
    (x as i32).wrapping_add(y as i32).saturating_cast()
}

pub fn add_i32(x: i32, y: i32) -> i32 {
    (x as i64).wrapping_add(y as i64).saturating_cast()
}

pub fn sub_i16(x: i16, y: i16) -> i16 {
    (x as i32).wrapping_sub(y as i32).saturating_cast()
}

pub fn sub_i32(x: i32, y: i32) -> i32 {
    (x as i64).wrapping_sub(y as i64).saturating_cast()
}

/// Return the 16 LSB of L_var1
pub fn extract_l_i32(x: i32) -> i16 {
    (x & 0xFFFF) as i16
}

/// Return the 16 MSB of L_var1
pub fn extract_h_i32(x: i32) -> i16 {
    (x >> 16) as i16
}

pub fn shl_i16(var1: i16, var2: i16) -> i16 {
    let mut var_out: i16 = 0;
    let mut resultat: i32 = 0;

    if var2 < 0 {
        var_out = shr_i16(var1, -var2);
    } else {
        resultat = (var1 as i32) * (1 << var2);
        if (var2 > 15 && var1 != 0) || (resultat != (resultat as i16) as i32) {
            if var1 > 0 {
                var_out = i16::MAX
            } else {
                var_out = i16::MIN
            }
        } else {
            var_out = extract_l_i32(resultat);
        }
    }

    var_out
}

pub fn shl_l_i16(mut var1: i32, mut var2: i16) -> i32 {
    let mut out = 0;
    if var2 <= 0 {
        out = shr_l_i16(var1, -var2);
    } else {
        while var2 > 0 {
            var2 -= 1;
            if var1 > 0x3fffffff {
                out = i32::MAX;
                break;
            } else if var1 < -1073741824i32 {
                out = i32::MIN;
                break;
            }
            var1 *= 2;
            out = var1;
        }
    }
    out
}

pub fn shr_i16(var1: i16, var2: i16) -> i16 {
    if var2 < 0 {
        shl_i16(var1, -var2)
    } else if var2 >= 15 {
        if var1 < 0 {
            -1
        } else {
            0
        }
    } else if var1 < 0 {
        !((!var1) >> var2)
    } else {
        var1 >> var2
    }
}

pub fn shr_l_i16(var1: i32, var2: i16) -> i32 {
    let mut out = 0;
    if var2 < 0 {
        out = shl_l_i16(var1, var2);
    } else if var2 >= 31 {
        out = if var1 < 0 { -1 } else { 0 }
    } else if var1 < 0 {
        out = !((!var1) >> var2);
    } else {
        out = var1 >> var2;
    }
    out
}

#[cfg(test)]
mod test {
    use std::ops::{Mul, Shr};

    use super::*;

    #[test]
    fn test_add_i16() {
        assert_eq!(add_i16(1, 1), 2);
    }

    #[test]
    fn test_mul_i16() {
        assert_eq!(mul_i16(-32768, -32768), 32767);
    }

    #[test]
    fn test_l_mul_i16() {
        assert_eq!(mul_l_i16(-32768, -32768), 2147483647);
    }

    #[test]
    fn test_sature() {
        assert_eq!(1i32.saturating_cast::<i16>(), 1);
        assert_eq!((0x00008000i32).saturating_cast::<i16>(), i16::MAX);
        assert_eq!((0xffff7fffu32 as i32).saturating_cast::<i16>(), i16::MIN);
    }

    #[test]
    fn test_shr_i16() {
        assert_eq!(shr_i16(1, 1), 0);
        assert_eq!(shr_i16(1, 14), 1 >> 14);
        assert_eq!(shr_i16(1, 15), 0);
        assert_eq!(shr_i16(-2, 15), -1);

        assert_eq!(1i16.shr(1), 0i16);
        assert_eq!(1i16.shr(14), 0i16);
        assert_eq!(1i16.shr(15), 0i16);
        assert_eq!((-2i16).shr(15), -1i16);
        println!("{:b}", 16384i16);
        assert_eq!((16384i16).shr(15), 0i16);
    }

    #[test]
    fn test_shl_i16() {
        assert_eq!(shl_i16(1, 1), 2);
    }
}
