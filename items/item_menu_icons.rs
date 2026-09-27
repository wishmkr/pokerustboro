//! Translated from `src/item_menu_icons.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sRotatingBall_Pal sRotatingBall_Gfx sCherryUnused sCherryUnused_Pal sBagOamData sSpriteAnim_Bag_Closed sSpriteAnim_Bag_Items sSpriteAnim_Bag_KeyItems sSpriteAnim_Bag_Pokeballs sSpriteAnim_Bag_TMsHMs sSpriteAnim_Bag_Berries sBagSpriteAnimTable sSpriteAffineAnim_BagNormal sSpriteAffineAnim_BagShake sBagAffineAnimCmds gBagMaleSpriteSheet gBagFemaleSpriteSheet gBagPaletteTable sBagSpriteTemplate sRotatingBallOamData sSpriteAffineAnim_RotatingBallStationary sRotatingBallSpriteAnimTable sSpriteAffineAnim_RotatingBallRotation1 sSpriteAffineAnim_RotatingBallRotation2 sRotatingBallAnimCmds sRotatingBallAnimCmds_FullRotation sRotatingBallTable sRotatingBallPaletteTable sRotatingBallSpriteTemplate sBerryPicOamData sBerryPicRotatingOamData sAnim_BerryPic sBerryPicSpriteAnimTable sBerryPicSpriteImageTable sBerryPicSpriteTemplate sSpriteAffineAnim_BerryPicRotation1 sSpriteAffineAnim_BerryPicRotation2 sBerryPicRotatingAnimCmds sBerryPicRotatingSpriteTemplate sBerryPicTable gBerryCheckCircleSpriteSheet gBerryCheckCirclePaletteTable sBerryCheckCircleOamData sSpriteAnim_BerryCheckCircle sBerryCheckCircleSpriteAnimTable sBerryCheckCircleSpriteTemplate
#[allow(unused_imports)]
use crate::data::item_menu_icons::*;

