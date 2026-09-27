const SINE_QUARTER: [i16; 65] = [
    0, 6, 12, 18, 25, 31, 37, 43, 49, 56, 62, 68, 74, 80, 86, 92, 97, 103, 109, 115, 120, 126, 131,
    136, 142, 147, 152, 157, 162, 167, 171, 176, 181, 185, 189, 193, 197, 201, 205, 209, 212, 216,
    219, 222, 225, 228, 231, 234, 236, 238, 241, 243, 244, 246, 248, 249, 251, 252, 253, 254, 254,
    255, 255, 255, 256,
];

const SINE_DEGREE_QUARTER: [i16; 91] = [
    0, 71, 143, 214, 286, 357, 428, 499, 570, 641, 711, 782, 852, 921, 991, 1060, 1129, 1198, 1266,
    1334, 1401, 1468, 1534, 1600, 1666, 1731, 1796, 1860, 1923, 1986, 2048, 2110, 2171, 2231, 2290,
    2349, 2408, 2465, 2522, 2578, 2633, 2687, 2741, 2793, 2845, 2896, 2946, 2996, 3044, 3091, 3138,
    3183, 3228, 3271, 3314, 3355, 3396, 3435, 3474, 3511, 3547, 3582, 3617, 3650, 3681, 3712, 3742,
    3770, 3798, 3824, 3849, 3873, 3896, 3917, 3937, 3956, 3974, 3991, 4006, 4021, 4034, 4046, 4056,
    4065, 4073, 4080, 4086, 4090, 4093, 4095, 4096,
];

const fn build_sine_table() -> [i16; 320] {
    let mut table = [0; 320];
    let mut index = 0;
    while index < table.len() {
        let phase = index & 0xff;
        table[index] = if phase <= 64 {
            SINE_QUARTER[phase]
        } else if phase <= 128 {
            SINE_QUARTER[128 - phase]
        } else if phase <= 192 {
            -SINE_QUARTER[phase - 128]
        } else {
            -SINE_QUARTER[256 - phase]
        };
        index += 1;
    }
    table
}

const fn build_sine_degree_table() -> [i16; 180] {
    let mut table = [0; 180];
    let mut angle = 0;
    while angle < table.len() {
        table[angle] = if angle <= 90 {
            SINE_DEGREE_QUARTER[angle]
        } else {
            SINE_DEGREE_QUARTER[180 - angle]
        };
        angle += 1;
    }
    table
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".rodata")]
pub static gSineTable: [i16; 320] = build_sine_table();

#[unsafe(no_mangle)]
#[unsafe(link_section = ".rodata")]
pub static gSineDegreeTable: [i16; 180] = build_sine_degree_table();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sin(index: i16, amplitude: i16) -> i16 {
    let value = unsafe {
        (&raw const gSineTable)
            .cast::<i16>()
            .add(index as usize)
            .read()
    };
    ((i32::from(amplitude) * i32::from(value)) >> 8) as i16
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Cos(index: i16, amplitude: i16) -> i16 {
    unsafe { Sin(index.wrapping_add(64), amplitude) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sin2(angle: u16) -> i16 {
    let angle_mod = usize::from(angle % 180);
    let value = unsafe {
        (&raw const gSineDegreeTable)
            .cast::<i16>()
            .add(angle_mod)
            .read()
    };
    if ((angle / 180) & 1) != 0 {
        -value
    } else {
        value
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Cos2(angle: u16) -> i16 {
    unsafe { Sin2(angle.wrapping_add(90)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_tables_have_the_original_shapes_and_anchor_values() {
        assert_eq!(gSineTable.len(), 320);
        assert_eq!(gSineTable[0], 0);
        assert_eq!(gSineTable[64], 256);
        assert_eq!(gSineTable[128], 0);
        assert_eq!(gSineTable[192], -256);
        assert_eq!(gSineTable[256], 0);
        assert_eq!(gSineTable[319], 255);

        assert_eq!(gSineDegreeTable.len(), 180);
        assert_eq!(gSineDegreeTable[0], 0);
        assert_eq!(gSineDegreeTable[30], 2048);
        assert_eq!(gSineDegreeTable[90], 4096);
        assert_eq!(gSineDegreeTable[179], 71);
    }

    #[test]
    fn fixed_point_trig_functions_match_cardinal_angles() {
        unsafe {
            assert_eq!(Sin(64, 100), 100);
            assert_eq!(Cos(0, 100), 100);
            assert_eq!(Sin2(90), 4096);
            assert_eq!(Sin2(270), -4096);
            assert_eq!(Cos2(0), 4096);
            assert_eq!(Cos2(180), -4096);
        }
    }
}
