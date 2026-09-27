//! Translated from `src/pokemon_animation.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_mut,
    unused_variables,
    unused_assignments,
    unused_parens,
    unused_braces,
    unused_labels,
    unused_comparisons,
    overflowing_literals,
    unused_unsafe,
    dead_code,
    unreachable_code,
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): sSpeciesToBackAnimSet sYellowFlashData sVerticalShakeData sMonAnimFunctions sBackAnimationIds sBackAnimNatureModTable sMonAffineAnim_0 sMonAffineAnim_1 sMonAffineAnims sZigzagData sBounceRotateToSidesData sTriangleDownData sShakeYellowFlashData_Fast sShakeYellowFlashData_Normal sShakeYellowFlashData_Slow sShakeYellowFlashData sColors.0
#[allow(unused_imports)]
use crate::data::pokemon_animation::*;

pub(crate) static mut sAnims: crate::ffi::Align4<[u8; 48]> = crate::ffi::Align4([0; 48]);
pub(crate) static mut sAnimIdx: u8 = 0u8;
pub(crate) static mut sIsSummaryAnim: u32 = 0u32;

unsafe extern "C" {
    static mut gBattlerPartyIndexes: u8;
    static mut gOamMatrices: u8;
    static mut gPlayerParty: u8;
    static mut gTasks: u8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn CalcCenterToCornerVec(a0: *mut u8, a1: u8, a2: u8, a3: u8);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn FreeOamMatrix(a0: u8);
    fn GetNature(a0: *mut u8) -> u8;
    fn InitSpriteAffineAnim(a0: *mut u8);
    fn ObjAffineSet(a0: *mut u8, a1: *mut u8, a2: i32, a3: i32);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
}