unsafe extern "C" {
    static mut gBagMenu: u8;
    static mut gDecompressionBuffer: u8;
    static mut gSprites: u8;
    fn AddItemIconSprite(a0: u16, a1: u16, a2: u16) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateSwapLineSprites(a0: *mut u8, a1: u8);
    fn DestroySprite(a0: *mut u8);
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn InitSpriteAffineAnim(a0: *mut u8);
    fn IsEnigmaBerryValid() -> u32;
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn SetSwapLineSpritesInvisibility(a0: *mut u8, a1: u8, a2: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn UpdateSwapLineSpritesPos(a0: *mut u8, a1: u8, a2: i16, a3: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveBagSprite(id: u8) {
    unsafe {
        let mut id = id;
        let mut spriteId: *mut u8 =
            (((((&raw mut gBagMenu).cast::<*mut u8>()).read()).wrapping_add(2052)).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize);
        if (((spriteId).read()) as i32) != 255i32 {
            FreeSpriteTilesByTag(((((id) as i32).wrapping_add(100i32)) as u16));
            FreeSpritePaletteByTag(((((id) as i32).wrapping_add(100i32)) as u16));
            FreeSpriteOamMatrix(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset((((spriteId).read()) as i32) as isize * 68),
            );
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset((((spriteId).read()) as i32) as isize * 68),
            );
            (spriteId).write(255u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddBagVisualSprite(bagPocketId: u8) {
    unsafe {
        let mut bagPocketId = bagPocketId;
        let mut spriteId: *mut u8 =
            ((((&raw mut gBagMenu).cast::<*mut u8>()).read()).wrapping_add(2052)).cast::<u8>();
        (spriteId).write(CreateSprite(
            (&raw const sBagSpriteTemplate).cast::<u8>().cast_mut(),
            68i16,
            66i16,
            0u8,
        ));
        SetBagVisualPocketId(bagPocketId, 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBagVisualPocketId(bagPocketId: u8, isSwitchingPockets: u8) {
    unsafe {
        let mut bagPocketId = bagPocketId;
        let mut isSwitchingPockets = isSwitchingPockets;
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((&raw mut gBagMenu).cast::<*mut u8>()).read()).wrapping_add(2052)).cast::<u8>())
                .read()) as i32) as isize
                * 68,
        );
        if (isSwitchingPockets) != 0 {
            ((sprite).wrapping_add(38).cast::<i16>()).write((-5i16));
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_BagVisualSwitchingPockets));
            (((sprite).wrapping_add(46)).cast::<i16>())
                .write(((((bagPocketId) as i32).wrapping_add(1i32)) as i16));
            StartSpriteAnim(sprite, 0u8);
        } else {
            StartSpriteAnim(sprite, ((((bagPocketId) as i32).wrapping_add(1i32)) as u8));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BagVisualSwitchingPockets(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) != 0i32 {
            let __p1 = (sprite).wrapping_add(38).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            StartSpriteAnim(
                sprite,
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShakeBagSprite() {
    unsafe {
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((&raw mut gBagMenu).cast::<*mut u8>()).read()).wrapping_add(2052)).cast::<u8>())
                .read()) as i32) as isize
                * 68,
        );
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            StartSpriteAffineAnim(sprite, 1u8);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_ShakeBagSprite));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ShakeBagSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            StartSpriteAffineAnim(sprite, 0u8);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddSwitchPocketRotatingBallSprite(rotationDirection: i16) {
    unsafe {
        let mut rotationDirection = rotationDirection;
        let mut spriteId: *mut u8 =
            (((((&raw mut gBagMenu).cast::<*mut u8>()).read()).wrapping_add(2052)).cast::<u8>())
                .wrapping_offset(1);
        LoadSpriteSheet((&raw const sRotatingBallTable).cast::<u8>().cast_mut());
        LoadSpritePalette(
            (&raw const sRotatingBallPaletteTable)
                .cast::<u8>()
                .cast_mut(),
        );
        (spriteId).write(CreateSprite(
            (&raw const sRotatingBallSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            16i16,
            16i16,
            0u8,
        ));
        (((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteId).read()) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .write(rotationDirection);
    }
}
pub(crate) unsafe extern "C" fn UpdateSwitchPocketRotatingBallCoords(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(40).cast::<i8>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_sub(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        .wrapping_add(1i32)
                        & 1i32),
                )) as i8),
        );
        ((sprite).wrapping_add(41).cast::<i8>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_sub(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        .wrapping_add(1i32)
                        & 1i32),
                )) as i8),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_SwitchPocketRotatingBallInit(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (1u32) as i32);
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == (-1i32) {
            ((sprite).wrapping_add(16).cast::<*mut *mut u8>()).write(
                ((&raw const sRotatingBallAnimCmds)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        } else {
            ((sprite).wrapping_add(16).cast::<*mut *mut u8>()).write(
                ((&raw const sRotatingBallAnimCmds_FullRotation)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        }
        InitSpriteAffineAnim(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((((sprite).wrapping_add(40).cast::<i8>()).read()) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((((sprite).wrapping_add(41).cast::<i8>()).read()) as i16));
        UpdateSwitchPocketRotatingBallCoords(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_SwitchPocketRotatingBallContinue));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_SwitchPocketRotatingBallContinue(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p1).write(((__p1).read()).wrapping_add(1));
        UpdateSwitchPocketRotatingBallCoords(sprite);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            == 16i32
        {
            RemoveBagSprite(1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddBagItemIconSprite(itemId: u16, id: u8) {
    unsafe {
        let mut itemId = itemId;
        let mut id = id;
        let mut spriteId: *mut u8 =
            (((((&raw mut gBagMenu).cast::<*mut u8>()).read()).wrapping_add(2052)).cast::<u8>())
                .wrapping_offset((((id) as i32).wrapping_add(2i32)) as isize);
        if (((spriteId).read()) as i32) == 255i32 {
            let mut iconSpriteId: u8 = 0u8;
            FreeSpriteTilesByTag(((((id) as i32).wrapping_add(102i32)) as u16));
            FreeSpritePaletteByTag(((((id) as i32).wrapping_add(102i32)) as u16));
            iconSpriteId = AddItemIconSprite(
                ((((id) as i32).wrapping_add(102i32)) as u16),
                ((((id) as i32).wrapping_add(102i32)) as u16),
                itemId,
            );
            if ((iconSpriteId) as i32) != 64i32 {
                (spriteId).write(iconSpriteId);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((iconSpriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(24i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((iconSpriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(88i16);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveBagItemIconSprite(id: u8) {
    unsafe {
        let mut id = id;
        RemoveBagSprite(((((id) as i32).wrapping_add(2i32)) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateItemMenuSwapLine() {
    unsafe {
        CreateSwapLineSprites(
            (((((&raw mut gBagMenu).cast::<*mut u8>()).read()).wrapping_add(2052)).cast::<u8>())
                .wrapping_offset(4),
            8u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetItemMenuSwapLineInvisibility(invisible: u8) {
    unsafe {
        let mut invisible = invisible;
        SetSwapLineSpritesInvisibility(
            (((((&raw mut gBagMenu).cast::<*mut u8>()).read()).wrapping_add(2052)).cast::<u8>())
                .wrapping_offset(4),
            8u8,
            invisible,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateItemMenuSwapLinePos(y: u8) {
    unsafe {
        let mut y = y;
        UpdateSwapLineSpritesPos(
            (((((&raw mut gBagMenu).cast::<*mut u8>()).read()).wrapping_add(2052)).cast::<u8>())
                .wrapping_offset(4),
            136u8,
            120i16,
            (((((y) as i32).wrapping_add(1i32)).wrapping_mul(16i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn ArrangeBerryGfx(src: *mut u8, dest: *mut u8) {
    unsafe {
        let mut src = src;
        let mut dest = dest;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        crate::c::memset(dest, 0i32, 2048u32);
        dest = (dest).wrapping_offset(256);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    dest = (dest).wrapping_offset(32);
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 6i32) {
                                break 'l3;
                            }
                            'l4: {
                                crate::c::memcpy(dest, src, 32u32);
                                dest = (dest).wrapping_offset(32);
                                src = (src).wrapping_offset(32);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if ((i) as i32) != 5i32 {
                        dest = (dest).wrapping_offset(32);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadBerryGfx(berryId: u8) {
    unsafe {
        let mut berryId = berryId;
        let mut pal = crate::ffi::Align4([0u8; 8]);
        if (((berryId) as i32) == 42i32) && ((IsEnigmaBerryValid()) != 0) {}
        (((&raw mut pal).cast::<u8>()).cast::<*mut u32>()).write(
            (((((&raw const sBerryPicTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((berryId) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<*mut u32>())
            .read(),
        );
        (((&raw mut pal).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(30020u16);
        LoadCompressedSpritePalette((&raw mut pal).cast::<u8>());
        LZDecompressWram(
            (((((&raw const sBerryPicTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((berryId) as i32) as isize * 8))
            .cast::<*mut u32>())
            .read(),
            ((&raw mut gDecompressionBuffer).cast::<u8>()).wrapping_offset(4096),
        );
        ArrangeBerryGfx(
            ((&raw mut gDecompressionBuffer).cast::<u8>()).wrapping_offset(4096),
            (&raw mut gDecompressionBuffer).cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateBerryTagSprite(id: u8, x: i16, y: i16) -> u8 {
    unsafe {
        let mut id = id;
        let mut x = x;
        let mut y = y;
        LoadBerryGfx(id);
        return CreateSprite(
            (&raw const sBerryPicSpriteTemplate).cast::<u8>().cast_mut(),
            x,
            y,
            0u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeBerryTagSpritePalette() {
    unsafe {
        FreeSpritePaletteByTag(30020u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateSpinningBerrySprite(
    berryId: u8,
    x: u8,
    y: u8,
    startAffine: u8,
) -> u8 {
    unsafe {
        let mut berryId = berryId;
        let mut x = x;
        let mut y = y;
        let mut startAffine = startAffine;
        let mut spriteId: u8 = 0u8;
        FreeSpritePaletteByTag(30020u16);
        LoadBerryGfx(berryId);
        spriteId = CreateSprite(
            (&raw const sBerryPicRotatingSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            ((x) as i16),
            ((y) as i16),
            0u8,
        );
        if ((startAffine) as i32) == 1i32 {
            StartSpriteAffineAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                1u8,
            );
        }
        return spriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateBerryFlavorCircleSprite(x: i16) -> u8 {
    unsafe {
        let mut x = x;
        return CreateSprite(
            (&raw const sBerryCheckCircleSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            x,
            116i16,
            0u8,
        );
    }
}
