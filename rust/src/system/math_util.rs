#[unsafe(no_mangle)]
pub fn MathUtil_Mul16(x: i16, y: i16) -> i16 {
    ((i32::from(x) * i32::from(y)) / 256) as i16
}

#[unsafe(no_mangle)]
pub fn MathUtil_Mul16Shift(shift: u8, x: i16, y: i16) -> i16 {
    let divisor = 1_i32.checked_shl(u32::from(shift)).unwrap_or(0);
    if divisor == 0 {
        return 0;
    }
    ((i32::from(x) * i32::from(y)) / divisor) as i16
}

#[unsafe(no_mangle)]
pub fn MathUtil_Mul32(x: i32, y: i32) -> i32 {
    ((i64::from(x) * i64::from(y)) / 256) as i32
}

#[unsafe(no_mangle)]
pub fn MathUtil_Div16(x: i16, y: i16) -> i16 {
    if y == 0 {
        return 0;
    }
    ((i32::from(x) * 256) / i32::from(y)) as i16
}

#[unsafe(no_mangle)]
pub fn MathUtil_Div16Shift(shift: u8, x: i16, y: i16) -> i16 {
    if y == 0 || shift >= 31 {
        return 0;
    }
    let multiplier = i64::from(1_i32 << shift);
    ((i64::from(x) * multiplier) / i64::from(y)) as i16
}

#[unsafe(no_mangle)]
pub fn MathUtil_Div32(x: i32, y: i32) -> i32 {
    if y == 0 {
        return 0;
    }
    ((i64::from(x) * 256) / i64::from(y)) as i32
}

#[unsafe(no_mangle)]
pub fn MathUtil_Inv16(y: i16) -> i16 {
    if y == 0 {
        return 0;
    }
    (0x1_0000_i32 / i32::from(y)) as i16
}

#[unsafe(no_mangle)]
pub fn MathUtil_Inv16Shift(shift: u8, y: i16) -> i16 {
    if y == 0 || shift >= 23 {
        return 0;
    }
    let numerator = i64::from(0x100_i32 << shift);
    (numerator / i64::from(y)) as i16
}

#[unsafe(no_mangle)]
pub fn MathUtil_Inv32(y: i32) -> i32 {
    if y == 0 {
        return 0;
    }
    (0x1_0000_i64 / i64::from(y)) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_point_multiplication_matches_c_formulas() {
        assert_eq!(MathUtil_Mul16(384, 128), 192);
        assert_eq!(MathUtil_Mul16(-384, 128), -192);
        assert_eq!(MathUtil_Mul16Shift(4, 32, 12), 24);
        assert_eq!(MathUtil_Mul32(100_000, -512), -200_000);
    }

    #[test]
    fn fixed_point_division_matches_c_formulas() {
        assert_eq!(MathUtil_Div16(3, 2), 384);
        assert_eq!(MathUtil_Div16(-3, 2), -384);
        assert_eq!(MathUtil_Div16Shift(4, 30, 3), 160);
        assert_eq!(MathUtil_Div32(100_000, 200), 128_000);
    }

    #[test]
    fn division_by_zero_follows_existing_guarded_behavior() {
        assert_eq!(MathUtil_Div16(10, 0), 0);
        assert_eq!(MathUtil_Div16Shift(8, 10, 0), 0);
        assert_eq!(MathUtil_Div32(10, 0), 0);
    }

    #[test]
    fn inverse_helpers_use_expected_fixed_point_scales() {
        assert_eq!(MathUtil_Inv16(256), 256);
        assert_eq!(MathUtil_Inv16Shift(8, 256), 256);
        assert_eq!(MathUtil_Inv32(256), 256);
    }
}