pub(crate) unsafe extern "C" fn MonAnimDummySpriteCallback(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
pub(crate) unsafe extern "C" fn SetPosForRotation(
    sprite: *mut u8,
    index: u16,
    amplitudeX: i16,
    amplitudeY: i16,
) {
    unsafe {
        let mut sprite = sprite;
        let mut index = index;
        let mut amplitudeX = amplitudeX;
        let mut amplitudeY = amplitudeY;
        let mut xAdder: i16 = 0i16;
        let mut yAdder: i16 = 0i16;
        amplitudeX = ((((amplitudeX) as i32).wrapping_mul((-1i32))) as i16);
        amplitudeY = ((((amplitudeY) as i32).wrapping_mul((-1i32))) as i16);
        xAdder = ((((Cos(((index) as i16), amplitudeX)) as i32)
            .wrapping_sub(((Sin(((index) as i16), amplitudeY)) as i32))) as i16);
        yAdder = ((((Cos(((index) as i16), amplitudeY)) as i32)
            .wrapping_add(((Sin(((index) as i16), amplitudeX)) as i32))) as i16);
        amplitudeX = ((((amplitudeX) as i32).wrapping_mul((-1i32))) as i16);
        amplitudeY = ((((amplitudeY) as i32).wrapping_mul((-1i32))) as i16);
        ((sprite).wrapping_add(36).cast::<i16>())
            .write(((((xAdder) as i32).wrapping_add(((amplitudeX) as i32))) as i16));
        ((sprite).wrapping_add(38).cast::<i16>())
            .write(((((yAdder) as i32).wrapping_add(((amplitudeY) as i32))) as i16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSpeciesBackAnimSet(species: u16) -> u8 {
    unsafe {
        let mut species = species;
        if ((((((&raw const sSpeciesToBackAnimSet).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((species) as i32) as isize))
        .read()) as i32)
            != 0i32
        {
            return ((((((((&raw const sSpeciesToBackAnimSet).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((species) as i32) as isize))
            .read()) as i32)
                .wrapping_sub(1i32)) as u8);
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleMonAnimation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u32 = 0u32;
        let mut sprite: *mut u8 = (((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            << 16)
            | (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as u16) as i32)) as usize as *mut u8);
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write((((sprite).wrapping_add(46)).cast::<i16>()).read());
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read());
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(1i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            {
                i = 2u32;
                'l1: loop {
                    if !(i < crate::c::div_u32(16u32, 2u32)) {
                        break 'l1;
                    }
                    'l2: {
                        ((((sprite).wrapping_add(46)).cast::<i16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(0i16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(
                ((((&raw const sMonAnimFunctions)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32) as isize,
                ))
                .read(),
            );
            ((&raw mut sIsSummaryAnim).cast::<u8>().cast::<u32>()).write(0u32);
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if core::mem::transmute::<_, usize>(
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) == (SpriteCallbackDummy as *const () as usize)
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read(),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read(),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LaunchAnimationTaskForFrontSprite(sprite: *mut u8, frontAnimId: u8) {
    unsafe {
        let mut sprite = sprite;
        let mut frontAnimId = frontAnimId;
        let mut taskId: u8 = CreateTask(Some(Task_HandleMonAnimation), 128u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((((sprite) as usize as u32) >> 16) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write((((sprite) as usize as u32) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((frontAnimId) as i16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartMonSummaryAnimation(sprite: *mut u8, frontAnimId: u8) {
    unsafe {
        let mut sprite = sprite;
        let mut frontAnimId = frontAnimId;
        ((&raw mut sIsSummaryAnim).cast::<u8>().cast::<u32>()).write(1u32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(
            ((((&raw const sMonAnimFunctions)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .wrapping_offset(((frontAnimId) as i32) as isize))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LaunchAnimationTaskForBackSprite(sprite: *mut u8, backAnimSet: u8) {
    unsafe {
        let mut sprite = sprite;
        let mut backAnimSet = backAnimSet;
        let mut nature: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        let mut animId: u8 = 0u8;
        let mut battler: u8 = 0u8;
        taskId = CreateTask(Some(Task_HandleMonAnimation), 128u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((((sprite) as usize as u32) >> 16) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write((((sprite) as usize as u32) as i16));
        battler = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8);
        nature = GetNature(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as i32) as isize
                    * 100,
            ),
        );
        animId = ((((3i32).wrapping_mul(((backAnimSet) as i32))).wrapping_add(
            ((((((&raw const sBackAnimNatureModTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((nature) as i32) as isize))
            .read()) as i32),
        )) as u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(
            ((((((&raw const sBackAnimationIds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((animId) as i32) as isize))
            .read()) as i16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSpriteCB_MonAnimDummy(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(MonAnimDummySpriteCallback));
    }
}
pub(crate) unsafe extern "C" fn SetAffineData(
    sprite: *mut u8,
    xScale: i16,
    yScale: i16,
    rotation: u16,
) {
    unsafe {
        let mut sprite = sprite;
        let mut xScale = xScale;
        let mut yScale = yScale;
        let mut rotation = rotation;
        let mut matrixNum: u8 = 0u8;
        let mut affineSrcData = crate::ffi::Align4([0u8; 8]);
        let mut dest = crate::ffi::Align4([0u8; 8]);
        (((&raw mut affineSrcData).cast::<u8>()).cast::<i16>()).write(xScale);
        (((&raw mut affineSrcData).cast::<u8>())
            .wrapping_add(2)
            .cast::<i16>())
        .write(yScale);
        (((&raw mut affineSrcData).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(rotation);
        matrixNum = ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8);
        ObjAffineSet(
            (&raw mut affineSrcData).cast::<u8>(),
            (&raw mut dest).cast::<u8>(),
            1i32,
            2i32,
        );
        ((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrixNum) as i32) as isize * 8))
        .cast::<i16>())
        .write((((&raw mut dest).cast::<u8>()).cast::<i16>()).read());
        ((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrixNum) as i32) as isize * 8))
        .wrapping_add(2)
        .cast::<i16>())
        .write((((&raw mut dest).cast::<u8>()).wrapping_add(2).cast::<i16>()).read());
        ((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrixNum) as i32) as isize * 8))
        .wrapping_add(4)
        .cast::<i16>())
        .write((((&raw mut dest).cast::<u8>()).wrapping_add(4).cast::<i16>()).read());
        ((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrixNum) as i32) as isize * 8))
        .wrapping_add(6)
        .cast::<i16>())
        .write((((&raw mut dest).cast::<u8>()).wrapping_add(6).cast::<i16>()).read());
    }
}
pub(crate) unsafe extern "C" fn HandleStartAffineAnim(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (3u32) as i32);
        ((sprite).wrapping_add(16).cast::<*mut *mut u8>()).write(
            ((&raw const sMonAffineAnims)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>(),
        );
        if ((&raw mut sIsSummaryAnim).cast::<u8>().cast::<u32>()).read() == 1u32 {
            InitSpriteAffineAnim(sprite);
        }
        if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0) {
            StartSpriteAffineAnim(sprite, 1u8);
        } else {
            StartSpriteAffineAnim(sprite, 0u8);
        }
        CalcCenterToCornerVec(
            sprite,
            ((crate::c::bf_read((sprite).wrapping_add(1), 6, 2, false) as u32) as u8),
            ((crate::c::bf_read((sprite).wrapping_add(3), 6, 2, false) as u32) as u8),
            ((crate::c::bf_read((sprite).wrapping_add(1), 0, 2, false) as u32) as u8),
        );
        crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
    }
}
pub(crate) unsafe extern "C" fn HandleSetAffineData(
    sprite: *mut u8,
    xScale: i16,
    yScale: i16,
    rotation: u16,
) {
    unsafe {
        let mut sprite = sprite;
        let mut xScale = xScale;
        let mut yScale = yScale;
        let mut rotation = rotation;
        if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0) {
            xScale = ((((xScale) as i32).wrapping_mul((-1i32))) as i16);
            rotation = ((((rotation) as i32).wrapping_mul((-1i32))) as u16);
        }
        SetAffineData(sprite, xScale, yScale, rotation);
    }
}
pub(crate) unsafe extern "C" fn TryFlipX(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0) {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_mul((-1i32))) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn InitAnimData(id: u8) -> u32 {
    unsafe {
        let mut id = id;
        if ((id) as i32) >= 4i32 {
            return 0u32;
        } else {
            (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(6)
            .cast::<i16>())
            .write(0i16);
            (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 12))
            .cast::<u16>())
            .write(0u16);
            (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(4)
            .cast::<i16>())
            .write(1i16);
            (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(2)
            .cast::<i16>())
            .write(0i16);
            (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(8)
            .cast::<i16>())
            .write(0i16);
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn AddNewAnim() -> u8 {
    unsafe {
        ((&raw mut sAnimIdx).cast::<u8>().cast::<u8>()).write(
            ((crate::c::rem_i32(
                ((((&raw mut sAnimIdx).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_add(1i32),
                4i32,
            )) as u8),
        );
        InitAnimData(((&raw mut sAnimIdx).cast::<u8>().cast::<u8>()).read());
        return ((&raw mut sAnimIdx).cast::<u8>().cast::<u8>()).read();
    }
}
pub(crate) unsafe extern "C" fn ResetSpriteAfterAnim(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (1u32) as i32);
        CalcCenterToCornerVec(
            sprite,
            ((crate::c::bf_read((sprite).wrapping_add(1), 6, 2, false) as u32) as u8),
            ((crate::c::bf_read((sprite).wrapping_add(3), 6, 2, false) as u32) as u8),
            ((crate::c::bf_read((sprite).wrapping_add(1), 0, 2, false) as u32) as u8),
        );
        if ((&raw mut sIsSummaryAnim).cast::<u8>().cast::<u32>()).read() == 1u32 {
            if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0) {
                crate::c::bf_write((sprite).wrapping_add(63), 0, 1, (1u16) as i32);
            } else {
                crate::c::bf_write((sprite).wrapping_add(63), 0, 1, (0u16) as i32);
            }
            FreeOamMatrix(
                ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
            );
            crate::c::bf_write(
                (sprite).wrapping_add(3),
                1,
                5,
                ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32)
                    | ((((crate::c::bf_read((sprite).wrapping_add(63), 0, 1, false) as u16) as i32)
                        << 3) as u32)) as i32,
            );
            crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (0u32) as i32);
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_CircularStretchTwice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 40i32
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let mut var: i16 = ((crate::c::rem_i32(
                crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        .wrapping_mul(512i32),
                    40i32,
                ),
                256i32,
            )) as i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                .write(((((Sin(var, 32i16)) as i32).wrapping_add(256i32)) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                .write(((((Cos(var, 32i16)) as i32).wrapping_add(256i32)) as i16));
            HandleSetAffineData(
                sprite,
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                0u16,
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalVibrate(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 40i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        } else {
            let mut sign: i8 = 0i8;
            if !((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                as i32)
                & 1i32)
                != 0)
            {
                sign = 1i8;
            } else {
                sign = (-1i8);
            }
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((Sin(
                    ((crate::c::rem_i32(
                        crate::c::div_i32(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                .read()) as i32)
                                .wrapping_mul(128i32),
                            40i32,
                        ),
                        256i32,
                    )) as i16),
                    6i16,
                )) as i32)
                    .wrapping_mul(((sign) as i32))) as i16),
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn HorizontalSlide(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                ((crate::c::rem_i32(
                    crate::c::div_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            .wrapping_mul(384i32),
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                    ),
                    256i32,
                )) as i16),
                6i16,
            ));
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalSlide(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(40i16);
        HorizontalSlide(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(HorizontalSlide));
    }
}
pub(crate) unsafe extern "C" fn VerticalSlide(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        } else {
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(
                    ((crate::c::rem_i32(
                        crate::c::div_i32(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                .read()) as i32)
                                .wrapping_mul(384i32),
                            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                        ),
                        256i32,
                    )) as i16),
                    6i16,
                )) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalSlide(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(40i16);
        VerticalSlide(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalSlide));
    }
}
pub(crate) unsafe extern "C" fn VerticalJumps(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut counter: i32 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32);
        if counter > 384i32 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        } else {
            let mut divCounter: i16 = ((crate::c::div_i32(counter, 128i32)) as i16);
            'l1: {
                let __sw1 = ((divCounter) as i32);
                if __sw1 == 0i32 || __sw1 == 1i32 {
                    ((sprite).wrapping_add(38).cast::<i16>()).write(
                        ((((Sin(
                            ((crate::c::rem_i32(counter, 128i32)) as i16),
                            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
                                .wrapping_mul(2i32)) as i16),
                        )) as i32)
                            .wrapping_neg()) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 2i32 || __sw1 == 3i32 {
                    counter = (counter).wrapping_sub(256i32);
                    ((sprite).wrapping_add(38).cast::<i16>()).write(
                        ((((Sin(
                            ((counter) as i16),
                            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
                                .wrapping_mul(3i32)) as i16),
                        )) as i32)
                            .wrapping_neg()) as i16),
                    );
                    break 'l1;
                }
            }
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(12i32)) as i16));
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalJumps_Big(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(4i16);
        VerticalJumps(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalJumps));
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalJumpsHorizontalJumps(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut counter: i32 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32);
        if counter > 768i32 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        } else {
            let mut divCounter: i16 = ((crate::c::div_i32(counter, 128i32)) as i16);
            'l1: {
                let __sw1 = ((divCounter) as i32);
                if __sw1 == 0i32 || __sw1 == 1i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    counter = 0i32;
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        ((crate::c::div_i32(
                            ((crate::c::rem_i32(counter, 128i32)).wrapping_mul(8i32))
                                .wrapping_neg(),
                            128i32,
                        )) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        (((crate::c::div_i32(crate::c::rem_i32(counter, 128i32), 8i32))
                            .wrapping_sub(8i32)) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 5i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        (((crate::c::div_i32(
                            ((crate::c::rem_i32(counter, 128i32)).wrapping_mul(8i32))
                                .wrapping_neg(),
                            128i32,
                        ))
                        .wrapping_add(8i32)) as i16),
                    );
                    break 'l1;
                }
            }
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(((crate::c::rem_i32(counter, 128i32)) as i16), 8i16)) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(12i32)) as i16));
    }
}
pub(crate) unsafe extern "C" fn Anim_GrowVibrate(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 40i32
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let mut index: i16 = ((crate::c::rem_i32(
                crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        .wrapping_mul(256i32),
                    40i32,
                ),
                256i32,
            )) as i16);
            if crate::c::rem_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
                2i32,
            ) == 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                    .write(((((Sin(index, 32i16)) as i32).wrapping_add(256i32)) as i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                    .write(((((Sin(index, 32i16)) as i32).wrapping_add(256i32)) as i16));
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                    .write(((((Sin(index, 8i16)) as i32).wrapping_add(256i32)) as i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                    .write(((((Sin(index, 8i16)) as i32).wrapping_add(256i32)) as i16));
            }
            HandleSetAffineData(
                sprite,
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                0u16,
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Zigzag(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        }
        if ((((((((&raw const sZigzagData).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    as isize
                    * 3,
            ))
        .cast::<i8>())
        .wrapping_offset(2))
        .read()) as i32)
            == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
        {
            if ((((((((&raw const sZigzagData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32) as isize
                        * 3,
                ))
            .cast::<i8>())
            .wrapping_offset(2))
            .read()) as i32)
                == 0i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            } else {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p1).write(((__p1).read()).wrapping_add(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            }
        }
        if ((((((((&raw const sZigzagData).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    as isize
                    * 3,
            ))
        .cast::<i8>())
        .wrapping_offset(2))
        .read()) as i32)
            == 0i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let __p2 = (sprite).wrapping_add(36).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    (((((((&raw const sZigzagData).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                                .read()) as i32) as isize
                                * 3,
                        ))
                    .cast::<i8>())
                    .read()) as i32),
                )) as i16),
            );
            let __p3 = (sprite).wrapping_add(38).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((((&raw const sZigzagData).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                                .read()) as i32) as isize
                                * 3,
                        ))
                    .cast::<i8>())
                    .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
            );
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p4).write(((__p4).read()).wrapping_add(1));
            TryFlipX(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_ZigzagFast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        Zigzag(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(Zigzag));
    }
}
pub(crate) unsafe extern "C" fn HorizontalShake(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut counter: i32 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32);
        if counter > 2304i32 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                ((crate::c::rem_i32(counter, 256i32)) as i16),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
            ));
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalShake(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(60i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(3i16);
        HorizontalShake(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(HorizontalShake));
    }
}
pub(crate) unsafe extern "C" fn VerticalShake(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut counter: i32 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32);
        if counter > 2304i32 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        } else {
            ((sprite).wrapping_add(38).cast::<i16>())
                .write(Sin(((crate::c::rem_i32(counter, 256i32)) as i16), 3i16));
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalShake(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(60i16);
        VerticalShake(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalShake));
    }
}
pub(crate) unsafe extern "C" fn Anim_CircularVibrate(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 512i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        } else {
            let mut sign: i8 = 0i8;
            let mut index: i32 = 0i32;
            let mut amplitude: i32 = 0i32;
            if !((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                as i32)
                & 1i32)
                != 0)
            {
                sign = 1i8;
            } else {
                sign = (-1i8);
            }
            amplitude = ((Sin(
                ((crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                    4i32,
                )) as i16),
                8i16,
            )) as i32);
            index = crate::c::rem_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
                256i32,
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(((index) as i16), ((amplitude) as i16))) as i32)
                    .wrapping_mul(((sign) as i32))) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((Cos(((index) as i16), ((amplitude) as i16))) as i32)
                    .wrapping_mul(((sign) as i32))) as i16),
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(9i32)) as i16));
    }
}
pub(crate) unsafe extern "C" fn Twist(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: i16 = (((sprite).wrapping_add(46)).cast::<i16>()).read();
        if (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            let __p1 = ((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 12))
            .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == 0i32)
                && ((((((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 12))
                .wrapping_add(8)
                .cast::<i16>())
                .read()) as i32)
                    == 0i32)
            {
                HandleStartAffineAnim(sprite);
                let __p2 = ((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 12))
                .wrapping_add(8)
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 12))
                .wrapping_add(6)
                .cast::<i16>())
                .read()) as i32)
            {
                HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
                if (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 12))
                .wrapping_add(4)
                .cast::<i16>())
                .read()) as i32)
                    > 1i32
                {
                    let __p3 = ((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 12))
                    .wrapping_add(4)
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_sub(1));
                    (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 12))
                    .cast::<u16>())
                    .write(10u16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                } else {
                    ResetSpriteAfterAnim(sprite);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(WaitAnimEnd));
                }
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(Sin(
                    ((crate::c::rem_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                        256i32,
                    )) as i16),
                    4096i16,
                ));
                HandleSetAffineData(
                    sprite,
                    256i16,
                    256i16,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as u16),
                );
            }
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p4).write((((((__p4).read()) as i32).wrapping_add(16i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_Twist(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(6)
        .cast::<i16>())
        .write(512i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .cast::<u16>())
        .write(0u16);
        Twist(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(Twist));
    }
}
pub(crate) unsafe extern "C" fn Spin(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 12))
            .cast::<u16>())
            .read()) as i32)
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
                (((crate::c::div_i32(
                    65536i32,
                    (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 12))
                    .wrapping_add(8)
                    .cast::<i16>())
                    .read()) as i32),
                ))
                .wrapping_mul(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            HandleSetAffineData(
                sprite,
                256i16,
                256i16,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u16),
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_Spin_Long(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .cast::<u16>())
        .write(60u16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(20i16);
        Spin(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(Spin));
    }
}
pub(crate) unsafe extern "C" fn CircleCounterclockwise(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8);
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(6)
            .cast::<i16>())
            .read()) as i32)
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let mut index: i16 = ((crate::c::rem_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_add(192i32),
                256i32,
            )) as i16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((Cos(
                    index,
                    (((((((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 12))
                    .wrapping_add(8)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_mul(2i32)) as i16),
                )) as i32)
                    .wrapping_neg()) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(
                    index,
                    (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((id) as i32) as isize * 12))
                    .wrapping_add(8)
                    .cast::<i16>())
                    .read(),
                )) as i32)
                    .wrapping_add(
                        (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((id) as i32) as isize * 12))
                        .wrapping_add(8)
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 12))
                .wrapping_add(2)
                .cast::<i16>())
                .read()) as i32),
            )) as i16),
        );
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_CircleCounterclockwise(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(6)
        .cast::<i16>())
        .write(512i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(6i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(2)
        .cast::<i16>())
        .write(24i16);
        CircleCounterclockwise(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(CircleCounterclockwise));
    }
}
pub(crate) unsafe extern "C" fn Anim_GlowBlack(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                    (((256i32).wrapping_add(
                        ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                            .wrapping_mul(16i32),
                    )) as i16),
                );
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 128i32
            {
                BlendPalette(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as u16),
                    16u16,
                    0u8,
                    0u16,
                );
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                    16i16,
                ));
                BlendPalette(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as u16),
                    16u16,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as u8),
                    0u16,
                );
            }
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(1i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalStretch(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut index1: i16 = 0i16;
        let mut index2: i16 = 0i16;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 40i32
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            index2 = ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_mul(128i32),
                40i32,
            )) as i16);
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                >= 10i32)
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    <= 29i32)
            {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                (__p1).write((((((__p1).read()) as i32).wrapping_add(51i32)) as i16));
                index1 = ((255i32
                    & ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)) as i16);
            }
            if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0) {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                    (((((Sin(index2, 40i16)) as i32).wrapping_sub(256i32))
                        .wrapping_add(((Sin(index1, 16i16)) as i32))) as i16),
                );
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                    ((((256i32).wrapping_sub(((Sin(index2, 40i16)) as i32)))
                        .wrapping_sub(((Sin(index1, 16i16)) as i32))) as i16),
                );
            }
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                .write(((((Sin(index2, 16i16)) as i32).wrapping_add(256i32)) as i16));
            SetAffineData(
                sprite,
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                0u16,
            );
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalStretch(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut posY: i16 = 0i16;
        let mut index1: i16 = 0i16;
        let mut index2: i16 = 0i16;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 40i32
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(38).cast::<i16>()).write(posY);
        } else {
            index2 = ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_mul(128i32),
                40i32,
            )) as i16);
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                >= 10i32)
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    <= 29i32)
            {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                (__p1).write((((((__p1).read()) as i32).wrapping_add(51i32)) as i16));
                index1 = ((255i32
                    & ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)) as i16);
            }
            if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0) {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                    (((((Sin(index2, 16i16)) as i32).wrapping_neg()).wrapping_sub(256i32)) as i16),
                );
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                    .write(((((Sin(index2, 16i16)) as i32).wrapping_add(256i32)) as i16));
            }
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                ((((256i32).wrapping_sub(((Sin(index2, 40i16)) as i32)))
                    .wrapping_sub(((Sin(index1, 8i16)) as i32))) as i16),
            );
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                != 256i32
            {
                posY = ((crate::c::div_i32(
                    (256i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32),
                    ),
                    8i32,
                )) as i16);
            }
            ((sprite).wrapping_add(38).cast::<i16>())
                .write(((((posY) as i32).wrapping_neg()) as i16));
            SetAffineData(
                sprite,
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                0u16,
            );
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn VerticalShakeTwice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut index: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8);
        let mut var7: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u8);
        let mut var5: u8 = (((((&raw const sVerticalShakeData).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                as isize
                * 2,
        ))
        .cast::<u8>())
        .read();
        let mut var6: u8 = ((((((&raw const sVerticalShakeData).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                as isize
                * 2,
        ))
        .cast::<u8>())
        .wrapping_offset(1))
        .read();
        let mut amplitude: u8 = 0u8;
        if ((var5) as i32) != 254i32 {
            amplitude = ((crate::c::div_i32(
                (((var6) as i32).wrapping_sub(((var7) as i32))).wrapping_mul(((var5) as i32)),
                ((var6) as i32),
            )) as u8);
        } else {
            amplitude = 0u8;
        }
        if ((var5) as i32) == 255i32 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        } else {
            ((sprite).wrapping_add(38).cast::<i16>())
                .write(Sin(((index) as i16), ((amplitude) as i16)));
            if ((var7) as i32) == ((var6) as i32) {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p1).write(((__p1).read()).wrapping_add(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            } else {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                    )) as i16),
                );
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalShakeTwice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(48i16);
        VerticalShakeTwice(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalShakeTwice));
    }
}
pub(crate) unsafe extern "C" fn Anim_TipMoveForward(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut counter: u8 = 0u8;
        TryFlipX(sprite);
        counter = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 35i32
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        } else {
            let mut index: i16 = ((crate::c::div_i32(
                (((counter) as i32).wrapping_sub(10i32)).wrapping_mul(128i32),
                20i32,
            )) as i16);
            if ((counter) as i32) < 10i32 {
                HandleSetAffineData(
                    sprite,
                    256i16,
                    256i16,
                    (((crate::c::div_i32(((counter) as i32), 2i32)).wrapping_mul(512i32)) as u16),
                );
            } else {
                if (((counter) as i32) >= 10i32) && (((counter) as i32) <= 29i32) {
                    ((sprite).wrapping_add(36).cast::<i16>())
                        .write(((((Sin(index, 5i16)) as i32).wrapping_neg()) as i16));
                } else {
                    HandleSetAffineData(
                        sprite,
                        256i16,
                        256i16,
                        (((crate::c::div_i32((35i32).wrapping_sub(((counter) as i32)), 2i32))
                            .wrapping_mul(1024i32)) as u16),
                    );
                }
            }
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalPivot(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 100i32
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let mut index: i16 = ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_mul(256i32),
                100i32,
            )) as i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(index, 10i16));
            HandleSetAffineData(sprite, 256i16, 256i16, ((Sin(index, 3276i16)) as u16));
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn VerticalSlideWobble(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut var: i32 = 0i32;
        let mut index: i16 = 0i16;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 100i32
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            index = ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_mul(256i32),
                100i32,
            )) as i16);
            var = crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_mul(512i32),
                100i32,
            );
            var = (var & 255i32);
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                index,
                (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            ));
            HandleSetAffineData(
                sprite,
                256i16,
                256i16,
                ((Sin(((var) as i16), 3276i16)) as u16),
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalSlideWobble(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(10i16);
        VerticalSlideWobble(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalSlideWobble));
    }
}
pub(crate) unsafe extern "C" fn RisingWobble(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut var: i32 = 0i32;
        let mut index: i16 = 0i16;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 100i32
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            index = ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_mul(256i32),
                100i32,
            )) as i16);
            var = crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_mul(512i32),
                100i32,
            );
            var = (var & 255i32);
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(
                    ((crate::c::div_i32(((index) as i32), 2i32)) as i16),
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
                        .wrapping_mul(2i32)) as i16),
                )) as i32)
                    .wrapping_neg()) as i16),
            );
            HandleSetAffineData(
                sprite,
                256i16,
                256i16,
                ((Sin(((var) as i16), 3276i16)) as u16),
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_RisingWobble(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(5i16);
        RisingWobble(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RisingWobble));
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalSlideWobble(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut var: i32 = 0i32;
        let mut index: i16 = 0i16;
        TryFlipX(sprite);
        var = 0i32;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 100i32
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            index = ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_mul(256i32),
                100i32,
            )) as i16);
            var = crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_mul(512i32),
                100i32,
            );
            var = (var & 255i32);
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(index, 8i16));
            HandleSetAffineData(
                sprite,
                256i16,
                256i16,
                ((Sin(((var) as i16), 3276i16)) as u16),
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn VerticalSquishBounce(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut posY: i16 = 0i16;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        }
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(3i32)
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let mut yScale: i16 = ((((Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                32i16,
            )) as i32)
                .wrapping_add(256i32)) as i16);
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32))
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    < (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
                        .wrapping_mul(2i32))
            {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p1).write(
                    (((((__p1).read()) as i32).wrapping_add(crate::c::div_i32(
                        128i32,
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                    ))) as i16),
                );
            }
            if ((yScale) as i32) > 256i32 {
                posY = ((crate::c::div_i32((256i32).wrapping_sub(((yScale) as i32)), 8i32)) as i16);
            }
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                (((((Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read(),
                    10i16,
                )) as i32)
                    .wrapping_neg())
                .wrapping_sub(((posY) as i32))) as i16),
            );
            HandleSetAffineData(
                sprite,
                (((256i32).wrapping_sub(
                    ((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                        32i16,
                    )) as i32),
                )) as i16),
                yScale,
                0u16,
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(((__p2).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    .wrapping_add(crate::c::div_i32(
                        128i32,
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                    ))
                    & 255i32) as i16),
            );
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalSquishBounce(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(16i16);
        VerticalSquishBounce(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalSquishBounce));
    }
}
pub(crate) unsafe extern "C" fn ShrinkGrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut posY: i16 = 0i16;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > (crate::c::div_i32(
                128i32,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            ))
            .wrapping_mul(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            )
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let mut yScale: i16 = ((((Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                32i16,
            )) as i32)
                .wrapping_add(256i32)) as i16);
            if ((yScale) as i32) > 256i32 {
                posY = ((crate::c::div_i32((256i32).wrapping_sub(((yScale) as i32)), 8i32)) as i16);
            }
            ((sprite).wrapping_add(38).cast::<i16>())
                .write(((((posY) as i32).wrapping_neg()) as i16));
            HandleSetAffineData(
                sprite,
                ((((Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                    48i16,
                )) as i32)
                    .wrapping_add(256i32)) as i16),
                yScale,
                0u16,
            );
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    .wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                    )
                    & 255i32) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_ShrinkGrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(3i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(8i16);
        }
        ShrinkGrow(sprite);
    }
}
pub(crate) unsafe extern "C" fn BounceRotateToSides(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut var: i16 = 0i16;
        let mut structId: u8 = 0u8;
        let mut r9: i8 = 0i8;
        let mut r10: i16 = 0i16;
        let mut r7: i16 = 0i16;
        let mut arrId: u32 = 0u32;
        TryFlipX(sprite);
        structId = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8);
        var = (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((structId) as i32) as isize * 12))
        .wrapping_add(6)
        .cast::<i16>())
        .read();
        r9 = (((((((&raw const sBounceRotateToSidesData)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(
            (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((structId) as i32) as isize * 12))
            .wrapping_add(8)
            .cast::<i16>())
            .read()) as i32) as isize
                * 24,
        ))
        .cast::<u8>())
        .wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                as isize
                * 3,
        ))
        .cast::<i8>())
        .read();
        r10 = ((((((((((((&raw const sBounceRotateToSidesData)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(
            (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((structId) as i32) as isize * 12))
            .wrapping_add(8)
            .cast::<i16>())
            .read()) as i32) as isize
                * 24,
        ))
        .cast::<u8>())
        .wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                as isize
                * 3,
        ))
        .cast::<i8>())
        .wrapping_offset(1))
        .read()) as i32)
            .wrapping_sub(((r9) as i32))) as i16);
        arrId = (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((structId) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .read()) as u32);
        r7 = ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read();
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if ((((((((((&raw const sBounceRotateToSidesData)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(((arrId) as i32) as isize * 24))
        .cast::<u8>())
        .wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                as isize
                * 3,
        ))
        .cast::<i8>())
        .wrapping_offset(2))
        .read()) as i32)
            == 0i32
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let mut rotation: u16 = 0u16;
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(
                    ((crate::c::div_i32(
                        ((r7) as i32).wrapping_mul(128i32),
                        ((((((((((&raw const sBounceRotateToSidesData)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((arrId) as i32) as isize * 24))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                                .read()) as i32) as isize
                                * 3,
                        ))
                        .cast::<i8>())
                        .wrapping_offset(2))
                        .read()) as i32),
                    )) as i16),
                    10i16,
                )) as i32)
                    .wrapping_neg()) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                (((crate::c::div_i32(
                    ((r10) as i32).wrapping_mul(((r7) as i32)),
                    ((((((((((&raw const sBounceRotateToSidesData)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((arrId) as i32) as isize * 24))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32) as isize
                            * 3,
                    ))
                    .cast::<i8>())
                    .wrapping_offset(2))
                    .read()) as i32),
                ))
                .wrapping_add(((r9) as i32))) as i16),
            );
            rotation = ((crate::c::div_i32(
                (((var) as i32)
                    .wrapping_mul(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                .wrapping_neg(),
                8i32,
            )) as u16);
            HandleSetAffineData(sprite, 256i16, 256i16, rotation);
            if ((r7) as i32)
                == ((((((((((&raw const sBounceRotateToSidesData)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((arrId) as i32) as isize * 24))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32) as isize
                        * 3,
                ))
                .cast::<i8>())
                .wrapping_offset(2))
                .read()) as i32)
            {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            } else {
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_BounceRotateToSides(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(6)
        .cast::<i16>())
        .write(4096i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read());
        BounceRotateToSides(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(BounceRotateToSides));
    }
}
pub(crate) unsafe extern "C" fn Anim_GlowOrange(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                    (((256i32).wrapping_add(
                        ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                            .wrapping_mul(16i32),
                    )) as i16),
                );
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 128i32
            {
                BlendPalette(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as u16),
                    16u16,
                    0u8,
                    735u16,
                );
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                    12i16,
                ));
                BlendPalette(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as u16),
                    16u16,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as u8),
                    735u16,
                );
            }
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_GlowRed(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                    (((256i32).wrapping_add(
                        ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                            .wrapping_mul(16i32),
                    )) as i16),
                );
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 128i32
            {
                BlendPalette(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as u16),
                    16u16,
                    0u8,
                    31u16,
                );
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                    12i16,
                ));
                BlendPalette(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as u16),
                    16u16,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as u8),
                    31u16,
                );
            }
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_GlowBlue(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                    (((256i32).wrapping_add(
                        ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                            .wrapping_mul(16i32),
                    )) as i16),
                );
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 128i32
            {
                BlendPalette(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as u16),
                    16u16,
                    0u8,
                    31744u16,
                );
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                    12i16,
                ));
                BlendPalette(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as u16),
                    16u16,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as u8),
                    31744u16,
                );
            }
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_GlowYellow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                    (((256i32).wrapping_add(
                        ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                            .wrapping_mul(16i32),
                    )) as i16),
                );
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 128i32
            {
                BlendPalette(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as u16),
                    16u16,
                    0u8,
                    1023u16,
                );
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                    12i16,
                ));
                BlendPalette(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as u16),
                    16u16,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as u8),
                    1023u16,
                );
            }
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_GlowPurple(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                    (((256i32).wrapping_add(
                        ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                            .wrapping_mul(16i32),
                    )) as i16),
                );
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 128i32
            {
                BlendPalette(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as u16),
                    16u16,
                    0u8,
                    24600u16,
                );
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                    12i16,
                ));
                BlendPalette(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as u16),
                    16u16,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as u8),
                    24600u16,
                );
            }
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_BackAndLunge(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        HandleStartAffineAnim(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(BackAndLunge_0));
    }
}
pub(crate) unsafe extern "C" fn BackAndLunge_0(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if (({
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 7i32
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(8i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(2i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(BackAndLunge_1));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn BackAndLunge_1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            )) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p2).write(((__p2).read()).wrapping_add(1));
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) <= 0i32 {
            let mut subResult: i16 = 0i16;
            let mut var: u8 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as u8);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            subResult = ((sprite).wrapping_add(36).cast::<i16>()).read();
            'l1: loop {
                'l2: {
                    subResult = ((((subResult) as i32).wrapping_sub(((var) as i32))) as i16);
                    let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    var = (var).wrapping_add(1);
                }
                if !(((subResult) as i32) > (-8i32)) {
                    break 'l1;
                }
            }
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(1i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(BackAndLunge_2));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn BackAndLunge_2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut rotation: u8 = 0u8;
        TryFlipX(sprite);
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            )) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p2).write(((__p2).read()).wrapping_add(1));
        rotation = ((crate::c::div_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                .wrapping_mul(6i32),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
        )) as u8);
        if (({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read());
        }
        HandleSetAffineData(
            sprite,
            256i16,
            256i16,
            ((((rotation) as i32).wrapping_mul(256i32)) as u16),
        );
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) < (-8i32) {
            ((sprite).wrapping_add(36).cast::<i16>()).write((-8i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(2i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                .write(((rotation) as i16));
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(BackAndLunge_3));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn BackAndLunge_3(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            > 11i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(2i32)) as i16));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                < 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            }
            HandleSetAffineData(
                sprite,
                256i16,
                256i16,
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    << 8) as u16),
            );
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == 0i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(BackAndLunge_4));
            }
        } else {
            let __p2 = (sprite).wrapping_add(36).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32),
                )) as i16),
            );
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p3).write((((((__p3).read()) as i32).wrapping_mul((-1i32))) as i16));
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p4).write(((__p4).read()).wrapping_add(1));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn BackAndLunge_4(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) > 0i32 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_BackFlip(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        HandleStartAffineAnim(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(BackFlip_0));
    }
}
pub(crate) unsafe extern "C" fn BackFlip_0(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_sub(1));
        if (crate::c::rem_i32(
            ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32),
            2i32,
        ) == 0i32)
            && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                <= 0i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(10i16);
        }
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) > 7i32 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(8i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write((-8i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(BackFlip_1));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn BackFlip_1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((Cos(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                16i16,
            )) as i32)
                .wrapping_sub(8i32)) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                16i16,
            )) as i32)
                .wrapping_sub(8i32)) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            > 63i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(160i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(10i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(BackFlip_2));
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            > 64i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(64i16);
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn BackFlip_2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32) > 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            let mut rotation: u32 = 0u32;
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((Cos(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                    5i16,
                )) as i32)
                    .wrapping_sub(4i32)) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                (((((Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                    5i16,
                )) as i32)
                    .wrapping_neg())
                .wrapping_add(4i32)) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write((((((__p2).read()) as i32).wrapping_sub(4i32)) as i16));
            rotation = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                as i32)
                .wrapping_sub(32i32)) as u32);
            HandleSetAffineData(
                sprite,
                256i16,
                256i16,
                (((rotation).wrapping_mul(512u32)) as u16),
            );
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                <= 32i32
            {
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                ResetSpriteAfterAnim(sprite);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            }
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_Flicker(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32) > 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    == 0i32
                {
                    1i32
                } else {
                    0i32
                }) as i16),
            );
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as u16)
                    as i32,
            );
            if (({
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                > 19i32
            {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            }
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(2i16);
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_BackFlipBig(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        HandleStartAffineAnim(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(BackFlipBig_0));
    }
}
pub(crate) unsafe extern "C" fn BackFlipBig_0(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) <= (-16i32) {
            ((sprite).wrapping_add(36).cast::<i16>()).write((-16i16));
            ((sprite).wrapping_add(38).cast::<i16>()).write(16i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(BackFlipBig_1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(160i16);
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn BackFlipBig_1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut rotation: u32 = 0u32;
        TryFlipX(sprite);
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(4i32)) as i16));
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
            22i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                22i16,
            )) as i32)
                .wrapping_neg()) as i16),
        );
        rotation = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            .wrapping_sub(32i32)) as u32);
        HandleSetAffineData(
            sprite,
            256i16,
            256i16,
            (((rotation).wrapping_mul(512u32)) as u16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            <= 32i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(BackFlipBig_2));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn BackFlipBig_2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) <= 0i32 {
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_FrontFlip(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        HandleStartAffineAnim(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(FrontFlip_0));
    }
}
pub(crate) unsafe extern "C" fn FrontFlip_0(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_sub(1));
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) > 15i32 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(FrontFlip_1));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn FrontFlip_1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(16i32)) as i16));
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) <= (-16i32) {
            ((sprite).wrapping_add(36).cast::<i16>()).write((-16i16));
            ((sprite).wrapping_add(38).cast::<i16>()).write(16i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(FrontFlip_2));
        } else {
            let __p2 = (sprite).wrapping_add(36).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_sub(2i32)) as i16));
            let __p3 = (sprite).wrapping_add(38).cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as i16));
        }
        HandleSetAffineData(
            sprite,
            256i16,
            256i16,
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                << 8) as u16),
        );
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn FrontFlip_2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_sub(1));
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) >= 0i32 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_TumblingFrontFlip(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(2)
        .cast::<i16>())
        .write(2i16);
        TumblingFrontFlip(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TumblingFrontFlip));
    }
}
pub(crate) unsafe extern "C" fn TumblingFrontFlip(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
        ))
        .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            let __p1 = ((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
            ))
            .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            TryFlipX(sprite);
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == 0i32
            {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p2).write(((__p2).read()).wrapping_add(1));
                HandleStartAffineAnim(sprite);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                    (((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
                    ))
                    .wrapping_add(2)
                    .cast::<i16>())
                    .read(),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write((-1i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write((-1i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            }
            let __p3 = (sprite).wrapping_add(36).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_mul(2i32))
                    .wrapping_mul(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    ),
                )) as i16),
            );
            let __p4 = (sprite).wrapping_add(38).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_mul(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                                .read()) as i32),
                        ),
                )) as i16),
            );
            let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p5).write((((((__p5).read()) as i32).wrapping_add(8i32)) as i16));
            if (((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) <= (-16i32))
                || (((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) >= 16i32)
            {
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        .wrapping_mul(16i32)) as i16),
                );
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p6).write((((((__p6).read()) as i32).wrapping_mul((-1i32))) as i16));
                let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p7).write(((__p7).read()).wrapping_add(1));
            } else {
                if (((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) <= (-16i32))
                    || (((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) >= 16i32)
                {
                    ((sprite).wrapping_add(38).cast::<i16>()).write(((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32))).wrapping_mul(16i32)) as i16));
                    let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                    (__p8).write((((((__p8).read()) as i32).wrapping_mul((-1i32))) as i16));
                    let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
            }
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                > 5i32)
                && (((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) <= 0i32)
            {
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                if (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(4)
                .cast::<i16>())
                .read()) as i32)
                    > 1i32
                {
                    let __p10 = ((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
                    ))
                    .wrapping_add(4)
                    .cast::<i16>();
                    (__p10).write(((__p10).read()).wrapping_sub(1));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                    (((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
                    ))
                    .cast::<u16>())
                    .write(10u16);
                } else {
                    ResetSpriteAfterAnim(sprite);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(WaitAnimEnd));
                }
            }
            HandleSetAffineData(
                sprite,
                256i16,
                256i16,
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32)
                    << 8) as u16),
            );
            TryFlipX(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_Figure8(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        HandleStartAffineAnim(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(Figure8));
    }
}
pub(crate) unsafe extern "C" fn Figure8(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
                16i16,
            )) as i32)
                .wrapping_neg()) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((Sin(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32)
                    .wrapping_mul(2i32)
                    & 255i32) as i16),
                8i16,
            )) as i32)
                .wrapping_neg()) as i16),
        );
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
            > 192i32)
            && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                == 1i32)
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p2).write(((__p2).read()).wrapping_add(1));
        } else {
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                > 64i32)
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    == 0i32)
            {
                HandleSetAffineData(sprite, (-256i16), 256i16, 0u16);
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
            > 255i32
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_FlashYellow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 1i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                (((256i32).wrapping_add(
                    ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                        .wrapping_mul(16i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        }
        if ((((((((&raw const sYellowFlashData).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    as isize
                    * 2,
            ))
        .cast::<u8>())
        .wrapping_offset(1))
        .read()) as i32)
            == 255i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                == 1i32
            {
                if ((((((&raw const sYellowFlashData).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize
                            * 2,
                    ))
                .cast::<u8>())
                .read())
                    != 0
                {
                    BlendPalette(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as u16),
                        16u16,
                        16u8,
                        1023u16,
                    );
                } else {
                    BlendPalette(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as u16),
                        16u16,
                        0u8,
                        1023u16,
                    );
                }
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            }
            if ((((((((&raw const sYellowFlashData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize
                        * 2,
                ))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                (__p3).write(((__p3).read()).wrapping_add(1));
            } else {
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p4).write(((__p4).read()).wrapping_add(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SwingConcave(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
        }
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(8)
            .cast::<i16>())
            .read()) as i32)
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            if (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(4)
            .cast::<i16>())
            .read()) as i32)
                > 1i32
            {
                let __p1 = ((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(4)
                .cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_sub(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            } else {
                ResetSpriteAfterAnim(sprite);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            }
        } else {
            let mut index: i16 = ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_mul(256i32),
                (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(8)
                .cast::<i16>())
                .read()) as i32),
            )) as i16);
            ((sprite).wrapping_add(36).cast::<i16>())
                .write(((((Sin(index, 10i16)) as i32).wrapping_neg()) as i16));
            HandleSetAffineData(sprite, 256i16, 256i16, ((Sin(index, 3276i16)) as u16));
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write(((__p2).read()).wrapping_add(1));
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_SwingConcave_FastShort(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(50i16);
        SwingConcave(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SwingConcave));
    }
}
pub(crate) unsafe extern "C" fn SwingConvex(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
        }
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(8)
            .cast::<i16>())
            .read()) as i32)
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            if (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(4)
            .cast::<i16>())
            .read()) as i32)
                > 1i32
            {
                let __p1 = ((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(4)
                .cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_sub(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            } else {
                ResetSpriteAfterAnim(sprite);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            }
        } else {
            let mut index: i16 = ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_mul(256i32),
                (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(8)
                .cast::<i16>())
                .read()) as i32),
            )) as i16);
            ((sprite).wrapping_add(36).cast::<i16>())
                .write(((((Sin(index, 10i16)) as i32).wrapping_neg()) as i16));
            HandleSetAffineData(
                sprite,
                256i16,
                256i16,
                ((((Sin(index, 3276i16)) as i32).wrapping_neg()) as u16),
            );
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write(((__p2).read()).wrapping_add(1));
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_SwingConvex_FastShort(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(50i16);
        SwingConvex(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SwingConvex));
    }
}
pub(crate) unsafe extern "C" fn Anim_RotateUpSlamDown(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        HandleStartAffineAnim(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
            (((crate::c::div_i32(
                (14i32).wrapping_mul(((((sprite).wrapping_add(40).cast::<i8>()).read()) as i32)),
                10i32,
            ))
            .wrapping_neg()) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(128i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RotateUpSlamDown_0));
    }
}
pub(crate) unsafe extern "C" fn RotateUpSlamDown_0(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p1).write(((__p1).read()).wrapping_sub(1));
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                .wrapping_add(
                    ((Cos(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
                    )) as i32),
                )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
            )) as i32)
                .wrapping_neg()) as i16),
        );
        HandleSetAffineData(
            sprite,
            256i16,
            256i16,
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                .wrapping_sub(128i32)
                << 8) as u16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            <= 120i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(120i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(RotateUpSlamDown_1));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn RotateUpSlamDown_1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            == 20i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(RotateUpSlamDown_2));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn RotateUpSlamDown_2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                .wrapping_add(
                    ((Cos(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
                    )) as i32),
                )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
            )) as i32)
                .wrapping_neg()) as i16),
        );
        HandleSetAffineData(
            sprite,
            256i16,
            256i16,
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                .wrapping_sub(128i32)
                << 8) as u16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            >= 128i32
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(Anim_VerticalShake));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn DeepVerticalSquishBounce(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
        ))
        .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            let __p1 = ((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
            ))
            .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == 0i32
            {
                HandleStartAffineAnim(sprite);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                == 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                    256i16,
                ));
                ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                    16i16,
                ));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                    32i16,
                ));
                HandleSetAffineData(
                    sprite,
                    (((256i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                    )) as i16),
                    (((256i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32),
                    )) as i16),
                    0u16,
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    == 128i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(1i16);
                }
            } else {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    == 1i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                        32i16,
                    ));
                    ((sprite).wrapping_add(38).cast::<i16>()).write(
                        ((((Sin(
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                            8i16,
                        )) as i32)
                            .wrapping_neg()) as i16),
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                        128i16,
                    ));
                    HandleSetAffineData(
                        sprite,
                        (((256i32).wrapping_add(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                                .read()) as i32),
                        )) as i16),
                        (((256i32).wrapping_sub(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as i32),
                        )) as i16),
                        0u16,
                    );
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        == 128i32
                    {
                        if (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize
                                * 12,
                        ))
                        .wrapping_add(4)
                        .cast::<i16>())
                        .read()) as i32)
                            > 1i32
                        {
                            let __p2 = ((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(
                                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
                                        as isize
                                        * 12,
                                ))
                            .wrapping_add(4)
                            .cast::<i16>();
                            (__p2).write(((__p2).read()).wrapping_sub(1));
                            (((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
                                    as isize
                                    * 12,
                            ))
                            .cast::<u16>())
                            .write(10u16);
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                                .write(0i16);
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .write(0i16);
                        } else {
                            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
                            ResetSpriteAfterAnim(sprite);
                            ((sprite)
                                .wrapping_add(28)
                                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                            .write(Some(WaitAnimEnd));
                        }
                    }
                }
            }
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
                    ))
                    .wrapping_add(6)
                    .cast::<i16>())
                    .read()) as i32),
                )) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_DeepVerticalSquishBounce(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(6)
        .cast::<i16>())
        .write(4i16);
        DeepVerticalSquishBounce(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(DeepVerticalSquishBounce));
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalJumps(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut counter: i32 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32);
        TryFlipX(sprite);
        if counter > 512i32 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        } else {
            'l1: {
                let __sw1 = crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                    128i32,
                );
                if __sw1 == 0i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        ((crate::c::div_i32(
                            ((crate::c::rem_i32(counter, 128i32)).wrapping_mul(8i32))
                                .wrapping_neg(),
                            128i32,
                        )) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        (((crate::c::div_i32(crate::c::rem_i32(counter, 128i32), 16i32))
                            .wrapping_sub(8i32)) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        ((crate::c::div_i32(crate::c::rem_i32(counter, 128i32), 16i32)) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        (((crate::c::div_i32(
                            ((crate::c::rem_i32(counter, 128i32)).wrapping_mul(8i32))
                                .wrapping_neg(),
                            128i32,
                        ))
                        .wrapping_add(8i32)) as i16),
                    );
                    break 'l1;
                }
            }
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(((crate::c::rem_i32(counter, 128i32)) as i16), 8i16)) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(12i32)) as i16));
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalJumpsVerticalStretch(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .write((-1i16));
        HandleStartAffineAnim(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        HorizontalJumpsVerticalStretch_0(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(HorizontalJumpsVerticalStretch_0));
    }
}
pub(crate) unsafe extern "C" fn HorizontalJumpsVerticalStretch_0(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
        ))
        .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            let __p1 = ((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
            ))
            .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            let mut counter: i32 = 0i32;
            TryFlipX(sprite);
            counter =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32);
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 128i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(HorizontalJumpsVerticalStretch_1));
            } else {
                let mut var: i32 = (8i32).wrapping_mul(
                    (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
                    ))
                    .wrapping_add(8)
                    .cast::<i16>())
                    .read()) as i32),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    ((crate::c::div_i32(
                        (var).wrapping_mul(crate::c::rem_i32(counter, 128i32)),
                        128i32,
                    )) as i16),
                );
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    ((((Sin(((crate::c::rem_i32(counter, 128i32)) as i16), 8i16)) as i32)
                        .wrapping_neg()) as i16),
                );
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p2).write((((((__p2).read()) as i32).wrapping_add(12i32)) as i16));
            }
            TryFlipX(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn HorizontalJumpsVerticalStretch_1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 48i32
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(HorizontalJumpsVerticalStretch_2));
        } else {
            let mut yDelta: i16 = 0i16;
            let mut yScale: i16 = ((((Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                64i16,
            )) as i32)
                .wrapping_add(256i32)) as i16);
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                >= 16i32)
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    <= 31i32)
            {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
                let __p2 = (sprite).wrapping_add(36).cast::<i16>();
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_sub(
                        (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize
                                * 12,
                        ))
                        .wrapping_add(8)
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
                );
            }
            yDelta = 0i16;
            if ((yScale) as i32) > 256i32 {
                yDelta =
                    ((crate::c::div_i32((256i32).wrapping_sub(((yScale) as i32)), 8i32)) as i16);
            }
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                (((((Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read(),
                    20i16,
                )) as i32)
                    .wrapping_neg())
                .wrapping_sub(((yDelta) as i32))) as i16),
            );
            HandleSetAffineData(
                sprite,
                (((256i32).wrapping_sub(
                    ((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                        32i16,
                    )) as i32),
                )) as i16),
                yScale,
                0u16,
            );
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p3).write(((__p3).read()).wrapping_add(1));
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p4).write((((((__p4).read()) as i32).wrapping_add(8i32)) as i16));
            let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p5).write((((((__p5).read()) as i32) & 255i32) as i16));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn HorizontalJumpsVerticalStretch_2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut counter: i32 = 0i32;
        TryFlipX(sprite);
        counter =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32);
        if counter > 128i32 {
            if (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(4)
            .cast::<i16>())
            .read()) as i32)
                > 1i32
            {
                let __p1 = ((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(4)
                .cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_sub(1));
                (((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
                ))
                .cast::<u16>())
                .write(10u16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(HorizontalJumpsVerticalStretch_0));
            } else {
                ResetSpriteAfterAnim(sprite);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            }
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        } else {
            let mut var: i32 = (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
                .wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
                ))
            .wrapping_add(8)
            .cast::<i16>())
            .read()) as i32);
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                (((crate::c::div_i32(
                    (var).wrapping_mul((crate::c::rem_i32(counter, 128i32)).wrapping_mul(8i32)),
                    128i32,
                ))
                .wrapping_add((8i32).wrapping_mul((var).wrapping_neg()))) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(((crate::c::rem_i32(counter, 128i32)) as i16), 8i16)) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(12i32)) as i16));
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn RotateToSides(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 254i32
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            if (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(4)
            .cast::<i16>())
            .read()) as i32)
                > 1i32
            {
                let __p2 = ((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
                ))
                .wrapping_add(4)
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_sub(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            } else {
                ResetSpriteAfterAnim(sprite);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            }
            TryFlipX(sprite);
        } else {
            let mut rotation: u16 = 0u16;
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                    16i16,
                )) as i32)
                    .wrapping_neg()) as i16),
            );
            rotation = ((Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                32i16,
            )) as u16);
            HandleSetAffineData(sprite, 256i16, 256i16, ((((rotation) as i32) << 8) as u16));
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    (((((((&raw mut sAnims).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 12,
                    ))
                    .wrapping_add(6)
                    .cast::<i16>())
                    .read()) as i32),
                )) as i16),
            );
            TryFlipX(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_RotateToSides_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(6)
        .cast::<i16>())
        .write(4i16);
        RotateToSides(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RotateToSides));
    }
}
pub(crate) unsafe extern "C" fn Anim_RotateUpToSides(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 254i32
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            TryFlipX(sprite);
        } else {
            let mut rotation: u16 = 0u16;
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                    16i16,
                )) as i32)
                    .wrapping_neg()) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(
                    ((crate::c::rem_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32),
                        128i32,
                    )) as i16),
                    16i16,
                )) as i32)
                    .wrapping_neg()) as i16),
            );
            rotation = ((Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                32i16,
            )) as u16);
            HandleSetAffineData(sprite, 256i16, 256i16, ((((rotation) as i32) << 8) as u16));
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
            TryFlipX(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_FlickerIncreasing(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        } else {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p2).write(((__p2).read()).wrapping_add(1));
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 10i32
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_TipHopForward(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        HandleStartAffineAnim(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TipHopForward_0));
    }
}
pub(crate) unsafe extern "C" fn TipHopForward_0(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 31i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(32i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(TipHopForward_1));
        } else {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        }
        HandleSetAffineData(
            sprite,
            256i16,
            256i16,
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                << 8) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn TipHopForward_1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 512i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(TipHopForward_2));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((crate::c::div_i32(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        .wrapping_mul(16i32))
                    .wrapping_neg(),
                    512i32,
                )) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(
                    ((crate::c::rem_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                        128i32,
                    )) as i16),
                    4i16,
                )) as i32)
                    .wrapping_neg()) as i16),
            );
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(12i32)) as i16));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn TipHopForward_2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(2i32)) as i16));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32) < 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((Sin(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_mul(2i32)) as i16),
                    16i16,
                )) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        HandleSetAffineData(
            sprite,
            256i16,
            256i16,
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                << 8) as u16),
        );
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_PivotShake(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut rotation: u16 = 0u16;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 255i32
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(16i32)) as i16));
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((Sin(
                    ((crate::c::rem_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32),
                        128i32,
                    )) as i16),
                    8i16,
                )) as i32)
                    .wrapping_neg()) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(
                    ((crate::c::rem_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32),
                        128i32,
                    )) as i16),
                    8i16,
                )) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        rotation = ((Sin(
            ((crate::c::rem_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
                128i32,
            )) as i16),
            16i16,
        )) as u16);
        HandleSetAffineData(sprite, 256i16, 256i16, ((((rotation) as i32) << 8) as u16));
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_TipAndShake(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        HandleStartAffineAnim(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TipAndShake_0));
    }
}
pub(crate) unsafe extern "C" fn TipAndShake_0(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 24i32
        {
            if (({
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                > 4i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(TipAndShake_1));
            }
        } else {
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as i16));
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                8i16,
            ));
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                    8i16,
                )) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        HandleSetAffineData(
            sprite,
            256i16,
            256i16,
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                .wrapping_neg()
                << 8) as u16),
        );
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn TipAndShake_1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 32i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(1i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(TipAndShake_2));
        } else {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                8i16,
            ));
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                    8i16,
                )) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        HandleSetAffineData(
            sprite,
            256i16,
            256i16,
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                .wrapping_neg()
                << 8) as u16),
        );
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn TipAndShake_2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    .wrapping_mul(4i32),
            )) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32) > 9i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(32i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(TipAndShake_3));
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
            8i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                8i16,
            )) as i32)
                .wrapping_neg()) as i16),
        );
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            <= 28i32)
            || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                >= 36i32)
        {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p2).write((((((__p2).read()) as i32).wrapping_mul((-1i32))) as i16));
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        HandleSetAffineData(
            sprite,
            256i16,
            256i16,
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                .wrapping_neg()
                << 8) as u16),
        );
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn TipAndShake_3(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            <= 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(2i32)) as i16));
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                8i16,
            ));
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                    8i16,
                )) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        HandleSetAffineData(
            sprite,
            256i16,
            256i16,
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                .wrapping_neg()
                << 8) as u16),
        );
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_VibrateToCorners(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 40i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        } else {
            let mut sign: i8 = 0i8;
            if !((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                as i32)
                & 1i32)
                != 0)
            {
                sign = 1i8;
            } else {
                sign = (-1i8);
            }
            if crate::c::div_i32(
                crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                    4i32,
                ),
                2i32,
            ) == 0i32
            {
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    ((((Sin(
                        ((crate::c::rem_i32(
                            crate::c::div_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32)
                                    .wrapping_mul(128i32),
                                40i32,
                            ),
                            256i32,
                        )) as i16),
                        16i16,
                    )) as i32)
                        .wrapping_mul(((sign) as i32))) as i16),
                );
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    ((((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32).wrapping_neg())
                        as i16),
                );
            } else {
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    (((((Sin(
                        ((crate::c::rem_i32(
                            crate::c::div_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32)
                                    .wrapping_mul(128i32),
                                40i32,
                            ),
                            256i32,
                        )) as i16),
                        16i16,
                    )) as i32)
                        .wrapping_neg())
                    .wrapping_mul(((sign) as i32))) as i16),
                );
                ((sprite).wrapping_add(38).cast::<i16>())
                    .write(((sprite).wrapping_add(36).cast::<i16>()).read());
            }
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_GrowInStages(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32) > 0i32
        {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p2).write(((__p2).read()).wrapping_sub(1));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                != 3i32
            {
                let mut scale: i16 = ((crate::c::div_i32(
                    (8i32).wrapping_mul(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                    ),
                    20i32,
                )) as i16);
                scale = Sin(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_sub(((scale) as i32))) as i16),
                    64i16,
                );
                HandleSetAffineData(
                    sprite,
                    (((256i32).wrapping_sub(((scale) as i32))) as i16),
                    (((256i32).wrapping_sub(((scale) as i32))) as i16),
                    0u16,
                );
            }
        } else {
            let mut var: i16 = 0i16;
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                == 3i32
            {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    > 63i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(64i16);
                    HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
                    ResetSpriteAfterAnim(sprite);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(WaitAnimEnd));
                }
                var = Cos(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                    64i16,
                );
            } else {
                var = Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                    64i16,
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    > 63i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(3i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(10i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
                } else {
                    if (((var) as i32) > 48i32)
                        && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                            .read()) as i32)
                            == 1i32)
                    {
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                            .write(2i16);
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                            .write(20i16);
                    } else {
                        if (((var) as i32) > 16i32)
                            && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .read()) as i32)
                                == 0i32)
                        {
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .write(1i16);
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                                .write(20i16);
                        }
                    }
                }
            }
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as i16));
            HandleSetAffineData(
                sprite,
                (((256i32).wrapping_sub(((var) as i32))) as i16),
                (((256i32).wrapping_sub(((var) as i32))) as i16),
                0u16,
            );
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalSpring(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 512i32
        {
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let mut yScale: i16 = 0i16;
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                ((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32),
                    256i32,
                )) as i16),
                8i16,
            ));
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
            yScale = Sin(
                ((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32),
                    128i32,
                )) as i16),
                96i16,
            );
            HandleSetAffineData(
                sprite,
                256i16,
                ((((yScale) as i32).wrapping_add(256i32)) as i16),
                0u16,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalRepeatedSpring(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 256i32
        {
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let mut yScale: i16 = 0i16;
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                16i16,
            ));
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(4i32)) as i16));
            yScale = Sin(
                (((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32),
                    64i32,
                ))
                .wrapping_mul(2i32)) as i16),
                128i16,
            );
            HandleSetAffineData(
                sprite,
                256i16,
                ((((yScale) as i32).wrapping_add(256i32)) as i16),
                0u16,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_SpringRising(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        HandleStartAffineAnim(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpringRising_0));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn SpringRising_0(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut yScale: i16 = 0i16;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 63i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpringRising_1));
            yScale = Sin(64i16, 128i16);
        } else {
            yScale = Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                128i16,
            );
        }
        HandleSetAffineData(
            sprite,
            256i16,
            (((256i32).wrapping_add(((yScale) as i32))) as i16),
            0u16,
        );
    }
}
pub(crate) unsafe extern "C" fn SpringRising_1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut yScale: i16 = 0i16;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 95i32
        {
            yScale = Cos(0i16, 128i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p2).write(((__p2).read()).wrapping_add(1));
        } else {
            let mut sign: i16 = 0i16;
            let mut index: i16 = 0i16;
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32)
                    .wrapping_mul(4i32))
                .wrapping_neg())
                .wrapping_sub(
                    ((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                        8i16,
                    )) as i32),
                )) as i16),
            );
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                > 63i32
            {
                sign = (-1i16);
                index =
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_sub(64i32)) as i16);
            } else {
                sign = 1i16;
                index = 0i16;
            }
            yScale = ((((Cos(
                (((((index) as i32).wrapping_mul(2i32)).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32),
                )) as i16),
                128i16,
            )) as i32)
                .wrapping_mul(((sign) as i32))) as i16);
        }
        HandleSetAffineData(
            sprite,
            256i16,
            (((256i32).wrapping_add(((yScale) as i32))) as i16),
            0u16,
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
            == 3i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpringRising_2));
        }
    }
}
pub(crate) unsafe extern "C" fn SpringRising_2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut yScale: i16 = 0i16;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        yScale = Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
            128i16,
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((Cos(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                12i16,
            )) as i32)
                .wrapping_neg()) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 63i32
        {
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
        }
        HandleSetAffineData(
            sprite,
            256i16,
            (((256i32).wrapping_add(((yScale) as i32))) as i16),
            0u16,
        );
    }
}
pub(crate) unsafe extern "C" fn HorizontalSpring(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
        } else {
            let mut xScale: i16 = 0i16;
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                ((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32),
                    256i32,
                )) as i16),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
            ));
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32),
                )) as i16),
            );
            xScale = Sin(
                ((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32),
                    128i32,
                )) as i16),
                96i16,
            );
            HandleSetAffineData(
                sprite,
                (((256i32).wrapping_add(((xScale) as i32))) as i16),
                256i16,
                0u16,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalSpring(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(8i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(512i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(8i16);
        }
        HorizontalSpring(sprite);
    }
}
pub(crate) unsafe extern "C" fn HorizontalRepeatedSpring(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
        } else {
            let mut xScale: i16 = 0i16;
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                ((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32),
                    256i32,
                )) as i16),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
            ));
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32),
                )) as i16),
            );
            xScale = Sin(
                (((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32),
                    64i32,
                ))
                .wrapping_mul(2i32)) as i16),
                128i16,
            );
            HandleSetAffineData(
                sprite,
                (((256i32).wrapping_add(((xScale) as i32))) as i16),
                256i16,
                0u16,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalRepeatedSpring_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(4i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(256i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(16i16);
        }
        HorizontalRepeatedSpring(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalSlideShrink(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 512i32
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let mut scale: i16 = 0i16;
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                ((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32),
                    256i32,
                )) as i16),
                8i16,
            ));
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
            scale = Sin(
                ((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32),
                    128i32,
                )) as i16),
                96i16,
            );
            HandleSetAffineData(
                sprite,
                (((256i32).wrapping_add(((scale) as i32))) as i16),
                (((256i32).wrapping_add(((scale) as i32))) as i16),
                0u16,
            );
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_LungeGrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 512i32
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let mut scale: i16 = 0i16;
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((Sin(
                    ((crate::c::div_i32(
                        crate::c::rem_i32(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as i32),
                            256i32,
                        ),
                        2i32,
                    )) as i16),
                    16i16,
                )) as i32)
                    .wrapping_neg()) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
            scale = ((((Sin(
                ((crate::c::div_i32(
                    crate::c::rem_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32),
                        256i32,
                    ),
                    2i32,
                )) as i16),
                64i16,
            )) as i32)
                .wrapping_neg()) as i16);
            HandleSetAffineData(
                sprite,
                (((256i32).wrapping_add(((scale) as i32))) as i16),
                (((256i32).wrapping_add(((scale) as i32))) as i16),
                0u16,
            );
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_CircleIntoBackground(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 512i32
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let mut scale: i16 = 0i16;
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((Sin(
                    ((crate::c::rem_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32),
                        256i32,
                    )) as i16),
                    8i16,
                )) as i32)
                    .wrapping_neg()) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
            scale = Sin(
                ((crate::c::div_i32(
                    crate::c::rem_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32),
                        256i32,
                    ),
                    2i32,
                )) as i16),
                96i16,
            );
            HandleSetAffineData(
                sprite,
                (((256i32).wrapping_add(((scale) as i32))) as i16),
                (((256i32).wrapping_add(((scale) as i32))) as i16),
                0u16,
            );
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_RapidHorizontalHops(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 2048i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        } else {
            let mut caseVar: i16 = ((crate::c::rem_i32(
                crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                    512i32,
                ),
                4i32,
            )) as i16);
            'l1: {
                let __sw1 = ((caseVar) as i32);
                if __sw1 == 0i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        ((crate::c::div_i32(
                            ((crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ))
                            .wrapping_mul(16i32))
                            .wrapping_neg(),
                            512i32,
                        )) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        (((crate::c::div_i32(
                            crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ),
                            32i32,
                        ))
                        .wrapping_sub(16i32)) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        ((crate::c::div_i32(
                            crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ),
                            32i32,
                        )) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        (((crate::c::div_i32(
                            ((crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ))
                            .wrapping_mul(16i32))
                            .wrapping_neg(),
                            512i32,
                        ))
                        .wrapping_add(16i32)) as i16),
                    );
                    break 'l1;
                }
            }
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(
                    ((crate::c::rem_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                        128i32,
                    )) as i16),
                    4i16,
                )) as i32)
                    .wrapping_neg()) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(24i32)) as i16));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_FourPetal(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(64i16);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
            == 4i32
        {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                > 63i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                > 127i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                (__p4).write(((__p4).read()).wrapping_add(1));
            }
        }
        'l1: {
            let __sw5 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32);
            let __matched =
                __sw5 == 1i32 || __sw5 == 2i32 || __sw5 == 3i32 || __sw5 == 0i32 || __sw5 == 4i32;
            if __sw5 == 1i32 {
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    ((((Cos(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                        8i16,
                    )) as i32)
                        .wrapping_neg()) as i16),
                );
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    ((((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                        8i16,
                    )) as i32)
                        .wrapping_sub(8i32)) as i16),
                );
                break 'l1;
            }
            if __sw5 == 2i32 {
                ((sprite).wrapping_add(36).cast::<i16>()).write(((((((Sin(((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32))).wrapping_add(128i32)) as i16), 8i16)) as i32))).wrapping_add(8i32)) as i16));
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    ((((Cos(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                        8i16,
                    )) as i32)
                        .wrapping_neg()) as i16),
                );
                break 'l1;
            }
            if __sw5 == 3i32 {
                ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                    8i16,
                ));
                ((sprite).wrapping_add(38).cast::<i16>()).write(((((((Sin(((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32))).wrapping_add(128i32)) as i16), 8i16)) as i32))).wrapping_add(8i32)) as i16));
                break 'l1;
            }
            if __sw5 == 0i32 || __sw5 == 4i32 {
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    ((((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                        8i16,
                    )) as i32)
                        .wrapping_sub(8i32)) as i16),
                );
                ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                    8i16,
                ));
                break 'l1;
            }
            if !__matched {
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
                break 'l1;
            }
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalSquishBounce_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(32i16);
        VerticalSquishBounce(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalSquishBounce));
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalSlide_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(80i16);
        HorizontalSlide(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(HorizontalSlide));
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalSlide_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(80i16);
        VerticalSlide(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalSlide));
    }
}
pub(crate) unsafe extern "C" fn Anim_BounceRotateToSides_Small(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(6)
        .cast::<i16>())
        .write(2048i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read());
        BounceRotateToSides(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(BounceRotateToSides));
    }
}
pub(crate) unsafe extern "C" fn Anim_BounceRotateToSides_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(1i16);
        Anim_BounceRotateToSides(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_BounceRotateToSides_SmallSlow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(1i16);
        Anim_BounceRotateToSides_Small(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_ZigzagSlow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) <= 0i32 {
            Zigzag(sprite);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
        } else {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalShake_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(30i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(3i16);
        HorizontalShake(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(HorizontalShake));
    }
}
pub(crate) unsafe extern "C" fn Anim_VertialShake_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(30i16);
        VerticalShake(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalShake));
    }
}
pub(crate) unsafe extern "C" fn Anim_Twist_Twice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(6)
        .cast::<i16>())
        .write(1024i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .cast::<u16>())
        .write(0u16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(4)
        .cast::<i16>())
        .write(2i16);
        Twist(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(Twist));
    }
}
pub(crate) unsafe extern "C" fn Anim_CircleCounterclockwise_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(6)
        .cast::<i16>())
        .write(512i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(3i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(2)
        .cast::<i16>())
        .write(12i16);
        CircleCounterclockwise(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(CircleCounterclockwise));
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalShakeTwice_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(24i16);
        VerticalShakeTwice(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalShakeTwice));
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalSlideWobble_Small(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(5i16);
        VerticalSlideWobble(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalSlideWobble));
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalJumps_Small(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(3i16);
        VerticalJumps(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalJumps));
    }
}
pub(crate) unsafe extern "C" fn Anim_Spin(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .cast::<u16>())
        .write(60u16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(30i16);
        Spin(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(Spin));
    }
}
pub(crate) unsafe extern "C" fn Anim_TumblingFrontFlip_Twice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(2)
        .cast::<i16>())
        .write(1i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(4)
        .cast::<i16>())
        .write(2i16);
        TumblingFrontFlip(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TumblingFrontFlip));
    }
}
pub(crate) unsafe extern "C" fn Anim_DeepVerticalSquishBounce_Twice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(6)
        .cast::<i16>())
        .write(4i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(4)
        .cast::<i16>())
        .write(2i16);
        DeepVerticalSquishBounce(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(DeepVerticalSquishBounce));
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalJumpsVerticalStretch_Twice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(1i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(4)
        .cast::<i16>())
        .write(2i16);
        HandleStartAffineAnim(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        HorizontalJumpsVerticalStretch_0(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(HorizontalJumpsVerticalStretch_0));
    }
}
pub(crate) unsafe extern "C" fn Anim_RotateToSides(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(6)
        .cast::<i16>())
        .write(2i16);
        RotateToSides(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RotateToSides));
    }
}
pub(crate) unsafe extern "C" fn Anim_RotateToSides_Twice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(6)
        .cast::<i16>())
        .write(4i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(4)
        .cast::<i16>())
        .write(2i16);
        RotateToSides(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RotateToSides));
    }
}
pub(crate) unsafe extern "C" fn Anim_SwingConcave(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(100i16);
        SwingConcave(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SwingConcave));
    }
}
pub(crate) unsafe extern "C" fn Anim_SwingConcave_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(50i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(4)
        .cast::<i16>())
        .write(2i16);
        SwingConcave(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SwingConcave));
    }
}
pub(crate) unsafe extern "C" fn Anim_SwingConvex(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(100i16);
        SwingConvex(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SwingConvex));
    }
}
pub(crate) unsafe extern "C" fn Anim_SwingConvex_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(50i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(4)
        .cast::<i16>())
        .write(2i16);
        SwingConvex(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SwingConvex));
    }
}
pub(crate) unsafe extern "C" fn VerticalShakeBack(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut counter: i32 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32);
        if counter > 2304i32 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        } else {
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(
                    ((crate::c::rem_i32((counter).wrapping_add(192i32), 256i32)) as i16),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                )) as i32)
                    .wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32),
                    )) as i16),
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalShakeBack(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(60i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(3i16);
        VerticalShakeBack(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalShakeBack));
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalShakeBack_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(30i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(3i16);
        VerticalShakeBack(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalShakeBack));
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalShakeHorizontalSlide_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 2048i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        } else {
            let mut divCase: i16 = ((crate::c::rem_i32(
                crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                    512i32,
                ),
                4i32,
            )) as i16);
            'l1: {
                let __sw1 = ((divCase) as i32);
                if __sw1 == 0i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        ((crate::c::div_i32(
                            crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ),
                            32i32,
                        )) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        ((crate::c::div_i32(
                            ((crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ))
                            .wrapping_mul(16i32))
                            .wrapping_neg(),
                            512i32,
                        )) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        (((crate::c::div_i32(
                            ((crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ))
                            .wrapping_mul(16i32))
                            .wrapping_neg(),
                            512i32,
                        ))
                        .wrapping_add(16i32)) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        (((crate::c::div_i32(
                            crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ),
                            32i32,
                        ))
                        .wrapping_sub(16i32)) as i16),
                    );
                    break 'l1;
                }
            }
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                ((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                    128i32,
                )) as i16),
                4i16,
            ));
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(24i32)) as i16));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn VerticalStretchBothEnds(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut index1: i16 = 0i16;
        let mut index2: i16 = 0i16;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
        {
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                <= 1i32
            {
                ResetSpriteAfterAnim(sprite);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            } else {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p1).write(((__p1).read()).wrapping_sub(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            }
        } else {
            let mut amplitude: u8 = 0u8;
            let mut cmpVal1: u8 = 0u8;
            let mut cmpVal2: u8 = 0u8;
            let mut xScale: i16 = 0i16;
            let mut yScale: i16 = 0i16;
            index2 = ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    .wrapping_mul(128i32),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16);
            cmpVal1 = ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
                4i32,
            )) as u8);
            cmpVal2 = ((((cmpVal1) as i32).wrapping_mul(3i32)) as u8);
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                >= ((cmpVal1) as i32))
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    < ((cmpVal2) as i32))
            {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                (__p2).write((((((__p2).read()) as i32).wrapping_add(51i32)) as i16));
                index1 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                    .read()) as i32)
                    & 255i32) as i16);
            }
            if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0) {
                xScale = (((-256i32).wrapping_sub(((Sin(index2, 16i16)) as i32))) as i16);
            } else {
                xScale = (((256i32).wrapping_add(((Sin(index2, 16i16)) as i32))) as i16);
            }
            amplitude =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u8);
            yScale = ((((256i32).wrapping_sub(((Sin(index2, ((amplitude) as i16))) as i32)))
                .wrapping_sub(
                    ((Sin(
                        index1,
                        ((crate::c::div_i32(((amplitude) as i32), 5i32)) as i16),
                    )) as i32),
                )) as i16);
            SetAffineData(sprite, xScale, yScale, 0u16);
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalStretchBothEnds_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(40i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(40i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        VerticalStretchBothEnds(sprite);
    }
}
pub(crate) unsafe extern "C" fn HorizontalStretchFar(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut index1: i16 = 0i16;
        let mut index2: i16 = 0i16;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                <= 1i32
            {
                ResetSpriteAfterAnim(sprite);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            } else {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p1).write(((__p1).read()).wrapping_sub(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            }
        } else {
            let mut amplitude: u8 = 0u8;
            let mut cmpVal1: u8 = 0u8;
            let mut cmpVal2: u8 = 0u8;
            let mut xScale: i16 = 0i16;
            index2 = ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    .wrapping_mul(128i32),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16);
            cmpVal1 = ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
                4i32,
            )) as u8);
            cmpVal2 = ((((cmpVal1) as i32).wrapping_mul(3i32)) as u8);
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                >= ((cmpVal1) as i32))
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    < ((cmpVal2) as i32))
            {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                (__p2).write((((((__p2).read()) as i32).wrapping_add(51i32)) as i16));
                index1 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                    .read()) as i32)
                    & 255i32) as i16);
            }
            amplitude =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u8);
            if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0) {
                xScale = ((((-256i32).wrapping_add(((Sin(index2, ((amplitude) as i16))) as i32)))
                    .wrapping_add(
                        ((Sin(
                            index1,
                            (((crate::c::div_i32(((amplitude) as i32), 5i32)).wrapping_mul(2i32))
                                as i16),
                        )) as i32),
                    )) as i16);
            } else {
                xScale = ((((256i32).wrapping_sub(((Sin(index2, ((amplitude) as i16))) as i32)))
                    .wrapping_sub(
                        ((Sin(
                            index1,
                            (((crate::c::div_i32(((amplitude) as i32), 5i32)).wrapping_mul(2i32))
                                as i16),
                        )) as i32),
                    )) as i16);
            }
            SetAffineData(sprite, xScale, 256i16, 0u16);
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalStretchFar_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(40i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(40i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        HorizontalStretchFar(sprite);
    }
}
pub(crate) unsafe extern "C" fn VerticalShakeLowTwice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut var6: u8 = 0u8;
        let mut var7: u8 = 0u8;
        let mut var8: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8);
        let mut var9: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u8);
        let mut var5: u8 = (((((&raw const sVerticalShakeData).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                as isize
                * 2,
        ))
        .cast::<u8>())
        .read();
        if ((var5) as i32) != 255i32 {
            var5 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as u8);
        }
        var6 = ((((((&raw const sVerticalShakeData).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    as isize
                    * 2,
            ))
        .cast::<u8>())
        .wrapping_offset(1))
        .read();
        var7 = 0u8;
        if (((((((&raw const sVerticalShakeData).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    as isize
                    * 2,
            ))
        .cast::<u8>())
        .read()) as i32)
            != 254i32
        {
            var7 = ((crate::c::div_i32(
                (((var6) as i32).wrapping_sub(((var9) as i32))).wrapping_mul(((var5) as i32)),
                ((var6) as i32),
            )) as u8);
        } else {
            var7 = 0u8;
        }
        if ((var5) as i32) == 255i32 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        } else {
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Sin(
                    ((crate::c::rem_i32(((var8) as i32).wrapping_add(192i32), 256i32)) as i16),
                    ((var7) as i16),
                )) as i32)
                    .wrapping_add(((var7) as i32))) as i16),
            );
            if ((var9) as i32) == ((var6) as i32) {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p1).write(((__p1).read()).wrapping_add(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            } else {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                    )) as i16),
                );
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalShakeLowTwice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(40i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(6i16);
        VerticalShakeLowTwice(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalShakeLowTwice));
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalShake_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(70i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(6i16);
        HorizontalShake(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(HorizontalShake));
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalSlide_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(20i16);
        HorizontalSlide(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(HorizontalSlide));
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalVibrate_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 40i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        } else {
            let mut sign: i8 = 0i8;
            if !((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                as i32)
                & 1i32)
                != 0)
            {
                sign = 1i8;
            } else {
                sign = (-1i8);
            }
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((Sin(
                    ((crate::c::rem_i32(
                        crate::c::div_i32(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                .read()) as i32)
                                .wrapping_mul(128i32),
                            40i32,
                        ),
                        256i32,
                    )) as i16),
                    9i16,
                )) as i32)
                    .wrapping_mul(((sign) as i32))) as i16),
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalVibrate_Fastest(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 40i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        } else {
            let mut sign: i8 = 0i8;
            if !((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                as i32)
                & 1i32)
                != 0)
            {
                sign = 1i8;
            } else {
                sign = (-1i8);
            }
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((Sin(
                    ((crate::c::rem_i32(
                        crate::c::div_i32(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                .read()) as i32)
                                .wrapping_mul(128i32),
                            40i32,
                        ),
                        256i32,
                    )) as i16),
                    12i16,
                )) as i32)
                    .wrapping_mul(((sign) as i32))) as i16),
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalShakeBack_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(70i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(6i16);
        VerticalShakeBack(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalShakeBack));
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalShakeLowTwice_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(24i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(6i16);
        VerticalShakeLowTwice(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalShakeLowTwice));
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalShakeLowTwice_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(56i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(9i16);
        VerticalShakeLowTwice(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(VerticalShakeLowTwice));
    }
}
pub(crate) unsafe extern "C" fn Anim_CircleCounterclockwise_Long(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u8 = (({
            let __v1 = ((AddNewAnim()) as i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(__v1);
            __v1
        }) as u8);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(6)
        .cast::<i16>())
        .write(1024i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(8)
        .cast::<i16>())
        .write(6i16);
        (((((&raw mut sAnims).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((id) as i32) as isize * 12))
        .wrapping_add(2)
        .cast::<i16>())
        .write(24i16);
        CircleCounterclockwise(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(CircleCounterclockwise));
    }
}
pub(crate) unsafe extern "C" fn GrowStutter(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut index1: i16 = 0i16;
        let mut index2: i16 = 0i16;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
        {
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                <= 1i32
            {
                ResetSpriteAfterAnim(sprite);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            } else {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p1).write(((__p1).read()).wrapping_sub(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            }
        } else {
            let mut amplitude: u8 = 0u8;
            let mut cmpVal1: u8 = 0u8;
            let mut cmpVal2: u8 = 0u8;
            let mut xScale: i16 = 0i16;
            let mut yScale: i16 = 0i16;
            index2 = ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    .wrapping_mul(128i32),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16);
            cmpVal1 = ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
                4i32,
            )) as u8);
            cmpVal2 = ((((cmpVal1) as i32).wrapping_mul(3i32)) as u8);
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                >= ((cmpVal1) as i32))
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    < ((cmpVal2) as i32))
            {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                (__p2).write((((((__p2).read()) as i32).wrapping_add(51i32)) as i16));
                index1 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                    .read()) as i32)
                    & 255i32) as i16);
            }
            amplitude =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u8);
            if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0) {
                xScale = ((((Sin(index2, ((amplitude) as i16))) as i32).wrapping_add(
                    ((Sin(
                        index1,
                        (((crate::c::div_i32(((amplitude) as i32), 5i32)).wrapping_mul(2i32))
                            as i16),
                    )) as i32)
                        .wrapping_sub(256i32),
                )) as i16);
            } else {
                xScale = ((((256i32).wrapping_sub(
                    ((Sin(
                        index1,
                        (((crate::c::div_i32(((amplitude) as i32), 5i32)).wrapping_mul(2i32))
                            as i16),
                    )) as i32),
                ))
                .wrapping_sub(((Sin(index2, ((amplitude) as i16))) as i32)))
                    as i16);
            }
            yScale = ((((256i32).wrapping_sub(
                ((Sin(
                    index1,
                    ((crate::c::div_i32(((amplitude) as i32), 5i32)) as i16),
                )) as i32),
            ))
            .wrapping_sub(((Sin(index2, ((amplitude) as i16))) as i32)))
                as i16);
            SetAffineData(sprite, xScale, yScale, 0u16);
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_GrowStutter_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(40i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(40i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        GrowStutter(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalShakeHorizontalSlide(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 2048i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        } else {
            let mut divCase: i16 = ((crate::c::rem_i32(
                crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                    512i32,
                ),
                4i32,
            )) as i16);
            'l1: {
                let __sw1 = ((divCase) as i32);
                if __sw1 == 0i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        ((crate::c::div_i32(
                            crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ),
                            32i32,
                        )) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        ((crate::c::div_i32(
                            ((crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ))
                            .wrapping_mul(16i32))
                            .wrapping_neg(),
                            512i32,
                        )) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        (((crate::c::div_i32(
                            ((crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ))
                            .wrapping_mul(16i32))
                            .wrapping_neg(),
                            512i32,
                        ))
                        .wrapping_add(16i32)) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        (((crate::c::div_i32(
                            crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ),
                            32i32,
                        ))
                        .wrapping_sub(16i32)) as i16),
                    );
                    break 'l1;
                }
            }
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                ((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                    128i32,
                )) as i16),
                4i16,
            ));
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(48i32)) as i16));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalShakeHorizontalSlide_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 2048i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        } else {
            let mut divCase: i16 = ((crate::c::rem_i32(
                crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                    512i32,
                ),
                4i32,
            )) as i16);
            'l1: {
                let __sw1 = ((divCase) as i32);
                if __sw1 == 0i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        ((crate::c::div_i32(
                            crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ),
                            32i32,
                        )) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        ((crate::c::div_i32(
                            ((crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ))
                            .wrapping_mul(16i32))
                            .wrapping_neg(),
                            512i32,
                        )) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        (((crate::c::div_i32(
                            ((crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ))
                            .wrapping_mul(16i32))
                            .wrapping_neg(),
                            512i32,
                        ))
                        .wrapping_add(16i32)) as i16),
                    );
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        (((crate::c::div_i32(
                            crate::c::rem_i32(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                                512i32,
                            ),
                            32i32,
                        ))
                        .wrapping_sub(16i32)) as i16),
                    );
                    break 'l1;
                }
            }
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                ((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                    96i32,
                )) as i16),
                4i16,
            ));
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(64i32)) as i16));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn TriangleDown(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        }
        if crate::c::div_i32(
            ((((((((&raw const sTriangleDownData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32) as isize
                        * 3,
                ))
            .cast::<i8>())
            .wrapping_offset(2))
            .read()) as i32),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
        ) == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        }
        if crate::c::div_i32(
            ((((((((&raw const sTriangleDownData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32) as isize
                        * 3,
                ))
            .cast::<i8>())
            .wrapping_offset(2))
            .read()) as i32),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
        ) == 0i32
        {
            if (({
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                let __t3 = ((__p2).read()).wrapping_sub(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == 0i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            }
        } else {
            let mut amplitude: i32 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32);
            let __p4 = (sprite).wrapping_add(36).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32).wrapping_add(
                    (((((((&raw const sTriangleDownData).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                                .read()) as i32) as isize
                                * 3,
                        ))
                    .cast::<i8>())
                    .read()) as i32)
                        .wrapping_mul(amplitude),
                )) as i16),
            );
            let __p5 = (sprite).wrapping_add(38).cast::<i16>();
            (__p5).write(
                (((((__p5).read()) as i32).wrapping_add(
                    ((((((((&raw const sTriangleDownData).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                                .read()) as i32) as isize
                                * 3,
                        ))
                    .cast::<i8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_mul(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .read()) as i32),
                        ),
                )) as i16),
            );
            let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p6).write(((__p6).read()).wrapping_add(1));
            TryFlipX(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_TriangleDown_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(1i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(1i16);
        TriangleDown(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TriangleDown));
    }
}
pub(crate) unsafe extern "C" fn Anim_TriangleDown(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(2i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(1i16);
        TriangleDown(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TriangleDown));
    }
}
pub(crate) unsafe extern "C" fn Anim_TriangleDown_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(2i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(2i16);
        TriangleDown(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TriangleDown));
    }
}
pub(crate) unsafe extern "C" fn Grow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 255i32
        {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                <= 1i32
            {
                ResetSpriteAfterAnim(sprite);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
                HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            } else {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p1).write(((__p1).read()).wrapping_sub(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            }
        } else {
            let mut scale: i16 = 0i16;
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32),
                )) as i16),
            );
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                > 256i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(256i16);
            }
            scale = Sin(
                ((crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32),
                    2i32,
                )) as i16),
                64i16,
            );
            HandleSetAffineData(
                sprite,
                (((256i32).wrapping_sub(((scale) as i32))) as i16),
                (((256i32).wrapping_sub(((scale) as i32))) as i16),
                0u16,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_Grow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(4i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(1i16);
        }
        Grow(sprite);
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_Grow_Twice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(8i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(2i16);
        }
        Grow(sprite);
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalSpring_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(8i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(512i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(16i16);
        }
        HorizontalSpring(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalSpring_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(4i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(256i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(16i16);
        }
        HorizontalSpring(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalRepeatedSpring_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(8i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(512i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(16i16);
        }
        HorizontalRepeatedSpring(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalRepeatedSpring(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(8i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(512i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(8i16);
        }
        HorizontalRepeatedSpring(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_ShrinkGrow_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(5i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(8i16);
        }
        ShrinkGrow(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_ShrinkGrow_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(3i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(4i16);
        }
        ShrinkGrow(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalStretchBothEnds(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(30i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(60i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        VerticalStretchBothEnds(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_VerticalStretchBothEnds_Twice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(2i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(20i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(70i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        VerticalStretchBothEnds(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalStretchFar_Twice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(2i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(20i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(70i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        HorizontalStretchFar(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalStretchFar(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(30i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(60i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        HorizontalStretchFar(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_GrowStutter_Twice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(2i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(20i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(70i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        GrowStutter(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_GrowStutter(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(30i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(60i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        GrowStutter(sprite);
    }
}
pub(crate) unsafe extern "C" fn ConcaveArc(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 255i32
        {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                <= 1i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            } else {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                (__p1).write(((crate::c::rem_i32((((__p1).read()) as i32), 256i32)) as i16));
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                (__p2).write(((__p2).read()).wrapping_sub(1));
            }
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                )) as i32)
                    .wrapping_neg()) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                ((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_add(192i32),
                    256i32,
                )) as i16),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
            ));
            if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) > 0i32 {
                let __p3 = (sprite).wrapping_add(38).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_mul((-1i32))) as i16));
            }
            let __p4 = (sprite).wrapping_add(38).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32),
                )) as i16),
            );
            let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p5).write(
                (((((__p5).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32),
                )) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_ConcaveArcLarge_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(12i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(12i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(4i16);
        }
        ConcaveArc(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_ConcaveArcLarge(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(12i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(12i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(6i16);
        }
        ConcaveArc(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_ConcaveArcLarge_Twice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(2i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(12i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(12i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(8i16);
        }
        ConcaveArc(sprite);
    }
}
pub(crate) unsafe extern "C" fn ConvexDoubleArc(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 256i32
        {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                <= ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
            } else {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p1).write(((__p1).read()).wrapping_add(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            }
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        } else {
            let mut posX: i16 = 0i16;
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                > 159i32
            {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    > 256i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(256i16);
                }
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    ((((Sin(
                        ((crate::c::rem_i32(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as i32),
                            256i32,
                        )) as i16),
                        8i16,
                    )) as i32)
                        .wrapping_neg()) as i16),
                );
            } else {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    > 95i32
                {
                    ((sprite).wrapping_add(38).cast::<i16>()).write(
                        ((((Sin(96i16, 6i16)) as i32).wrapping_sub(
                            ((Sin(
                                (((((((((sprite).wrapping_add(46)).cast::<i16>())
                                    .wrapping_offset(7))
                                .read()) as i32)
                                    .wrapping_sub(96i32))
                                .wrapping_mul(2i32)) as i16),
                                4i16,
                            )) as i32),
                        )) as i16),
                    );
                } else {
                    ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                        6i16,
                    ));
                }
            }
            posX = ((((Sin(
                ((crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32),
                    2i32,
                )) as i16),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
            )) as i32)
                .wrapping_neg()) as i16);
            if crate::c::rem_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
                2i32,
            ) == 0i32
            {
                posX = ((((posX) as i32).wrapping_mul((-1i32))) as i16);
            }
            ((sprite).wrapping_add(36).cast::<i16>()).write(posX);
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32),
                )) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_ConvexDoubleArc_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(2i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(16i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(4i16);
        }
        ConvexDoubleArc(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_ConvexDoubleArc(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(2i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(16i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(6i16);
        }
        ConvexDoubleArc(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_ConvexDoubleArc_Twice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(3i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(16i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(8i16);
        }
        ConvexDoubleArc(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_ConcaveArcSmall_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(4i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(6i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(4i16);
        }
        ConcaveArc(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_ConcaveArcSmall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(4i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(6i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(6i16);
        }
        ConcaveArc(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_ConcaveArcSmall_Twice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(2i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(4i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(6i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(8i16);
        }
        ConcaveArc(sprite);
    }
}
pub(crate) unsafe extern "C" fn SetHorizontalDip(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut index: u16 = ((Sin(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_mul(128i32),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            )) as i16),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
        )) as u16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write((((((index) as i32) << 8).wrapping_neg()) as i16));
        SetPosForRotation(
            sprite,
            index,
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
            0i16,
        );
        HandleSetAffineData(
            sprite,
            256i16,
            256i16,
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalDip(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(60i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(8i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write((-32i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(1i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                <= (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            {
                ResetSpriteAfterAnim(sprite);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
                return;
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            }
        } else {
            SetHorizontalDip(sprite);
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalDip_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(90i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(8i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write((-32i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(1i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                <= (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            {
                ResetSpriteAfterAnim(sprite);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
                return;
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            }
        } else {
            SetHorizontalDip(sprite);
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_HorizontalDip_Twice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(30i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(8i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write((-32i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(2i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
        {
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                <= (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            {
                ResetSpriteAfterAnim(sprite);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimEnd));
                return;
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            }
        } else {
            SetHorizontalDip(sprite);
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn ShrinkGrowVibrate(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
        {
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            HandleSetAffineData(sprite, 256i16, 256i16, 0u16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            let mut sinY: i8 = 0i8;
            let mut y: u16 = 0u16;
            let mut index: i16 = ((crate::c::rem_i32(
                crate::c::div_i32(
                    ((((crate::c::rem_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                    ))
                    .wrapping_mul(256i32)) as u16) as i32),
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32),
                ),
                256i32,
            )) as i16);
            if crate::c::rem_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
                2i32,
            ) == 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                    .write(((((Sin(index, 32i16)) as i32).wrapping_add(256i32)) as i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                    .write(((((Sin(index, 32i16)) as i32).wrapping_add(256i32)) as i16));
                sinY = ((Sin(index, 32i16)) as i8);
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                    .write(((((Sin(index, 8i16)) as i32).wrapping_add(256i32)) as i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                    .write(((((Sin(index, 8i16)) as i32).wrapping_add(256i32)) as i16));
                sinY = ((Sin(index, 8i16)) as i8);
            }
            y = ((crate::c::div_i32(((sinY) as i32), 8i32)) as u16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(((y) as i16));
            HandleSetAffineData(
                sprite,
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                0u16,
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_ShrinkGrowVibrate_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (sprite).wrapping_add(38).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(40i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(80i16);
        }
        ShrinkGrowVibrate(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_ShrinkGrowVibrate(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (sprite).wrapping_add(38).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(40i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(40i16);
        }
        ShrinkGrowVibrate(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_ShrinkGrowVibrate_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            HandleStartAffineAnim(sprite);
            let __p1 = (sprite).wrapping_add(38).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(80i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(80i16);
        }
        ShrinkGrowVibrate(sprite);
    }
}
pub(crate) unsafe extern "C" fn JoltRight(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)
            <= ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                .wrapping_neg()
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32)
                    .wrapping_neg()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(2i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(JoltRight_0));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn JoltRight_0(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            )) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p2).write(((__p2).read()).wrapping_add(1));
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) >= 0i32 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(JoltRight_1));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn JoltRight_1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            )) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p2).write(((__p2).read()).wrapping_add(1));
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
        {
            ((sprite).wrapping_add(36).cast::<i16>())
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read());
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(JoltRight_2));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn JoltRight_2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            >= ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(JoltRight_3));
        } else {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32),
                )) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p2).write((((((__p2).read()) as i32).wrapping_mul((-1i32))) as i16));
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn JoltRight_3(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        TryFlipX(sprite);
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(2i32)) as i16));
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) <= 0i32 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ResetSpriteAfterAnim(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        }
        TryFlipX(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_JoltRight_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        HandleStartAffineAnim(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(4i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(12i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(16i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(4i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(2i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(JoltRight));
    }
}
pub(crate) unsafe extern "C" fn Anim_JoltRight(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        HandleStartAffineAnim(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(2i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(8i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(12i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(2i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(JoltRight));
    }
}
pub(crate) unsafe extern "C" fn Anim_JoltRight_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        HandleStartAffineAnim(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(6i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(6i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(2i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(JoltRight));
    }
}
pub(crate) unsafe extern "C" fn SetShakeFlashYellowPos(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>())
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read());
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 1i32 {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write((((((__p1).read()) as i32).wrapping_mul((-1i32))) as i16));
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        } else {
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn ShakeFlashYellow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut array: *mut u8 = ((((&raw const sShakeYellowFlashData)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                as isize,
        ))
        .read();
        SetShakeFlashYellowPos(sprite);
        if (((((array).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                as isize
                * 4,
        ))
        .wrapping_add(1))
        .read()) as i32)
            == 255i32
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                == 1i32
            {
                if (((array).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize
                        * 4,
                ))
                .read())
                    != 0
                {
                    BlendPalette(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as u16),
                        16u16,
                        16u8,
                        1023u16,
                    );
                } else {
                    BlendPalette(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as u16),
                        16u16,
                        0u8,
                        1023u16,
                    );
                }
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            }
            if (((((array).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    as isize
                    * 4,
            ))
            .wrapping_add(1))
            .read()) as i32)
                == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                (__p1).write(((__p1).read()).wrapping_add(1));
            } else {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_ShakeFlashYellow_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 1i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                (((256i32).wrapping_add(
                    ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                        .wrapping_mul(16i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        }
        ShakeFlashYellow(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_ShakeFlashYellow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 1i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                (((256i32).wrapping_add(
                    ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                        .wrapping_mul(16i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(1i16);
        }
        ShakeFlashYellow(sprite);
    }
}
pub(crate) unsafe extern "C" fn Anim_ShakeFlashYellow_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 1i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                (((256i32).wrapping_add(
                    ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                        .wrapping_mul(16i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(2i16);
        }
        ShakeFlashYellow(sprite);
    }
}
pub(crate) unsafe extern "C" fn ShakeGlow_Blend(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            > 127i32
        {
            BlendPalette(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as u16),
                16u16,
                0u8,
                31u16,
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimEnd));
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                12i16,
            ));
            BlendPalette(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as u16),
                16u16,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u8),
                ((((&raw const sColors_0).cast::<u8>().cast_mut().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32) as isize,
                    ))
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ShakeGlow_Move(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
        {
            TryFlipX(sprite);
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                > (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            {
                if (({
                    let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    let __t2 = ((__p1).read()).wrapping_add(1);
                    (__p1).write(__t2);
                    __t2
                }) as i32)
                    < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                }
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            } else {
                let mut sign: i8 = (((1i32).wrapping_sub(
                    (crate::c::rem_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                        2i32,
                    ))
                    .wrapping_mul(2i32),
                )) as i8);
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    ((((sign) as i32).wrapping_mul(
                        ((Sin(
                            ((crate::c::rem_i32(
                                crate::c::div_i32(
                                    ((((((sprite).wrapping_add(46)).cast::<i16>())
                                        .wrapping_offset(5))
                                    .read()) as i32)
                                        .wrapping_mul(384i32),
                                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                                ),
                                256i32,
                            )) as i16),
                            6i16,
                        )) as i32),
                    )) as i16),
                );
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
            TryFlipX(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn Anim_ShakeGlowRed_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                (((256i32).wrapping_add(
                    ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                        .wrapping_mul(16i32),
                )) as i16),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(10i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(2i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        }
        if crate::c::rem_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            2i32,
        ) == 0i32
        {
            ShakeGlow_Blend(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >= crate::c::div_i32(
                (128i32).wrapping_sub(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    ),
                ),
                2i32,
            )
        {
            ShakeGlow_Move(sprite);
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_ShakeGlowRed(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                (((256i32).wrapping_add(
                    ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                        .wrapping_mul(16i32),
                )) as i16),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(20i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        }
        if crate::c::rem_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            2i32,
        ) == 0i32
        {
            ShakeGlow_Blend(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >= crate::c::div_i32(
                (128i32).wrapping_sub(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    ),
                ),
                2i32,
            )
        {
            ShakeGlow_Move(sprite);
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_ShakeGlowRed_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                (((256i32).wrapping_add(
                    ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                        .wrapping_mul(16i32),
                )) as i16),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(80i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        }
        if crate::c::rem_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            2i32,
        ) == 0i32
        {
            ShakeGlow_Blend(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >= crate::c::div_i32(
                (128i32).wrapping_sub(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    ),
                ),
                2i32,
            )
        {
            ShakeGlow_Move(sprite);
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_ShakeGlowGreen_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                (((256i32).wrapping_add(
                    ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                        .wrapping_mul(16i32),
                )) as i16),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(10i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(2i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(1i16);
        }
        if crate::c::rem_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            2i32,
        ) == 0i32
        {
            ShakeGlow_Blend(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >= crate::c::div_i32(
                (128i32).wrapping_sub(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    ),
                ),
                2i32,
            )
        {
            ShakeGlow_Move(sprite);
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_ShakeGlowGreen(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                (((256i32).wrapping_add(
                    ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                        .wrapping_mul(16i32),
                )) as i16),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(20i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(1i16);
        }
        if crate::c::rem_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            2i32,
        ) == 0i32
        {
            ShakeGlow_Blend(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >= crate::c::div_i32(
                (128i32).wrapping_sub(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    ),
                ),
                2i32,
            )
        {
            ShakeGlow_Move(sprite);
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_ShakeGlowGreen_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                (((256i32).wrapping_add(
                    ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                        .wrapping_mul(16i32),
                )) as i16),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(80i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(1i16);
        }
        if crate::c::rem_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            2i32,
        ) == 0i32
        {
            ShakeGlow_Blend(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >= crate::c::div_i32(
                (128i32).wrapping_sub(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    ),
                ),
                2i32,
            )
        {
            ShakeGlow_Move(sprite);
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_ShakeGlowBlue_Fast(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                (((256i32).wrapping_add(
                    ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                        .wrapping_mul(16i32),
                )) as i16),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(10i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(2i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(2i16);
        }
        if crate::c::rem_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            2i32,
        ) == 0i32
        {
            ShakeGlow_Blend(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >= crate::c::div_i32(
                (128i32).wrapping_sub(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    ),
                ),
                2i32,
            )
        {
            ShakeGlow_Move(sprite);
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_ShakeGlowBlue(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                (((256i32).wrapping_add(
                    ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                        .wrapping_mul(16i32),
                )) as i16),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(20i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(2i16);
        }
        if crate::c::rem_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            2i32,
        ) == 0i32
        {
            ShakeGlow_Blend(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >= crate::c::div_i32(
                (128i32).wrapping_sub(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    ),
                ),
                2i32,
            )
        {
            ShakeGlow_Move(sprite);
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Anim_ShakeGlowBlue_Slow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                (((256i32).wrapping_add(
                    ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                        .wrapping_mul(16i32),
                )) as i16),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(80i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(2i16);
        }
        if crate::c::rem_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            2i32,
        ) == 0i32
        {
            ShakeGlow_Blend(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >= crate::c::div_i32(
                (128i32).wrapping_sub(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    ),
                ),
                2i32,
            )
        {
            ShakeGlow_Move(sprite);
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn WaitAnimEnd(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
