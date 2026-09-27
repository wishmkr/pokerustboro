//! Translated from `src/battle_interface.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sOamData_64x32 sHealthboxPlayerSpriteTemplates sHealthboxOpponentSpriteTemplates sHealthboxSafariSpriteTemplate sOamData_Healthbar sHealthbarSpriteTemplates sUnused_Subsprites_0 sUnused_Subsprites_2 sUnused_Subsprites_1 sUnused_Subsprites_3 sHealthBar_Subsprites_Player sHealthBar_Subsprites_Opponent sUnused_SubspriteTable sHealthBar_SubspriteTables sStatusSummaryBar_Subsprites_Enter sStatusSummaryBar_Subsprites_Exit sStatusSummaryBar_SubspriteTable_Enter sStatusSummaryBar_SubspriteTable_Exit sUnusedStatusSummary sStatusSummaryBarSpriteSheet sStatusSummaryBarSpritePal sStatusSummaryBallsSpritePal sStatusSummaryBallsSpriteSheet sOamData_Unused64x32 sOamData_StatusSummaryBalls sStatusSummaryBarSpriteTemplates sStatusSummaryBallsSpriteTemplates sEmptyWhiteText_GrayHighlight sEmptyWhiteText_TransparentHighlight sStatusIconColors sHealthboxWindowTemplate
#[allow(unused_imports)]
use crate::data::battle_interface::*;

unsafe extern "C" {
    static mut gBattleSpritesDataPtr: u8;
    static mut gBattleStruct: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerPositions: u8;
    static mut gBattlersCount: u8;
    static mut gBitTable: u8;
    static mut gDisplayedStringBattle: u8;
    static mut gEnemyParty: u8;
    static mut gExperienceTables: u8;
    static mut gHealthboxElementsGfxTable: u8;
    static mut gHealthboxSpriteIds: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gNatureNamePointers: u8;
    static mut gNumSafariBalls: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSpeciesInfo: u8;
    static mut gSpeciesNames: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gText_HealthboxGender_Female: u8;
    static mut gText_HealthboxGender_Male: u8;
    static mut gText_HealthboxGender_None: u8;
    static mut gText_HealthboxNickname: u8;
    static mut gText_SafariBallLeft: u8;
    static mut gText_SafariBalls: u8;
    static mut gText_Slash: u8;
    fn AddTextPrinterParameterized4(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: *mut u8,
        a7: i8,
        a8: *mut u8,
    );
    fn AddWindow(a0: *mut u8) -> u16;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateSpriteAtEnd(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndFreeResources(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn FillPalette(a0: u16, a1: u16, a2: u16);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut u8) -> u8;
    fn GetNature(a0: *mut u8) -> u8;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn IsDoubleBattle() -> u8;
    fn LoadBattleBarGfx(a0: u8);
    fn LoadCompressedSpriteSheetUsingHeap(a0: *mut u8) -> u8;
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn PlaySE1WithPanning(a0: u16, a1: i8);
    fn PlaySE2WithPanning(a0: u16, a1: i8);
    fn RemoveWindow(a0: u8);
    fn RenderTextHandleBold(a0: *mut u8, a1: u8, a2: *mut u8) -> u8;
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetSubspriteTables(a0: *mut u8, a1: *mut u8);
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
    fn TaskDummy(a0: u8);
}

pub(crate) unsafe extern "C" fn DummiedOutFunction(
    unused1: i16,
    unused2: i16,
    unused3: i32,
) -> i32 {
    unsafe {
        let mut unused1 = unused1;
        let mut unused2 = unused2;
        let mut unused3 = unused3;
        return 9i32;
    }
}
pub(crate) unsafe extern "C" fn Debug_DrawNumber(number: i16, dest: *mut u16, unk: u8) {
    unsafe {
        let mut number = number;
        let mut dest = dest;
        let mut unk = unk;
        let mut i: i8 = 0i8;
        let mut j: i8 = 0i8;
        let mut buff = crate::ffi::Align4([0u8; 4]);
        {
            i = 0i8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut buff).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 3i8;
            'l3: loop {
                'l4: {
                    if ((number) as i32) > 0i32 {
                        (((&raw mut buff).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                            .write(((crate::c::rem_i32(((number) as i32), 10i32)) as u8));
                        number = ((crate::c::div_i32(((number) as i32), 10i32)) as i16);
                    } else {
                        {
                            'l5: loop {
                                if !(((i) as i32) > (-1i32)) {
                                    break 'l5;
                                }
                                'l6: {
                                    (((&raw mut buff).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .write(255u8);
                                }
                                i = (i).wrapping_sub(1);
                            }
                        }
                        if (((((&raw mut buff).cast::<u8>()).wrapping_offset(3)).read()) as i32)
                            == 255i32
                        {
                            (((&raw mut buff).cast::<u8>()).wrapping_offset(3)).write(0u8);
                        }
                        break 'l3;
                    }
                }
                i = (i).wrapping_sub(1);
            }
        }
        if !((unk) != 0) {
            {
                i = 0i8;
                j = 0i8;
                'l7: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l7;
                    }
                    'l8: {
                        if (((((&raw mut buff).cast::<u8>())
                            .wrapping_offset(((j) as i32) as isize))
                        .read()) as i32)
                            == 255i32
                        {
                            let __p1 =
                                (dest).wrapping_offset((((j) as i32).wrapping_add(0i32)) as isize);
                            (__p1).write((((((__p1).read()) as i32) & 64512i32) as u16));
                            let __p2 =
                                (dest).wrapping_offset((((j) as i32).wrapping_add(0i32)) as isize);
                            (__p2).write((((((__p2).read()) as i32) | 30i32) as u16));
                            let __p3 =
                                (dest).wrapping_offset((((i) as i32).wrapping_add(32i32)) as isize);
                            (__p3).write((((((__p3).read()) as i32) & 64512i32) as u16));
                            let __p4 =
                                (dest).wrapping_offset((((i) as i32).wrapping_add(32i32)) as isize);
                            (__p4).write((((((__p4).read()) as i32) | 30i32) as u16));
                        } else {
                            let __p5 =
                                (dest).wrapping_offset((((j) as i32).wrapping_add(0i32)) as isize);
                            (__p5).write((((((__p5).read()) as i32) & 64512i32) as u16));
                            let __p6 =
                                (dest).wrapping_offset((((j) as i32).wrapping_add(0i32)) as isize);
                            (__p6).write(
                                (((((__p6).read()) as i32)
                                    | (20i32).wrapping_add(
                                        (((((&raw mut buff).cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize))
                                        .read()) as i32),
                                    )) as u16),
                            );
                            let __p7 =
                                (dest).wrapping_offset((((i) as i32).wrapping_add(32i32)) as isize);
                            (__p7).write((((((__p7).read()) as i32) & 64512i32) as u16));
                            let __p8 =
                                (dest).wrapping_offset((((i) as i32).wrapping_add(32i32)) as isize);
                            (__p8).write(
                                (((((__p8).read()) as i32)
                                    | (52i32).wrapping_add(
                                        (((((&raw mut buff).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32),
                                    )) as u16),
                            );
                        }
                        j = (j).wrapping_add(1);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0i8;
                'l9: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l9;
                    }
                    'l10: {
                        if (((((&raw mut buff).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == 255i32
                        {
                            let __p9 =
                                (dest).wrapping_offset((((i) as i32).wrapping_add(0i32)) as isize);
                            (__p9).write((((((__p9).read()) as i32) & 64512i32) as u16));
                            let __p10 =
                                (dest).wrapping_offset((((i) as i32).wrapping_add(0i32)) as isize);
                            (__p10).write((((((__p10).read()) as i32) | 30i32) as u16));
                            let __p11 =
                                (dest).wrapping_offset((((i) as i32).wrapping_add(32i32)) as isize);
                            (__p11).write((((((__p11).read()) as i32) & 64512i32) as u16));
                            let __p12 =
                                (dest).wrapping_offset((((i) as i32).wrapping_add(32i32)) as isize);
                            (__p12).write((((((__p12).read()) as i32) | 30i32) as u16));
                        } else {
                            let __p13 =
                                (dest).wrapping_offset((((i) as i32).wrapping_add(0i32)) as isize);
                            (__p13).write((((((__p13).read()) as i32) & 64512i32) as u16));
                            let __p14 =
                                (dest).wrapping_offset((((i) as i32).wrapping_add(0i32)) as isize);
                            (__p14).write(
                                (((((__p14).read()) as i32)
                                    | (20i32).wrapping_add(
                                        (((((&raw mut buff).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32),
                                    )) as u16),
                            );
                            let __p15 =
                                (dest).wrapping_offset((((i) as i32).wrapping_add(32i32)) as isize);
                            (__p15).write((((((__p15).read()) as i32) & 64512i32) as u16));
                            let __p16 =
                                (dest).wrapping_offset((((i) as i32).wrapping_add(32i32)) as isize);
                            (__p16).write(
                                (((((__p16).read()) as i32)
                                    | (52i32).wrapping_add(
                                        (((((&raw mut buff).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32),
                                    )) as u16),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Debug_DrawNumberPair(number1: i16, number2: i16, dest: *mut u16) {
    unsafe {
        let mut number1 = number1;
        let mut number2 = number2;
        let mut dest = dest;
        ((dest).wrapping_offset(4)).write(30u16);
        Debug_DrawNumber(number2, dest, 0u8);
        Debug_DrawNumber(number1, (dest).wrapping_offset(5), 1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateBattlerHealthboxSprites(battler: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut data6: i16 = 0i16;
        let mut healthboxLeftSpriteId: u8 = 0u8;
        let mut healthboxRightSpriteId: u8 = 0u8;
        let mut healthbarSpriteId: u8 = 0u8;
        let mut healthBarSpritePtr: *mut u8 = core::ptr::null_mut();
        if !((IsDoubleBattle()) != 0) {
            if ((GetBattlerSide(battler)) as i32) == 0i32 {
                healthboxLeftSpriteId = CreateSprite(
                    ((&raw const sHealthboxPlayerSpriteTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    240i16,
                    160i16,
                    1u8,
                );
                healthboxRightSpriteId = CreateSpriteAtEnd(
                    ((&raw const sHealthboxPlayerSpriteTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    240i16,
                    160i16,
                    1u8,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((healthboxLeftSpriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    6,
                    2,
                    (0u32) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    6,
                    2,
                    (0u32) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    ((((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
                        .wrapping_add(4),
                        0,
                        10,
                        false,
                    ) as u16) as i32)
                        .wrapping_add(64i32)) as u16) as i32,
                );
            } else {
                healthboxLeftSpriteId = CreateSprite(
                    ((&raw const sHealthboxOpponentSpriteTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    240i16,
                    160i16,
                    1u8,
                );
                healthboxRightSpriteId = CreateSpriteAtEnd(
                    ((&raw const sHealthboxOpponentSpriteTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    240i16,
                    160i16,
                    1u8,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    ((((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
                        .wrapping_add(4),
                        0,
                        10,
                        false,
                    ) as u16) as i32)
                        .wrapping_add(32i32)) as u16) as i32,
                );
                data6 = 2i16;
            }
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxLeftSpriteId) as i32) as isize * 68))
            .wrapping_add(6)
            .cast::<u16>())
            .write(((healthboxRightSpriteId) as u16));
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(((healthboxLeftSpriteId) as i16));
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_HealthBoxOther));
        } else {
            if ((GetBattlerSide(battler)) as i32) == 0i32 {
                healthboxLeftSpriteId = CreateSprite(
                    (((&raw const sHealthboxPlayerSpriteTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (crate::c::div_i32(((GetBattlerPosition(battler)) as i32), 2i32)) as isize
                            * 24,
                    ),
                    240i16,
                    160i16,
                    1u8,
                );
                healthboxRightSpriteId = CreateSpriteAtEnd(
                    (((&raw const sHealthboxPlayerSpriteTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (crate::c::div_i32(((GetBattlerPosition(battler)) as i32), 2i32)) as isize
                            * 24,
                    ),
                    240i16,
                    160i16,
                    1u8,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthboxLeftSpriteId) as i32) as isize * 68))
                .wrapping_add(6)
                .cast::<u16>())
                .write(((healthboxRightSpriteId) as u16));
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(((healthboxLeftSpriteId) as i16));
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    ((((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
                        .wrapping_add(4),
                        0,
                        10,
                        false,
                    ) as u16) as i32)
                        .wrapping_add(32i32)) as u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_HealthBoxOther));
                data6 = 1i16;
            } else {
                healthboxLeftSpriteId = CreateSprite(
                    (((&raw const sHealthboxOpponentSpriteTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (crate::c::div_i32(((GetBattlerPosition(battler)) as i32), 2i32)) as isize
                            * 24,
                    ),
                    240i16,
                    160i16,
                    1u8,
                );
                healthboxRightSpriteId = CreateSpriteAtEnd(
                    (((&raw const sHealthboxOpponentSpriteTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (crate::c::div_i32(((GetBattlerPosition(battler)) as i32), 2i32)) as isize
                            * 24,
                    ),
                    240i16,
                    160i16,
                    1u8,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthboxLeftSpriteId) as i32) as isize * 68))
                .wrapping_add(6)
                .cast::<u16>())
                .write(((healthboxRightSpriteId) as u16));
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(((healthboxLeftSpriteId) as i16));
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    ((((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
                        .wrapping_add(4),
                        0,
                        10,
                        false,
                    ) as u16) as i32)
                        .wrapping_add(32i32)) as u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_HealthBoxOther));
                data6 = 2i16;
            }
        }
        healthbarSpriteId = CreateSpriteAtEnd(
            (((&raw const sHealthbarSpriteTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gBattlerPositions).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as i32) as isize
                    * 24,
            ),
            140i16,
            60i16,
            0u8,
        );
        healthBarSpritePtr = ((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthbarSpriteId) as i32) as isize * 68);
        SetSubspriteTables(
            healthBarSpritePtr,
            (((&raw const sHealthBar_SubspriteTables)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((GetBattlerSide(battler)) as i32) as isize * 8),
        );
        crate::c::bf_write((healthBarSpritePtr).wrapping_add(66), 6, 2, (2u8) as i32);
        crate::c::bf_write((healthBarSpritePtr).wrapping_add(5), 2, 2, (1u16) as i32);
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            GetHealthboxElementGfxPtr(1u8),
                            (((100728832i32).wrapping_add(
                                ((crate::c::bf_read(
                                    (healthBarSpritePtr).wrapping_add(4),
                                    0,
                                    10,
                                    false,
                                ) as u16) as i32)
                                    .wrapping_mul(crate::c::div_i32(256i32, 8i32)),
                            )) as usize as *mut u8),
                            ((67108864i32
                                | (crate::c::div_i32(64i32, crate::c::div_i32(32i32, 8i32))
                                    & 2097151i32)) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthboxLeftSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(((healthbarSpriteId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthboxLeftSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((battler) as i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxLeftSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        ((((healthBarSpritePtr).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((healthboxLeftSpriteId) as i16));
        ((((healthBarSpritePtr).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(data6);
        crate::c::bf_write((healthBarSpritePtr).wrapping_add(62), 2, 1, (1u16) as i32);
        return healthboxLeftSpriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateSafariPlayerHealthboxSprites() -> u8 {
    unsafe {
        let mut healthboxLeftSpriteId: u8 = 0u8;
        let mut healthboxRightSpriteId: u8 = 0u8;
        healthboxLeftSpriteId = CreateSprite(
            (&raw const sHealthboxSafariSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            240i16,
            160i16,
            1u8,
        );
        healthboxRightSpriteId = CreateSpriteAtEnd(
            (&raw const sHealthboxSafariSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            240i16,
            160i16,
            1u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxLeftSpriteId) as i32) as isize * 68))
            .wrapping_add(1),
            6,
            2,
            (0u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
            .wrapping_add(1),
            6,
            2,
            (0u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
            .wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
                .wrapping_add(4),
                0,
                10,
                false,
            ) as u16) as i32)
                .wrapping_add(64i32)) as u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthboxLeftSpriteId) as i32) as isize * 68))
        .wrapping_add(6)
        .cast::<u16>())
        .write(((healthboxRightSpriteId) as u16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(((healthboxLeftSpriteId) as i16));
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_HealthBoxOther));
        return healthboxLeftSpriteId;
    }
}
pub(crate) unsafe extern "C" fn GetHealthboxElementGfxPtr(elementId: u8) -> *mut u8 {
    unsafe {
        let mut elementId = elementId;
        return (((&raw mut gHealthboxElementsGfxTable).cast::<u8>())
            .wrapping_offset(((elementId) as i32) as isize * 32))
        .cast::<u8>();
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_HealthBar(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut healthboxSpriteId: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as u8);
        'l1: {
            let __sw1 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                    .wrapping_add(32)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(16i32)) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                    .wrapping_add(34)
                    .cast::<i16>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                    .wrapping_add(32)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(16i32)) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                    .wrapping_add(34)
                    .cast::<i16>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 2i32 || !__matched {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                    .wrapping_add(32)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(8i32)) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                    .wrapping_add(34)
                    .cast::<i16>())
                    .read(),
                );
                break 'l1;
            }
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .read(),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_HealthBoxOther(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut healthboxMainSpriteId: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as u8);
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxMainSpriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(64i32)) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxMainSpriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .read(),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxMainSpriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .read(),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxMainSpriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattleBarStruct(
    battler: u8,
    healthboxSpriteId: u8,
    maxVal: i32,
    oldVal: i32,
    receivedValue: i32,
) {
    unsafe {
        let mut battler = battler;
        let mut healthboxSpriteId = healthboxSpriteId;
        let mut maxVal = maxVal;
        let mut oldVal = oldVal;
        let mut receivedValue = receivedValue;
        ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((battler) as i32) as isize * 20))
        .write(healthboxSpriteId);
        (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((battler) as i32) as isize * 20))
        .wrapping_add(4)
        .cast::<i32>())
        .write(maxVal);
        (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((battler) as i32) as isize * 20))
        .wrapping_add(8)
        .cast::<i32>())
        .write(oldVal);
        (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((battler) as i32) as isize * 20))
        .wrapping_add(12)
        .cast::<i32>())
        .write(receivedValue);
        (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((battler) as i32) as isize * 20))
        .wrapping_add(16)
        .cast::<i32>())
        .write((-32768i32));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetHealthboxSpriteInvisible(healthboxSpriteId: u8) {
    unsafe {
        let mut healthboxSpriteId = healthboxSpriteId;
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                .wrapping_add(6)
                .cast::<u16>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetHealthboxSpriteVisible(healthboxSpriteId: u8) {
    unsafe {
        let mut healthboxSpriteId = healthboxSpriteId;
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                .wrapping_add(6)
                .cast::<u16>())
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateSpritePos(spriteId: u8, x: i16, y: i16) {
    unsafe {
        let mut spriteId = spriteId;
        let mut x = x;
        let mut y = y;
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
        .write(x);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
        .write(y);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestoryHealthboxSprite(healthboxSpriteId: u8) {
    unsafe {
        let mut healthboxSpriteId = healthboxSpriteId;
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                .wrapping_add(6)
                .cast::<u16>())
                .read()) as i32) as isize
                    * 68,
            ),
        );
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as i32) as isize
                    * 68,
            ),
        );
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DummyBattleInterfaceFunc(
    healthboxSpriteId: u8,
    isDoubleBattleBattlerOnly: u8,
) {
    unsafe {
        let mut healthboxSpriteId = healthboxSpriteId;
        let mut isDoubleBattleBattlerOnly = isDoubleBattleBattlerOnly;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateOamPriorityInAllHealthboxes(priority: u8) {
    unsafe {
        let mut priority = priority;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut healthboxLeftSpriteId: u8 = (((&raw mut gHealthboxSpriteIds)
                        .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .read();
                    let mut healthboxRightSpriteId: u8 = ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .read()) as u8);
                    let mut healthbarSpriteId: u8 = ((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as u8);
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((healthboxLeftSpriteId) as i32) as isize * 68))
                        .wrapping_add(5),
                        2,
                        2,
                        ((priority) as u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((healthboxRightSpriteId) as i32) as isize * 68))
                        .wrapping_add(5),
                        2,
                        2,
                        ((priority) as u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((healthbarSpriteId) as i32) as isize * 68))
                        .wrapping_add(5),
                        2,
                        2,
                        ((priority) as u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitBattlerHealthboxCoords(battler: u8) {
    unsafe {
        let mut battler = battler;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        if !((IsDoubleBattle()) != 0) {
            if ((GetBattlerSide(battler)) as i32) != 0i32 {
                x = 44i16;
                y = 30i16;
            } else {
                x = 158i16;
                y = 88i16;
            }
        } else {
            'l1: {
                let __sw1 = ((GetBattlerPosition(battler)) as i32);
                if __sw1 == 0i32 {
                    x = 159i16;
                    y = 76i16;
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    x = 171i16;
                    y = 101i16;
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    x = 44i16;
                    y = 19i16;
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    x = 32i16;
                    y = 44i16;
                    break 'l1;
                }
            }
        }
        UpdateSpritePos(
            (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read(),
            x,
            y,
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateLvlInHealthbox(healthboxSpriteId: u8, lvl: u8) {
    unsafe {
        let mut healthboxSpriteId = healthboxSpriteId;
        let mut lvl = lvl;
        let mut windowId: u32 = 0u32;
        let mut spriteTileNum: u32 = 0u32;
        let mut windowTileData: *mut u8 = core::ptr::null_mut();
        let mut text = crate::ffi::Align4([0u8; 16]);
        let mut xPos: u32 = 0u32;
        let mut objVram: *mut u8 = core::ptr::null_mut();
        ((&raw mut text).cast::<u8>()).write(249u8);
        (((&raw mut text).cast::<u8>()).wrapping_offset(1)).write(5u8);
        objVram = ConvertIntToDecimalStringN(
            ((&raw mut text).cast::<u8>()).wrapping_offset(2),
            ((lvl) as i32),
            0i32,
            3u8,
        );
        xPos = (((5i32).wrapping_mul(
            (3i32).wrapping_sub(
                (((objVram) as usize)
                    .wrapping_sub((((&raw mut text).cast::<u8>()).wrapping_offset(2)) as usize)
                    as i32
                    / 1),
            ),
        )) as u32);
        windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
            (&raw mut text).cast::<u8>(),
            xPos,
            3u32,
            2u32,
            &raw mut windowId,
        );
        spriteTileNum = ((((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(4),
            0,
            10,
            false,
        ) as u16) as i32)
            .wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u32);
        if ((GetBattlerSide(
            ((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as u8),
        )) as i32)
            == 0i32
        {
            objVram = ((100728832i32) as usize as *mut u8);
            if !((IsDoubleBattle()) != 0) {
                objVram = (objVram)
                    .wrapping_offset((((spriteTileNum).wrapping_add(2080u32)) as i32) as isize);
            } else {
                objVram = (objVram)
                    .wrapping_offset((((spriteTileNum).wrapping_add(1056u32)) as i32) as isize);
            }
        } else {
            objVram = ((100728832i32) as usize as *mut u8);
            objVram = (objVram)
                .wrapping_offset((((spriteTileNum).wrapping_add(1024u32)) as i32) as isize);
        }
        TextIntoHealthboxObject(objVram, windowTileData, 3i32);
        RemoveWindowOnHealthbox(windowId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateHpTextInHealthbox(
    healthboxSpriteId: u8,
    value: i16,
    maxOrCurrent: u8,
) {
    unsafe {
        let mut healthboxSpriteId = healthboxSpriteId;
        let mut value = value;
        let mut maxOrCurrent = maxOrCurrent;
        let mut windowId: u32 = 0u32;
        let mut spriteTileNum: u32 = 0u32;
        let mut windowTileData: *mut u8 = core::ptr::null_mut();
        let mut text = crate::ffi::Align4([0u8; 32]);
        let mut objVram: *mut u8 = core::ptr::null_mut();
        if (((GetBattlerSide(
            ((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as u8),
        )) as i32)
            == 0i32)
            && (!((IsDoubleBattle()) != 0))
        {
            spriteTileNum = ((((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                .wrapping_add(4),
                0,
                10,
                false,
            ) as u16) as i32)
                .wrapping_mul(crate::c::div_i32(256i32, 8i32)))
                as u32);
            if ((maxOrCurrent) as i32) != 0i32 {
                ConvertIntToDecimalStringN(
                    (&raw mut text).cast::<u8>(),
                    ((value) as i32),
                    1i32,
                    3u8,
                );
                windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
                    (&raw mut text).cast::<u8>(),
                    0u32,
                    5u32,
                    2u32,
                    &raw mut windowId,
                );
                objVram = ((100728832i32) as usize as *mut u8);
                objVram = (objVram)
                    .wrapping_offset((((spriteTileNum).wrapping_add(2880u32)) as i32) as isize * 1);
                HpTextIntoHealthboxObject(objVram, windowTileData, 2u32);
                RemoveWindowOnHealthbox(windowId);
            } else {
                ConvertIntToDecimalStringN(
                    (&raw mut text).cast::<u8>(),
                    ((value) as i32),
                    1i32,
                    3u8,
                );
                (((&raw mut text).cast::<u8>()).wrapping_offset(3)).write(186u8);
                (((&raw mut text).cast::<u8>()).wrapping_offset(4)).write(255u8);
                windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
                    (&raw mut text).cast::<u8>(),
                    4u32,
                    5u32,
                    2u32,
                    &raw mut windowId,
                );
                objVram = ((100728832i32) as usize as *mut u8);
                objVram = (objVram)
                    .wrapping_offset((((spriteTileNum).wrapping_add(992u32)) as i32) as isize * 1);
                HpTextIntoHealthboxObject(objVram, windowTileData, 1u32);
                objVram = ((100728832i32) as usize as *mut u8);
                objVram = (objVram)
                    .wrapping_offset((((spriteTileNum).wrapping_add(2816u32)) as i32) as isize * 1);
                HpTextIntoHealthboxObject(objVram, (windowTileData).wrapping_offset(32), 2u32);
                RemoveWindowOnHealthbox(windowId);
            }
        } else {
            let mut battler: u8 = 0u8;
            crate::c::memcpy(
                (&raw mut text).cast::<u8>(),
                ((&raw const sEmptyWhiteText_GrayHighlight)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
                20u32,
            );
            battler = ((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as u8);
            if (((IsDoubleBattle()) as i32) == 1i32) || (((GetBattlerSide(battler)) as i32) == 1i32)
            {
                UpdateHpTextInHealthboxInDoubles(healthboxSpriteId, value, maxOrCurrent);
            } else {
                let mut var: u32 = 0u32;
                let mut i: u8 = 0u8;
                if ((GetBattlerSide(
                    ((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as u8),
                )) as i32)
                    == 0i32
                {
                    if ((maxOrCurrent) as i32) == 0i32 {
                        var = 29u32;
                    } else {
                        var = 89u32;
                    }
                } else {
                    if ((maxOrCurrent) as i32) == 0i32 {
                        var = 20u32;
                    } else {
                        var = 48u32;
                    }
                }
                ConvertIntToDecimalStringN(
                    ((&raw mut text).cast::<u8>()).wrapping_offset(6),
                    ((value) as i32),
                    1i32,
                    3u8,
                );
                RenderTextHandleBold(
                    ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                        .wrapping_add(372)
                        .cast::<*mut u8>())
                    .read(),
                    9u8,
                    (&raw mut text).cast::<u8>(),
                );
                {
                    i = 0u8;
                    'l1: loop {
                        if !(((i) as i32) < 3i32) {
                            break 'l1;
                        }
                        'l2: {
                            'l3: loop {
                                'l4: {
                                    'l5: loop {
                                        'l6: {
                                            CpuSet(
                                                (((((&raw mut gMonSpritesGfxPtr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(372)
                                                .cast::<*mut u8>())
                                                .read())
                                                .wrapping_offset(
                                                    ((((i) as i32).wrapping_mul(64i32))
                                                        .wrapping_add(32i32))
                                                        as isize,
                                                ),
                                                (((100728832u32).wrapping_add(
                                                    ((crate::c::div_i32(256i32, 8i32)) as u32)
                                                        .wrapping_mul(
                                                            (((crate::c::bf_read(
                                                                (((&raw mut gSprites)
                                                                    .cast::<u8>())
                                                                .wrapping_offset(
                                                                    ((healthboxSpriteId) as i32)
                                                                        as isize
                                                                        * 68,
                                                                ))
                                                                .wrapping_add(4),
                                                                0,
                                                                10,
                                                                false,
                                                            )
                                                                as u16)
                                                                as u32)
                                                                .wrapping_add(var))
                                                            .wrapping_add(((i) as u32)),
                                                        ),
                                                ))
                                                    as usize
                                                    as *mut u8),
                                                ((67108864i32
                                                    | (crate::c::div_i32(
                                                        32i32,
                                                        crate::c::div_i32(32i32, 8i32),
                                                    ) & 2097151i32))
                                                    as u32),
                                            );
                                        }
                                        if !((0i32) != 0) {
                                            break 'l5;
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l3;
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateHpTextInHealthboxInDoubles(
    healthboxSpriteId: u8,
    value: i16,
    maxOrCurrent: u8,
) {
    unsafe {
        let mut healthboxSpriteId = healthboxSpriteId;
        let mut value = value;
        let mut maxOrCurrent = maxOrCurrent;
        let mut windowId: u32 = 0u32;
        let mut spriteTileNum: u32 = 0u32;
        let mut windowTileData: *mut u8 = core::ptr::null_mut();
        let mut text = crate::ffi::Align4([0u8; 32]);
        let mut objVram: *mut u8 = core::ptr::null_mut();
        if ((GetBattlerSide(
            ((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as u8),
        )) as i32)
            == 0i32
        {
            if (crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32) as isize
                        * 4,
                ))
                .wrapping_add(0),
                4,
                1,
                false,
            ) as u16)
                != 0
            {
                spriteTileNum = ((((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(4),
                    0,
                    10,
                    false,
                ) as u16) as i32)
                    .wrapping_mul(crate::c::div_i32(256i32, 8i32)))
                    as u32);
                objVram = ((100728832i32) as usize as *mut u8)
                    .wrapping_offset(((spriteTileNum) as i32) as isize * 1);
                if ((maxOrCurrent) as i32) != 0i32 {
                    ConvertIntToDecimalStringN(
                        (&raw mut text).cast::<u8>(),
                        ((value) as i32),
                        1i32,
                        3u8,
                    );
                    windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
                        (&raw mut text).cast::<u8>(),
                        0u32,
                        5u32,
                        0u32,
                        &raw mut windowId,
                    );
                    HpTextIntoHealthboxObject(
                        (((100728832i32) as usize as *mut u8)
                            .wrapping_offset(((spriteTileNum) as i32) as isize * 1))
                        .wrapping_offset(192),
                        windowTileData,
                        2u32,
                    );
                    RemoveWindowOnHealthbox(windowId);
                    'l1: loop {
                        'l2: {
                            'l3: loop {
                                'l4: {
                                    CpuSet(
                                        GetHealthboxElementGfxPtr(116u8),
                                        ((100730496i32) as usize as *mut u8).wrapping_offset(
                                            (((crate::c::bf_read(
                                                (((&raw mut gSprites).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((healthboxSpriteId) as i32) as isize * 68,
                                                    ))
                                                .wrapping_add(4),
                                                0,
                                                10,
                                                false,
                                            ) as u16)
                                                as i32)
                                                .wrapping_mul(crate::c::div_i32(256i32, 8i32)))
                                                as isize
                                                * 1,
                                        ),
                                        ((67108864i32
                                            | (crate::c::div_i32(
                                                32i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l3;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l1;
                        }
                    }
                } else {
                    ConvertIntToDecimalStringN(
                        (&raw mut text).cast::<u8>(),
                        ((value) as i32),
                        1i32,
                        3u8,
                    );
                    (((&raw mut text).cast::<u8>()).wrapping_offset(3)).write(186u8);
                    (((&raw mut text).cast::<u8>()).wrapping_offset(4)).write(255u8);
                    windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
                        (&raw mut text).cast::<u8>(),
                        4u32,
                        5u32,
                        0u32,
                        &raw mut windowId,
                    );
                    FillHealthboxObject(objVram, 0u32, 3u32);
                    HpTextIntoHealthboxObject(
                        ((100728928i32) as usize as *mut u8)
                            .wrapping_offset(((spriteTileNum) as i32) as isize * 1),
                        windowTileData,
                        3u32,
                    );
                    RemoveWindowOnHealthbox(windowId);
                }
            }
        } else {
            let mut battler: u8 = 0u8;
            crate::c::memcpy(
                (&raw mut text).cast::<u8>(),
                ((&raw const sEmptyWhiteText_TransparentHighlight)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
                20u32,
            );
            battler = ((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as u8);
            if (crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 4))
                .wrapping_add(0),
                4,
                1,
                false,
            ) as u16)
                != 0
            {
                let mut var: u8 = 4u8;
                let mut r7: u8 = 0u8;
                let mut txtPtr: *mut u8 = core::ptr::null_mut();
                let mut i: u8 = 0u8;
                if ((maxOrCurrent) as i32) == 0i32 {
                    var = 0u8;
                }
                r7 = ((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as u8);
                txtPtr = ConvertIntToDecimalStringN(
                    ((&raw mut text).cast::<u8>()).wrapping_offset(6),
                    ((value) as i32),
                    1i32,
                    3u8,
                );
                if !((maxOrCurrent) != 0) {
                    StringCopy(txtPtr, (&raw mut gText_Slash).cast::<u8>());
                }
                RenderTextHandleBold(
                    ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                        .wrapping_add(372)
                        .cast::<*mut u8>())
                    .read(),
                    9u8,
                    (&raw mut text).cast::<u8>(),
                );
                {
                    i = var;
                    'l5: loop {
                        if !(((i) as i32) < ((var) as i32).wrapping_add(3i32)) {
                            break 'l5;
                        }
                        'l6: {
                            if ((i) as i32) < 3i32 {
                                'l7: loop {
                                    'l8: {
                                        'l9: loop {
                                            'l10: {
                                                CpuSet(
                                                    (((((&raw mut gMonSpritesGfxPtr)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(372)
                                                    .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_offset(
                                                        (((((i) as i32)
                                                            .wrapping_sub(((var) as i32)))
                                                        .wrapping_mul(64i32))
                                                        .wrapping_add(32i32))
                                                            as isize,
                                                    ),
                                                    (((100728832i32).wrapping_add(
                                                        (32i32).wrapping_mul(
                                                            ((1i32).wrapping_add(
                                                                ((crate::c::bf_read(
                                                                    (((&raw mut gSprites)
                                                                        .cast::<u8>())
                                                                    .wrapping_offset(
                                                                        ((r7) as i32) as isize * 68,
                                                                    ))
                                                                    .wrapping_add(4),
                                                                    0,
                                                                    10,
                                                                    false,
                                                                )
                                                                    as u16)
                                                                    as i32),
                                                            ))
                                                            .wrapping_add(((i) as i32)),
                                                        ),
                                                    ))
                                                        as usize
                                                        as *mut u8),
                                                    ((67108864i32
                                                        | (crate::c::div_i32(
                                                            32i32,
                                                            crate::c::div_i32(32i32, 8i32),
                                                        ) & 2097151i32))
                                                        as u32),
                                                );
                                            }
                                            if !((0i32) != 0) {
                                                break 'l9;
                                            }
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l7;
                                    }
                                }
                            } else {
                                'l11: loop {
                                    'l12: {
                                        'l13: loop {
                                            'l14: {
                                                CpuSet(
                                                    (((((&raw mut gMonSpritesGfxPtr)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(372)
                                                    .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_offset(
                                                        (((((i) as i32)
                                                            .wrapping_sub(((var) as i32)))
                                                        .wrapping_mul(64i32))
                                                        .wrapping_add(32i32))
                                                            as isize,
                                                    ),
                                                    (((100728864i32).wrapping_add(
                                                        (32i32).wrapping_mul(
                                                            ((i) as i32).wrapping_add(
                                                                ((crate::c::bf_read(
                                                                    (((&raw mut gSprites)
                                                                        .cast::<u8>())
                                                                    .wrapping_offset(
                                                                        ((r7) as i32) as isize * 68,
                                                                    ))
                                                                    .wrapping_add(4),
                                                                    0,
                                                                    10,
                                                                    false,
                                                                )
                                                                    as u16)
                                                                    as i32),
                                                            ),
                                                        ),
                                                    ))
                                                        as usize
                                                        as *mut u8),
                                                    ((67108864i32
                                                        | (crate::c::div_i32(
                                                            32i32,
                                                            crate::c::div_i32(32i32, 8i32),
                                                        ) & 2097151i32))
                                                        as u32),
                                                );
                                            }
                                            if !((0i32) != 0) {
                                                break 'l13;
                                            }
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l11;
                                    }
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((maxOrCurrent) as i32) == 0i32 {
                    'l15: loop {
                        'l16: {
                            'l17: loop {
                                'l18: {
                                    CpuSet(
                                        (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(372)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(224),
                                        (((100728832i32).wrapping_add(
                                            (((crate::c::bf_read(
                                                (((&raw mut gSprites).cast::<u8>())
                                                    .wrapping_offset(((r7) as i32) as isize * 68))
                                                .wrapping_add(4),
                                                0,
                                                10,
                                                false,
                                            ) as u16)
                                                as i32)
                                                .wrapping_add(4i32))
                                            .wrapping_mul(crate::c::div_i32(256i32, 8i32)),
                                        )) as usize
                                            as *mut u8),
                                        ((67108864i32
                                            | (crate::c::div_i32(
                                                32i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l17;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l15;
                        }
                    }
                    'l19: loop {
                        'l20: {
                            {
                                let mut tmp: u32 = 0u32;
                                (&raw mut tmp).write_volatile(0u32);
                                'l21: loop {
                                    'l22: {
                                        CpuSet(
                                            (&raw mut tmp).cast::<u8>(),
                                            (((100728832i32).wrapping_add(
                                                ((crate::c::bf_read(
                                                    (((&raw mut gSprites).cast::<u8>())
                                                        .wrapping_offset(
                                                            ((r7) as i32) as isize * 68,
                                                        ))
                                                    .wrapping_add(4),
                                                    0,
                                                    10,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    .wrapping_mul(crate::c::div_i32(256i32, 8i32)),
                                            )) as usize
                                                as *mut u8),
                                            ((83886080i32
                                                | (crate::c::div_i32(
                                                    32i32,
                                                    crate::c::div_i32(32i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l21;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l19;
                        }
                    }
                } else {
                    if ((GetBattlerSide(battler)) as i32) == 0i32 {
                        'l23: loop {
                            'l24: {
                                'l25: loop {
                                    'l26: {
                                        CpuSet(
                                            GetHealthboxElementGfxPtr(116u8),
                                            ((100728832i32) as usize as *mut u8).wrapping_offset(
                                                ((((crate::c::bf_read(
                                                    (((&raw mut gSprites).cast::<u8>())
                                                        .wrapping_offset(
                                                            ((healthboxSpriteId) as i32) as isize
                                                                * 68,
                                                        ))
                                                    .wrapping_add(4),
                                                    0,
                                                    10,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    .wrapping_add(52i32))
                                                .wrapping_mul(crate::c::div_i32(256i32, 8i32)))
                                                    as isize
                                                    * 1,
                                            ),
                                            ((67108864i32
                                                | (crate::c::div_i32(
                                                    32i32,
                                                    crate::c::div_i32(32i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l25;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l23;
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintSafariMonInfo(healthboxSpriteId: u8, mon: *mut u8) {
    unsafe {
        let mut healthboxSpriteId = healthboxSpriteId;
        let mut mon = mon;
        let mut text = crate::ffi::Align4([0u8; 20]);
        let mut j: i32 = 0i32;
        let mut spriteTileNum: i32 = 0i32;
        let mut barFontGfx: *mut u8 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        let mut var: u8 = 0u8;
        let mut nature: u8 = 0u8;
        let mut healthBarSpriteId: u8 = 0u8;
        crate::c::memcpy(
            (&raw mut text).cast::<u8>(),
            ((&raw const sEmptyWhiteText_GrayHighlight)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            20u32,
        );
        barFontGfx = (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
            .wrapping_add(372)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(
            ((1312i32).wrapping_add(
                ((GetBattlerPosition(
                    ((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as u8),
                )) as i32)
                    .wrapping_mul(384i32),
            )) as isize,
        );
        var = 5u8;
        nature = GetNature(mon);
        StringCopy(
            ((&raw mut text).cast::<u8>()).wrapping_offset(6),
            ((((&raw mut gNatureNamePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(((nature) as i32) as isize))
            .read(),
        );
        RenderTextHandleBold(barFontGfx, 9u8, (&raw mut text).cast::<u8>());
        {
            j = 6i32;
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((var) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut elementId: u8 = 0u8;
                    if (((((((&raw mut text).cast::<u8>()).wrapping_offset((j) as isize)).read())
                        as i32)
                        >= 55i32)
                        && ((((((&raw mut text).cast::<u8>()).wrapping_offset((j) as isize)).read())
                            as i32)
                            <= 74i32))
                        || (((((((&raw mut text).cast::<u8>()).wrapping_offset((j) as isize))
                            .read()) as i32)
                            >= 135i32)
                            && ((((((&raw mut text).cast::<u8>()).wrapping_offset((j) as isize))
                                .read()) as i32)
                                <= 154i32))
                    {
                        elementId = 44u8;
                    } else {
                        if (((((((&raw mut text).cast::<u8>()).wrapping_offset((j) as isize))
                            .read()) as i32)
                            >= 75i32)
                            && ((((((&raw mut text).cast::<u8>()).wrapping_offset((j) as isize))
                                .read()) as i32)
                                <= 79i32))
                            || (((((((&raw mut text).cast::<u8>()).wrapping_offset((j) as isize))
                                .read()) as i32)
                                >= 155i32)
                                && ((((((&raw mut text).cast::<u8>())
                                    .wrapping_offset((j) as isize))
                                .read()) as i32)
                                    <= 159i32))
                        {
                            elementId = 45u8;
                        } else {
                            elementId = 43u8;
                        }
                    }
                    'l3: loop {
                        'l4: {
                            'l5: loop {
                                'l6: {
                                    CpuSet(
                                        GetHealthboxElementGfxPtr(elementId),
                                        (barFontGfx).wrapping_offset(
                                            (((i) as i32).wrapping_mul(64i32)) as isize,
                                        ),
                                        ((67108864i32
                                            | (crate::c::div_i32(
                                                32i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l5;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
                i = (i).wrapping_add(1);
                j = (j).wrapping_add(1);
            }
        }
        {
            j = 1i32;
            'l7: loop {
                if !(j < ((var) as i32).wrapping_add(1i32)) {
                    break 'l7;
                }
                'l8: {
                    spriteTileNum = ((((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                        .wrapping_add(4),
                        0,
                        10,
                        false,
                    ) as u16) as i32)
                        .wrapping_add(crate::c::rem_i32(j, 8i32)))
                    .wrapping_add((crate::c::div_i32(j, 8i32)).wrapping_mul(64i32)))
                    .wrapping_mul(crate::c::div_i32(256i32, 8i32));
                    'l9: loop {
                        'l10: {
                            'l11: loop {
                                'l12: {
                                    CpuSet(
                                        barFontGfx,
                                        ((100728832i32) as usize as *mut u8)
                                            .wrapping_offset((spriteTileNum) as isize * 1),
                                        ((67108864i32
                                            | (crate::c::div_i32(
                                                32i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l11;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l9;
                        }
                    }
                    barFontGfx = (barFontGfx).wrapping_offset(32);
                    spriteTileNum = ((((8i32).wrapping_add(
                        ((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
                            .wrapping_add(4),
                            0,
                            10,
                            false,
                        ) as u16) as i32),
                    ))
                    .wrapping_add(crate::c::rem_i32(j, 8i32)))
                    .wrapping_add((crate::c::div_i32(j, 8i32)).wrapping_mul(64i32)))
                    .wrapping_mul(crate::c::div_i32(256i32, 8i32));
                    'l13: loop {
                        'l14: {
                            'l15: loop {
                                'l16: {
                                    CpuSet(
                                        barFontGfx,
                                        ((100728832i32) as usize as *mut u8)
                                            .wrapping_offset((spriteTileNum) as isize * 1),
                                        ((67108864i32
                                            | (crate::c::div_i32(
                                                32i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l15;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l13;
                        }
                    }
                    barFontGfx = (barFontGfx).wrapping_offset(32);
                }
                j = (j).wrapping_add(1);
            }
        }
        healthBarSpriteId = ((((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .read()) as u8);
        ConvertIntToDecimalStringN(
            ((&raw mut text).cast::<u8>()).wrapping_offset(6),
            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(124)).read())
                as i32),
            1i32,
            2u8,
        );
        ConvertIntToDecimalStringN(
            ((&raw mut text).cast::<u8>()).wrapping_offset(9),
            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(123)).read())
                as i32),
            1i32,
            2u8,
        );
        (((&raw mut text).cast::<u8>()).wrapping_offset(5)).write(0u8);
        (((&raw mut text).cast::<u8>()).wrapping_offset(8)).write(186u8);
        RenderTextHandleBold(
            ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                .wrapping_add(372)
                .cast::<*mut u8>())
            .read(),
            9u8,
            (&raw mut text).cast::<u8>(),
        );
        j = ((healthBarSpriteId) as i32);
        {
            j = 0i32;
            'l17: loop {
                if !(j < 5i32) {
                    break 'l17;
                }
                'l18: {
                    if j <= 1i32 {
                        'l19: loop {
                            'l20: {
                                'l21: loop {
                                    'l22: {
                                        CpuSet(
                                            (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(372)
                                            .cast::<*mut u8>())
                                            .read())
                                            .wrapping_offset(
                                                (((64i32).wrapping_mul(j)).wrapping_add(32i32))
                                                    as isize,
                                            ),
                                            ((100728832i32) as usize as *mut u8).wrapping_offset(
                                                (((((crate::c::bf_read(
                                                    (((&raw mut gSprites).cast::<u8>())
                                                        .wrapping_offset(
                                                            ((healthBarSpriteId) as i32) as isize
                                                                * 68,
                                                        ))
                                                    .wrapping_add(4),
                                                    0,
                                                    10,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    .wrapping_add(2i32))
                                                .wrapping_add(j))
                                                .wrapping_mul(crate::c::div_i32(256i32, 8i32)))
                                                    as isize
                                                    * 1,
                                            ),
                                            ((67108864i32
                                                | (crate::c::div_i32(
                                                    32i32,
                                                    crate::c::div_i32(32i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l21;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l19;
                            }
                        }
                    } else {
                        'l23: loop {
                            'l24: {
                                'l25: loop {
                                    'l26: {
                                        CpuSet(
                                            (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(372)
                                            .cast::<*mut u8>())
                                            .read())
                                            .wrapping_offset(
                                                (((64i32).wrapping_mul(j)).wrapping_add(32i32))
                                                    as isize,
                                            ),
                                            ((100729024i32) as usize as *mut u8).wrapping_offset(
                                                (((j).wrapping_add(
                                                    ((crate::c::bf_read(
                                                        (((&raw mut gSprites).cast::<u8>())
                                                            .wrapping_offset(
                                                                ((healthBarSpriteId) as i32)
                                                                    as isize
                                                                    * 68,
                                                            ))
                                                        .wrapping_add(4),
                                                        0,
                                                        10,
                                                        false,
                                                    )
                                                        as u16)
                                                        as i32),
                                                ))
                                                .wrapping_mul(crate::c::div_i32(256i32, 8i32)))
                                                    as isize
                                                    * 1,
                                            ),
                                            ((67108864i32
                                                | (crate::c::div_i32(
                                                    32i32,
                                                    crate::c::div_i32(32i32, 8i32),
                                                ) & 2097151i32))
                                                as u32),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l25;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l23;
                            }
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SwapHpBarsWithHpText() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut healthBarSpriteId: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((core::mem::transmute::<_, usize>(
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .read(),
                    ) == (SpriteCallbackDummy as *const () as usize))
                        && (((GetBattlerSide(((i) as u8))) as i32) != 1i32))
                        && (((IsDoubleBattle()) != 0)
                            || (((GetBattlerSide(((i) as u8))) as i32) != 0i32))
                    {
                        let mut noBars: u8 = 0u8;
                        crate::c::bf_write(
                            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 4))
                            .wrapping_add(0),
                            4,
                            1,
                            ((((crate::c::bf_read(
                                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 4))
                                .wrapping_add(0),
                                4,
                                1,
                                false,
                            ) as u16) as i32)
                                ^ 1i32) as u16) as i32,
                        );
                        noBars = ((crate::c::bf_read(
                            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 4))
                            .wrapping_add(0),
                            4,
                            1,
                            false,
                        ) as u16) as u8);
                        if ((GetBattlerSide(((i) as u8))) as i32) == 0i32 {
                            if !((IsDoubleBattle()) != 0) {
                                break 'l2;
                            }
                            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 128u32) != 0 {
                                break 'l2;
                            }
                            if ((noBars) as i32) == 1i32 {
                                healthBarSpriteId =
                                    ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        (((((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(46))
                                    .cast::<i16>())
                                    .wrapping_offset(5))
                                    .read()) as u8);
                                'l3: loop {
                                    'l4: {
                                        {
                                            let mut tmp: u32 = 0u32;
                                            (&raw mut tmp).write_volatile(0u32);
                                            'l5: loop {
                                                'l6: {
                                                    CpuSet(
                                                        (&raw mut tmp).cast::<u8>(),
                                                        (((100728832i32).wrapping_add(
                                                            ((crate::c::bf_read(
                                                                (((&raw mut gSprites)
                                                                    .cast::<u8>())
                                                                .wrapping_offset(
                                                                    ((healthBarSpriteId) as i32)
                                                                        as isize
                                                                        * 68,
                                                                ))
                                                                .wrapping_add(4),
                                                                0,
                                                                10,
                                                                false,
                                                            )
                                                                as u16)
                                                                as i32)
                                                                .wrapping_mul(crate::c::div_i32(
                                                                    256i32, 8i32,
                                                                )),
                                                        ))
                                                            as usize
                                                            as *mut u8),
                                                        ((83886080i32
                                                            | (crate::c::div_i32(
                                                                256i32,
                                                                crate::c::div_i32(32i32, 8i32),
                                                            ) & 2097151i32))
                                                            as u32),
                                                    );
                                                }
                                                if !((0i32) != 0) {
                                                    break 'l5;
                                                }
                                            }
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l3;
                                    }
                                }
                                UpdateHpTextInHealthboxInDoubles(
                                    (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read(),
                                    ((GetMonData2(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 100,
                                        ),
                                        57i32,
                                    )) as i16),
                                    0u8,
                                );
                                UpdateHpTextInHealthboxInDoubles(
                                    (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read(),
                                    ((GetMonData2(
                                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 100,
                                        ),
                                        58i32,
                                    )) as i16),
                                    1u8,
                                );
                            } else {
                                UpdateStatusIconInHealthbox(
                                    (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read(),
                                );
                                UpdateHealthboxAttribute(
                                    (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read(),
                                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 100,
                                    ),
                                    5u8,
                                );
                                'l7: loop {
                                    'l8: {
                                        'l9: loop {
                                            'l10: {
                                                CpuSet(GetHealthboxElementGfxPtr(117u8), (((100730496i32).wrapping_add((((((crate::c::bf_read((((((&raw mut gSprites)).cast::<u8>()).wrapping_offset((((((((&raw mut gHealthboxSpriteIds)).cast::<u8>()).wrapping_offset((i) as isize)).read()) as i32)) as isize * 68))).wrapping_add(4), 0, 10, false) as u16)) as i32))).wrapping_mul(crate::c::div_i32(256i32, 8i32)))) as usize as *mut u8), (((67108864i32 | (crate::c::div_i32(32i32, crate::c::div_i32(32i32, 8i32)) & 2097151i32))) as u32));
                                            }
                                            if !((0i32) != 0) {
                                                break 'l9;
                                            }
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l7;
                                    }
                                }
                            }
                        } else {
                            if ((noBars) as i32) == 1i32 {
                                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 128u32)
                                    != 0
                                {
                                    PrintSafariMonInfo(
                                        (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .read(),
                                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 100,
                                        ),
                                    );
                                } else {
                                    healthBarSpriteId = ((((((((&raw mut gSprites)
                                        .cast::<u8>())
                                    .wrapping_offset(
                                        (((((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(46))
                                    .cast::<i16>())
                                    .wrapping_offset(5))
                                    .read())
                                        as u8);
                                    'l11: loop {
                                        'l12: {
                                            {
                                                let mut tmp: u32 = 0u32;
                                                (&raw mut tmp).write_volatile(0u32);
                                                'l13: loop {
                                                    'l14: {
                                                        CpuSet(
                                                            (&raw mut tmp).cast::<u8>(),
                                                            (((100728832i32).wrapping_add(
                                                                ((crate::c::bf_read(
                                                                    (((&raw mut gSprites)
                                                                        .cast::<u8>())
                                                                    .wrapping_offset(
                                                                        ((healthBarSpriteId) as i32)
                                                                            as isize
                                                                            * 68,
                                                                    ))
                                                                    .wrapping_add(4),
                                                                    0,
                                                                    10,
                                                                    false,
                                                                )
                                                                    as u16)
                                                                    as i32)
                                                                    .wrapping_mul(32i32),
                                                            ))
                                                                as usize
                                                                as *mut u8),
                                                            ((83886080i32
                                                                | (crate::c::div_i32(
                                                                    256i32,
                                                                    crate::c::div_i32(32i32, 8i32),
                                                                ) & 2097151i32))
                                                                as u32),
                                                        );
                                                    }
                                                    if !((0i32) != 0) {
                                                        break 'l13;
                                                    }
                                                }
                                            }
                                        }
                                        if !((0i32) != 0) {
                                            break 'l11;
                                        }
                                    }
                                    UpdateHpTextInHealthboxInDoubles(
                                        (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .read(),
                                        ((GetMonData2(
                                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                                ((((((&raw mut gBattlerPartyIndexes)
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 100,
                                            ),
                                            57i32,
                                        )) as i16),
                                        0u8,
                                    );
                                    UpdateHpTextInHealthboxInDoubles(
                                        (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .read(),
                                        ((GetMonData2(
                                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                                ((((((&raw mut gBattlerPartyIndexes)
                                                    .cast::<u16>())
                                                .cast::<u16>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 100,
                                            ),
                                            58i32,
                                        )) as i16),
                                        1u8,
                                    );
                                }
                            } else {
                                UpdateStatusIconInHealthbox(
                                    (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read(),
                                );
                                UpdateHealthboxAttribute(
                                    (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read(),
                                    ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 100,
                                    ),
                                    5u8,
                                );
                                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 128u32)
                                    != 0
                                {
                                    UpdateHealthboxAttribute(
                                        (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .read(),
                                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 100,
                                        ),
                                        4u8,
                                    );
                                }
                            }
                        }
                        let __p1 = (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(7);
                        (__p1).write((((((__p1).read()) as i32) ^ 1i32) as i16));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreatePartyStatusSummarySprites(
    battler: u8,
    partyInfo: *mut u8,
    skipPlayer: u8,
    isBattleStart: u8,
) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut partyInfo = partyInfo;
        let mut skipPlayer = skipPlayer;
        let mut isBattleStart = isBattleStart;
        let mut isOpponent: u8 = 0u8;
        let mut bar_X: i16 = 0i16;
        let mut bar_Y: i16 = 0i16;
        let mut bar_pos2_X: i16 = 0i16;
        let mut bar_data0: i16 = 0i16;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut var: i32 = 0i32;
        let mut summaryBarSpriteId: u8 = 0u8;
        let mut ballIconSpritesIds = crate::ffi::Align4([0u8; 6]);
        let mut taskId: u8 = 0u8;
        if (!((skipPlayer) != 0)) || (((GetBattlerPosition(battler)) as i32) != 3i32) {
            if ((GetBattlerSide(battler)) as i32) == 0i32 {
                isOpponent = 0u8;
                bar_X = 136i16;
                bar_Y = 96i16;
                bar_pos2_X = 100i16;
                bar_data0 = (-5i16);
            } else {
                isOpponent = 1u8;
                if (!((skipPlayer) != 0)) || (!((IsDoubleBattle()) != 0)) {
                    bar_X = 104i16;
                    bar_Y = 40i16;
                } else {
                    bar_X = 104i16;
                    bar_Y = 16i16;
                }
                bar_pos2_X = (-100i16);
                bar_data0 = 5i16;
            }
        } else {
            isOpponent = 1u8;
            bar_X = 104i16;
            bar_Y = 40i16;
            bar_pos2_X = (-100i16);
            bar_data0 = 5i16;
        }
        LoadCompressedSpriteSheetUsingHeap(
            (&raw const sStatusSummaryBarSpriteSheet)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadSpriteSheet(
            (&raw const sStatusSummaryBallsSpriteSheet)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadSpritePalette(
            (&raw const sStatusSummaryBarSpritePal)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadSpritePalette(
            (&raw const sStatusSummaryBallsSpritePal)
                .cast::<u8>()
                .cast_mut(),
        );
        summaryBarSpriteId = CreateSprite(
            (((&raw const sStatusSummaryBarSpriteTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((isOpponent) as i32) as isize * 24),
            bar_X,
            bar_Y,
            10u8,
        );
        SetSubspriteTables(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((summaryBarSpriteId) as i32) as isize * 68),
            ((&raw const sStatusSummaryBar_SubspriteTable_Enter)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((summaryBarSpriteId) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>())
        .write(bar_pos2_X);
        (((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((summaryBarSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .write(bar_data0);
        if (isOpponent) != 0 {
            let __p1 = (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((summaryBarSpriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(96i32)) as i16));
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((summaryBarSpriteId) as i32) as isize * 68))
                .wrapping_add(3),
                1,
                5,
                (8u32) as i32,
            );
        } else {
            let __p2 = (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((summaryBarSpriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(96i32)) as i16));
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut ballIconSpritesIds).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(CreateSpriteAtEnd(
                            (((&raw const sStatusSummaryBallsSpriteTemplates)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((isOpponent) as i32) as isize * 24),
                            bar_X,
                            ((((bar_Y) as i32).wrapping_sub(4i32)) as i16),
                            9u8,
                        ));
                    if !((isBattleStart) != 0) {
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(SpriteCB_StatusSummaryBalls_OnSwitchout));
                    }
                    if !((isOpponent) != 0) {
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .write(0i16);
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(38)
                        .cast::<i16>())
                        .write(0i16);
                    }
                    (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut ballIconSpritesIds).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(((summaryBarSpriteId) as i16));
                    if !((isOpponent) != 0) {
                        let __p3 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(32)
                        .cast::<i16>();
                        (__p3).write(
                            (((((__p3).read()) as i32)
                                .wrapping_add(((10i32).wrapping_mul(i)).wrapping_add(24i32)))
                                as i16),
                        );
                        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(((((i).wrapping_mul(7i32)).wrapping_add(10i32)) as i16));
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .write(120i16);
                    } else {
                        let __p4 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(32)
                        .cast::<i16>();
                        (__p4).write(
                            (((((__p4).read()) as i32).wrapping_sub(
                                ((10i32).wrapping_mul((5i32).wrapping_sub(i))).wrapping_add(24i32),
                            )) as i16),
                        );
                        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(
                            (((((6i32).wrapping_sub(i)).wrapping_mul(7i32)).wrapping_add(10i32))
                                as i16),
                        );
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .write((-120i16));
                    }
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut ballIconSpritesIds).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(((isOpponent) as i16));
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((GetBattlerSide(battler)) as i32) == 0i32 {
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0 {
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i < 6i32) {
                            break 'l3;
                        }
                        'l4: {
                            if (((((partyInfo).wrapping_offset((i) as isize * 8)).cast::<u16>())
                                .read()) as i32)
                                == 65535i32
                            {
                                crate::c::bf_write(
                                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(4),
                                    0,
                                    10,
                                    ((((crate::c::bf_read(
                                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                        .wrapping_add(4),
                                        0,
                                        10,
                                        false,
                                    ) as u16) as i32)
                                        .wrapping_add(1i32))
                                        as u16) as i32,
                                );
                                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(7))
                                .write(1i16);
                            } else {
                                if (((((partyInfo).wrapping_offset((i) as isize * 8))
                                    .cast::<u16>())
                                .read()) as i32)
                                    == 0i32
                                {
                                    crate::c::bf_write(
                                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                        .wrapping_add(4),
                                        0,
                                        10,
                                        ((((crate::c::bf_read(
                                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(4),
                                            0,
                                            10,
                                            false,
                                        ) as u16)
                                            as i32)
                                            .wrapping_add(3i32))
                                            as u16) as i32,
                                    );
                                } else {
                                    if (((partyInfo).wrapping_offset((i) as isize * 8))
                                        .wrapping_add(4)
                                        .cast::<u32>())
                                    .read()
                                        != 0u32
                                    {
                                        crate::c::bf_write(
                                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(4),
                                            0,
                                            10,
                                            ((((crate::c::bf_read(
                                                (((&raw mut gSprites).cast::<u8>())
                                                    .wrapping_offset(
                                                        (((((&raw mut ballIconSpritesIds)
                                                            .cast::<u8>())
                                                        .wrapping_offset((i) as isize))
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 68,
                                                    ))
                                                .wrapping_add(4),
                                                0,
                                                10,
                                                false,
                                            )
                                                as u16)
                                                as i32)
                                                .wrapping_add(2i32))
                                                as u16)
                                                as i32,
                                        );
                                    }
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            } else {
                {
                    i = 0i32;
                    var = 5i32;
                    j = 0i32;
                    'l5: loop {
                        if !(j < 6i32) {
                            break 'l5;
                        }
                        'l6: {
                            if (((((partyInfo).wrapping_offset((j) as isize * 8)).cast::<u16>())
                                .read()) as i32)
                                == 65535i32
                            {
                                crate::c::bf_write(
                                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                            .wrapping_offset((var) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(4),
                                    0,
                                    10,
                                    ((((crate::c::bf_read(
                                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                .wrapping_offset((var) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                        .wrapping_add(4),
                                        0,
                                        10,
                                        false,
                                    ) as u16) as i32)
                                        .wrapping_add(1i32))
                                        as u16) as i32,
                                );
                                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                        .wrapping_offset((var) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(7))
                                .write(1i16);
                                var = (var).wrapping_sub(1);
                                break 'l6;
                            } else {
                                if (((((partyInfo).wrapping_offset((j) as isize * 8))
                                    .cast::<u16>())
                                .read()) as i32)
                                    == 0i32
                                {
                                    crate::c::bf_write(
                                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                        .wrapping_add(4),
                                        0,
                                        10,
                                        ((((crate::c::bf_read(
                                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(4),
                                            0,
                                            10,
                                            false,
                                        ) as u16)
                                            as i32)
                                            .wrapping_add(3i32))
                                            as u16) as i32,
                                    );
                                } else {
                                    if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                        & 262144u32)
                                        != 0)
                                        && ((((((((&raw mut gBattleStruct).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(672))
                                        .read())
                                            as u32)
                                            & ((((&raw mut gBitTable).cast::<u32>())
                                                .cast::<u32>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                            != 0)
                                    {
                                        crate::c::bf_write(
                                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                    .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(4),
                                            0,
                                            10,
                                            ((((crate::c::bf_read(
                                                (((&raw mut gSprites).cast::<u8>())
                                                    .wrapping_offset(
                                                        (((((&raw mut ballIconSpritesIds)
                                                            .cast::<u8>())
                                                        .wrapping_offset((i) as isize))
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 68,
                                                    ))
                                                .wrapping_add(4),
                                                0,
                                                10,
                                                false,
                                            )
                                                as u16)
                                                as i32)
                                                .wrapping_add(3i32))
                                                as u16)
                                                as i32,
                                        );
                                    } else {
                                        if (((partyInfo).wrapping_offset((j) as isize * 8))
                                            .wrapping_add(4)
                                            .cast::<u32>())
                                        .read()
                                            != 0u32
                                        {
                                            crate::c::bf_write(
                                                (((&raw mut gSprites).cast::<u8>())
                                                    .wrapping_offset(
                                                        (((((&raw mut ballIconSpritesIds)
                                                            .cast::<u8>())
                                                        .wrapping_offset((i) as isize))
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 68,
                                                    ))
                                                .wrapping_add(4),
                                                0,
                                                10,
                                                ((((crate::c::bf_read(
                                                    (((&raw mut gSprites).cast::<u8>())
                                                        .wrapping_offset(
                                                            (((((&raw mut ballIconSpritesIds)
                                                                .cast::<u8>())
                                                            .wrapping_offset((i) as isize))
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 68,
                                                        ))
                                                    .wrapping_add(4),
                                                    0,
                                                    10,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    .wrapping_add(2i32))
                                                    as u16)
                                                    as i32,
                                            );
                                        }
                                    }
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                        j = (j).wrapping_add(1);
                    }
                }
            }
        } else {
            if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 32832u32) != 0 {
                {
                    var = 5i32;
                    i = 0i32;
                    'l7: loop {
                        if !(i < 6i32) {
                            break 'l7;
                        }
                        'l8: {
                            if (((((partyInfo).wrapping_offset((i) as isize * 8)).cast::<u16>())
                                .read()) as i32)
                                == 65535i32
                            {
                                crate::c::bf_write(
                                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                            .wrapping_offset((var) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(4),
                                    0,
                                    10,
                                    ((((crate::c::bf_read(
                                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                .wrapping_offset((var) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                        .wrapping_add(4),
                                        0,
                                        10,
                                        false,
                                    ) as u16) as i32)
                                        .wrapping_add(1i32))
                                        as u16) as i32,
                                );
                                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                        .wrapping_offset((var) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(7))
                                .write(1i16);
                            } else {
                                if (((((partyInfo).wrapping_offset((i) as isize * 8))
                                    .cast::<u16>())
                                .read()) as i32)
                                    == 0i32
                                {
                                    crate::c::bf_write(
                                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                .wrapping_offset((var) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                        .wrapping_add(4),
                                        0,
                                        10,
                                        ((((crate::c::bf_read(
                                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                    .wrapping_offset((var) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(4),
                                            0,
                                            10,
                                            false,
                                        ) as u16)
                                            as i32)
                                            .wrapping_add(3i32))
                                            as u16) as i32,
                                    );
                                } else {
                                    if (((partyInfo).wrapping_offset((i) as isize * 8))
                                        .wrapping_add(4)
                                        .cast::<u32>())
                                    .read()
                                        != 0u32
                                    {
                                        crate::c::bf_write(
                                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                    .wrapping_offset((var) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(4),
                                            0,
                                            10,
                                            ((((crate::c::bf_read(
                                                (((&raw mut gSprites).cast::<u8>())
                                                    .wrapping_offset(
                                                        (((((&raw mut ballIconSpritesIds)
                                                            .cast::<u8>())
                                                        .wrapping_offset((var) as isize))
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 68,
                                                    ))
                                                .wrapping_add(4),
                                                0,
                                                10,
                                                false,
                                            )
                                                as u16)
                                                as i32)
                                                .wrapping_add(2i32))
                                                as u16)
                                                as i32,
                                        );
                                    }
                                }
                            }
                            var = (var).wrapping_sub(1);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            } else {
                {
                    var = 0i32;
                    i = 0i32;
                    j = 0i32;
                    'l9: loop {
                        if !(j < 6i32) {
                            break 'l9;
                        }
                        'l10: {
                            if (((((partyInfo).wrapping_offset((j) as isize * 8)).cast::<u16>())
                                .read()) as i32)
                                == 65535i32
                            {
                                crate::c::bf_write(
                                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                            .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(4),
                                    0,
                                    10,
                                    ((((crate::c::bf_read(
                                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                        .wrapping_add(4),
                                        0,
                                        10,
                                        false,
                                    ) as u16) as i32)
                                        .wrapping_add(1i32))
                                        as u16) as i32,
                                );
                                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(7))
                                .write(1i16);
                                i = (i).wrapping_add(1);
                                break 'l10;
                            } else {
                                if (((((partyInfo).wrapping_offset((j) as isize * 8))
                                    .cast::<u16>())
                                .read()) as i32)
                                    == 0i32
                                {
                                    crate::c::bf_write(
                                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                .wrapping_offset(
                                                    ((5i32).wrapping_sub(var)) as isize,
                                                ))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ))
                                        .wrapping_add(4),
                                        0,
                                        10,
                                        ((((crate::c::bf_read(
                                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((5i32).wrapping_sub(var)) as isize,
                                                    ))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(4),
                                            0,
                                            10,
                                            false,
                                        ) as u16)
                                            as i32)
                                            .wrapping_add(3i32))
                                            as u16) as i32,
                                    );
                                } else {
                                    if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                        & 262144u32)
                                        != 0)
                                        && ((((((((&raw mut gBattleStruct).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(673))
                                        .read())
                                            as u32)
                                            & ((((&raw mut gBitTable).cast::<u32>())
                                                .cast::<u32>())
                                            .wrapping_offset((j) as isize))
                                            .read())
                                            != 0)
                                    {
                                        crate::c::bf_write(
                                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                (((((&raw mut ballIconSpritesIds).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((5i32).wrapping_sub(var)) as isize,
                                                    ))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 68,
                                            ))
                                            .wrapping_add(4),
                                            0,
                                            10,
                                            ((((crate::c::bf_read(
                                                (((&raw mut gSprites).cast::<u8>())
                                                    .wrapping_offset(
                                                        (((((&raw mut ballIconSpritesIds)
                                                            .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((5i32).wrapping_sub(var)) as isize,
                                                        ))
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 68,
                                                    ))
                                                .wrapping_add(4),
                                                0,
                                                10,
                                                false,
                                            )
                                                as u16)
                                                as i32)
                                                .wrapping_add(3i32))
                                                as u16)
                                                as i32,
                                        );
                                    } else {
                                        if (((partyInfo).wrapping_offset((j) as isize * 8))
                                            .wrapping_add(4)
                                            .cast::<u32>())
                                        .read()
                                            != 0u32
                                        {
                                            crate::c::bf_write(
                                                (((&raw mut gSprites).cast::<u8>())
                                                    .wrapping_offset(
                                                        (((((&raw mut ballIconSpritesIds)
                                                            .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((5i32).wrapping_sub(var)) as isize,
                                                        ))
                                                        .read())
                                                            as i32)
                                                            as isize
                                                            * 68,
                                                    ))
                                                .wrapping_add(4),
                                                0,
                                                10,
                                                ((((crate::c::bf_read(
                                                    (((&raw mut gSprites).cast::<u8>())
                                                        .wrapping_offset(
                                                            (((((&raw mut ballIconSpritesIds)
                                                                .cast::<u8>())
                                                            .wrapping_offset(
                                                                ((5i32).wrapping_sub(var)) as isize,
                                                            ))
                                                            .read())
                                                                as i32)
                                                                as isize
                                                                * 68,
                                                        ))
                                                    .wrapping_add(4),
                                                    0,
                                                    10,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    .wrapping_add(2i32))
                                                    as u16)
                                                    as i32,
                                            );
                                        }
                                    }
                                }
                            }
                            var = (var).wrapping_add(1);
                        }
                        j = (j).wrapping_add(1);
                    }
                }
            }
        }
        taskId = CreateTask(Some(TaskDummy), 5u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((battler) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((summaryBarSpriteId) as i16));
        {
            i = 0i32;
            'l11: loop {
                if !(i < 6i32) {
                    break 'l11;
                }
                'l12: {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(((3i32).wrapping_add(i)) as isize))
                    .write(
                        (((((&raw mut ballIconSpritesIds).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(((isBattleStart) as i16));
        if (isBattleStart) != 0 {
            crate::c::bf_write(
                (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(9),
                2,
                3,
                ((crate::c::bf_read(
                    (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(9),
                    2,
                    3,
                    false,
                ) as u8)
                    .wrapping_add(1)) as i32,
            );
        }
        PlaySE12WithPanning(114u16, 0i8);
        return taskId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_HidePartyStatusSummary(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut ballIconSpriteIds = crate::ffi::Align4([0u8; 6]);
        let mut isBattleStart: u8 = 0u8;
        let mut summaryBarSpriteId: u8 = 0u8;
        let mut battler: u8 = 0u8;
        let mut i: i32 = 0i32;
        isBattleStart = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .read()) as u8);
        summaryBarSpriteId = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        battler = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut ballIconSpriteIds).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(((3i32).wrapping_add(i)) as isize))
                            .read()) as u8),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        SetGpuReg(80u8, 16192u16);
        SetGpuReg(82u8, 16u16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(16i16);
        {
            i = 0i32;
            'l3: loop {
                if !(i < 6i32) {
                    break 'l3;
                }
                'l4: {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut ballIconSpriteIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(1),
                        2,
                        2,
                        (1u32) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((summaryBarSpriteId) as i32) as isize * 68))
            .wrapping_add(1),
            2,
            2,
            (1u32) as i32,
        );
        if (isBattleStart) != 0 {
            {
                i = 0i32;
                'l5: loop {
                    if !(i < 6i32) {
                        break 'l5;
                    }
                    'l6: {
                        if ((GetBattlerSide(battler)) as i32) != 0i32 {
                            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut ballIconSpriteIds).cast::<u8>())
                                    .wrapping_offset(((5i32).wrapping_sub(i)) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .write((((7i32).wrapping_mul(i)) as i16));
                            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut ballIconSpriteIds).cast::<u8>())
                                    .wrapping_offset(((5i32).wrapping_sub(i)) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(3))
                            .write(0i16);
                            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut ballIconSpriteIds).cast::<u8>())
                                    .wrapping_offset(((5i32).wrapping_sub(i)) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .write(0i16);
                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut ballIconSpriteIds).cast::<u8>())
                                    .wrapping_offset(((5i32).wrapping_sub(i)) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                            .write(Some(SpriteCB_StatusSummaryBalls_Exit));
                        } else {
                            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut ballIconSpriteIds).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .write((((7i32).wrapping_mul(i)) as i16));
                            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut ballIconSpriteIds).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(3))
                            .write(0i16);
                            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut ballIconSpriteIds).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .write(0i16);
                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut ballIconSpriteIds).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                            .write(Some(SpriteCB_StatusSummaryBalls_Exit));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            let __p1 = ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((summaryBarSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>();
            (__p1).write(((crate::c::div_i32((((__p1).read()) as i32), 2i32)) as i16));
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((summaryBarSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(0i16);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((summaryBarSpriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_StatusSummaryBar_Exit));
            SetSubspriteTables(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((summaryBarSpriteId) as i32) as isize * 68),
                ((&raw const sStatusSummaryBar_SubspriteTable_Exit)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HidePartyStatusSummary_BattleStart_1));
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HidePartyStatusSummary_DuringBattle));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HidePartyStatusSummary_BattleStart_1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if crate::c::rem_i32(
            (({
                let __p1 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11);
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_add(1));
                __t2
            }) as i32),
            2i32,
        ) == 0i32
        {
            if (({
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15);
                let __t4 = ((__p3).read()).wrapping_sub(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                < 0i32
            {
                return;
            }
            SetGpuReg(
                82u8,
                ((((16i32).wrapping_sub(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .read()) as i32),
                ) << 8)
                    | ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .read()) as i32)) as u16),
            );
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read()) as i32)
            == 0i32
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HidePartyStatusSummary_BattleStart_2));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HidePartyStatusSummary_BattleStart_2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut ballIconSpriteIds = crate::ffi::Align4([0u8; 6]);
        let mut i: i32 = 0i32;
        let mut battler: u8 = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u8);
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == (-1i32)
        {
            let mut summaryBarSpriteId: u8 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as u8);
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((&raw mut ballIconSpriteIds).cast::<u8>()).wrapping_offset((i) as isize))
                            .write(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(((3i32).wrapping_add(i)) as isize))
                                .read()) as u8),
                            );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            crate::c::bf_write(
                (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(9),
                2,
                3,
                ((crate::c::bf_read(
                    (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(9),
                    2,
                    3,
                    false,
                ) as u8)
                    .wrapping_sub(1)) as i32,
            );
            if ((crate::c::bf_read(
                (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(8)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(9),
                2,
                3,
                false,
            ) as u8) as i32)
                == 0i32
            {
                DestroySpriteAndFreeResources(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((summaryBarSpriteId) as i32) as isize * 68),
                );
                DestroySpriteAndFreeResources(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut ballIconSpriteIds).cast::<u8>()).read()) as i32) as isize * 68,
                ));
            } else {
                FreeSpriteOamMatrix(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((summaryBarSpriteId) as i32) as isize * 68),
                );
                DestroySprite(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((summaryBarSpriteId) as i32) as isize * 68),
                );
                FreeSpriteOamMatrix(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut ballIconSpriteIds).cast::<u8>()).read()) as i32) as isize * 68,
                ));
                DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut ballIconSpriteIds).cast::<u8>()).read()) as i32) as isize * 68,
                ));
            }
            {
                i = 1i32;
                'l3: loop {
                    if !(i < 6i32) {
                        break 'l3;
                    }
                    'l4: {
                        DestroySprite(
                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut ballIconSpriteIds).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as i32)
                == (-3i32)
            {
                crate::c::bf_write(
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 12))
                    .wrapping_add(0),
                    0,
                    1,
                    (0u8) as i32,
                );
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                DestroyTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HidePartyStatusSummary_DuringBattle(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut ballIconSpriteIds = crate::ffi::Align4([0u8; 6]);
        let mut i: i32 = 0i32;
        let mut battler: u8 = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u8);
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 0i32
        {
            SetGpuReg(
                82u8,
                ((((16i32).wrapping_sub(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .read()) as i32),
                ) << 8)
                    | ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .read()) as i32)) as u16),
            );
        } else {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as i32)
                == (-1i32)
            {
                let mut summaryBarSpriteId: u8 = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u8);
                {
                    i = 0i32;
                    'l1: loop {
                        if !(i < 6i32) {
                            break 'l1;
                        }
                        'l2: {
                            (((&raw mut ballIconSpriteIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .write(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(((3i32).wrapping_add(i)) as isize))
                                .read()) as u8),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                DestroySpriteAndFreeResources(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((summaryBarSpriteId) as i32) as isize * 68),
                );
                DestroySpriteAndFreeResources(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((&raw mut ballIconSpriteIds).cast::<u8>()).read()) as i32) as isize * 68,
                ));
                {
                    i = 1i32;
                    'l3: loop {
                        if !(i < 6i32) {
                            break 'l3;
                        }
                        'l4: {
                            DestroySprite(
                                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut ballIconSpriteIds).cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            } else {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32)
                    == (-3i32)
                {
                    crate::c::bf_write(
                        ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((battler) as i32) as isize * 12))
                        .wrapping_add(0),
                        0,
                        1,
                        (0u8) as i32,
                    );
                    SetGpuReg(80u8, 0u16);
                    SetGpuReg(82u8, 0u16);
                    DestroyTask(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_StatusSummaryBar_Enter(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) != 0i32 {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_StatusSummaryBar_Exit(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(32i32)) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 0i32 {
            let __p2 = (sprite).wrapping_add(36).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        >> 4),
                )) as i16),
            );
        } else {
            let __p3 = (sprite).wrapping_add(36).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_sub(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        >> 4),
                )) as i16),
            );
        }
        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p4).write((((((__p4).read()) as i32) & 15i32) as i16));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_StatusSummaryBalls_Enter(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut var1: u8 = 0u8;
        let mut var2: u16 = 0u16;
        let mut pan: i8 = 0i8;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32) > 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            return;
        }
        var1 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8);
        var2 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u16);
        var2 = ((((var2) as i32).wrapping_add(56i32)) as u16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((((var2) as i32) & 65520i32) as i16));
        if ((var1) as i32) != 0i32 {
            let __p2 = (sprite).wrapping_add(36).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add((((var2) as i32) >> 4))) as i16));
            if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) > 0i32 {
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            }
        } else {
            let __p3 = (sprite).wrapping_add(36).cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_sub((((var2) as i32) >> 4))) as i16));
            if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) < 0i32 {
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            }
        }
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) == 0i32 {
            pan = 63i8;
            if ((var1) as i32) != 0i32 {
                pan = (-64i8);
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                != 0i32
            {
                PlaySE2WithPanning(116u16, pan);
            } else {
                PlaySE1WithPanning(115u16, pan);
            }
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_StatusSummaryBalls_Exit(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut var1: u8 = 0u8;
        let mut var2: u16 = 0u16;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32) > 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            return;
        }
        var1 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8);
        var2 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u16);
        var2 = ((((var2) as i32).wrapping_add(56i32)) as u16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((((var2) as i32) & 65520i32) as i16));
        if ((var1) as i32) != 0i32 {
            let __p2 = (sprite).wrapping_add(36).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add((((var2) as i32) >> 4))) as i16));
        } else {
            let __p3 = (sprite).wrapping_add(36).cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_sub((((var2) as i32) >> 4))) as i16));
        }
        if (((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32))
            > 248i32)
            || (((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32))
                < (-8i32))
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_StatusSummaryBalls_OnSwitchout(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut barSpriteId: u8 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8);
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((barSpriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .read(),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((barSpriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateNickInHealthbox(healthboxSpriteId: u8, mon: *mut u8) {
    unsafe {
        let mut healthboxSpriteId = healthboxSpriteId;
        let mut mon = mon;
        let mut nickname = crate::ffi::Align4([0u8; 11]);
        let mut ptr: *mut u8 = core::ptr::null_mut();
        let mut windowId: u32 = 0u32;
        let mut spriteTileNum: u32 = 0u32;
        let mut windowTileData: *mut u8 = core::ptr::null_mut();
        let mut species: u16 = 0u16;
        let mut gender: u8 = 0u8;
        StringCopy(
            (&raw mut gDisplayedStringBattle).cast::<u8>(),
            (&raw mut gText_HealthboxNickname).cast::<u8>(),
        );
        GetMonData3(mon, 2i32, (&raw mut nickname).cast::<u8>());
        StringGet_Nickname((&raw mut nickname).cast::<u8>());
        ptr = StringAppend(
            (&raw mut gDisplayedStringBattle).cast::<u8>(),
            (&raw mut nickname).cast::<u8>(),
        );
        gender = GetMonGender(mon);
        species = ((GetMonData2(mon, 11i32)) as u16);
        if ((((species) as i32) == 29i32) || (((species) as i32) == 32i32))
            && (StringCompare(
                (&raw mut nickname).cast::<u8>(),
                (((&raw mut gSpeciesNames).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 11))
                .cast::<u8>(),
            ) == 0i32)
        {
            gender = 100u8;
        }
        'l1: {
            let __sw1 = ((gender) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 254i32;
            if !__matched {
                StringCopy(ptr, (&raw mut gText_HealthboxGender_None).cast::<u8>());
                windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
                    (&raw mut gDisplayedStringBattle).cast::<u8>(),
                    0u32,
                    3u32,
                    2u32,
                    &raw mut windowId,
                );
                break 'l1;
            }
            if __sw1 == 0i32 {
                StringCopy(ptr, (&raw mut gText_HealthboxGender_Male).cast::<u8>());
                windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
                    (&raw mut gDisplayedStringBattle).cast::<u8>(),
                    0u32,
                    3u32,
                    2u32,
                    &raw mut windowId,
                );
                break 'l1;
            }
            if __sw1 == 254i32 {
                StringCopy(ptr, (&raw mut gText_HealthboxGender_Female).cast::<u8>());
                windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
                    (&raw mut gDisplayedStringBattle).cast::<u8>(),
                    0u32,
                    3u32,
                    2u32,
                    &raw mut windowId,
                );
                break 'l1;
            }
        }
        spriteTileNum = ((((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(4),
            0,
            10,
            false,
        ) as u16) as i32)
            .wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u32);
        if ((GetBattlerSide(
            ((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as u8),
        )) as i32)
            == 0i32
        {
            TextIntoHealthboxObject(
                (((100728896u32).wrapping_add(spriteTileNum)) as usize as *mut u8),
                windowTileData,
                6i32,
            );
            ptr = ((100728832i32) as usize as *mut u8);
            if !((IsDoubleBattle()) != 0) {
                ptr = (ptr)
                    .wrapping_offset((((spriteTileNum).wrapping_add(2048u32)) as i32) as isize * 1);
            } else {
                ptr = (ptr)
                    .wrapping_offset((((spriteTileNum).wrapping_add(1024u32)) as i32) as isize * 1);
            }
            TextIntoHealthboxObject(ptr, (windowTileData).wrapping_offset(192), 1i32);
        } else {
            TextIntoHealthboxObject(
                (((100728864u32).wrapping_add(spriteTileNum)) as usize as *mut u8),
                windowTileData,
                7i32,
            );
        }
        RemoveWindowOnHealthbox(windowId);
    }
}
pub(crate) unsafe extern "C" fn TryAddPokeballIconToHealthbox(healthboxSpriteId: u8, noStatus: u8) {
    unsafe {
        let mut healthboxSpriteId = healthboxSpriteId;
        let mut noStatus = noStatus;
        let mut battler: u8 = 0u8;
        let mut healthBarSpriteId: u8 = 0u8;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 512u32) != 0 {
            return;
        }
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0 {
            return;
        }
        battler = ((((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .read()) as u8);
        if ((GetBattlerSide(battler)) as i32) == 0i32 {
            return;
        }
        if !((GetSetPokedexFlag(
            SpeciesToNationalPokedexNum(
                ((GetMonData2(
                    ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    11i32,
                )) as u16),
            ),
            1u8,
        )) != 0)
        {
            return;
        }
        healthBarSpriteId = ((((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .read()) as u8);
        if (noStatus) != 0 {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                GetHealthboxElementGfxPtr(70u8),
                                (((100728832i32).wrapping_add(
                                    (((crate::c::bf_read(
                                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            ((healthBarSpriteId) as i32) as isize * 68,
                                        ))
                                        .wrapping_add(4),
                                        0,
                                        10,
                                        false,
                                    ) as u16) as i32)
                                        .wrapping_add(8i32))
                                    .wrapping_mul(crate::c::div_i32(256i32, 8i32)),
                                )) as usize as *mut u8),
                                ((67108864i32
                                    | (crate::c::div_i32(32i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        } else {
            'l5: loop {
                'l6: {
                    {
                        let mut tmp: u32 = 0u32;
                        (&raw mut tmp).write_volatile(0u32);
                        'l7: loop {
                            'l8: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    (((100728832i32).wrapping_add(
                                        (((crate::c::bf_read(
                                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                ((healthBarSpriteId) as i32) as isize * 68,
                                            ))
                                            .wrapping_add(4),
                                            0,
                                            10,
                                            false,
                                        ) as u16)
                                            as i32)
                                            .wrapping_add(8i32))
                                        .wrapping_mul(crate::c::div_i32(256i32, 8i32)),
                                    )) as usize as *mut u8),
                                    ((83886080i32
                                        | (crate::c::div_i32(
                                            32i32,
                                            crate::c::div_i32(32i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l7;
                            }
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l5;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateStatusIconInHealthbox(healthboxSpriteId: u8) {
    unsafe {
        let mut healthboxSpriteId = healthboxSpriteId;
        let mut i: i32 = 0i32;
        let mut battler: u8 = 0u8;
        let mut healthBarSpriteId: u8 = 0u8;
        let mut status: u32 = 0u32;
        let mut pltAdder: u32 = 0u32;
        let mut statusGfxPtr: *mut u8 = core::ptr::null_mut();
        let mut tileNumAdder: i16 = 0i16;
        let mut statusPalId: u8 = 0u8;
        battler = ((((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .read()) as u8);
        healthBarSpriteId = ((((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .read()) as u8);
        if ((GetBattlerSide(battler)) as i32) == 0i32 {
            status = GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 100,
                ),
                55i32,
            );
            if !((IsDoubleBattle()) != 0) {
                tileNumAdder = 26i16;
            } else {
                tileNumAdder = 18i16;
            }
        } else {
            status = GetMonData2(
                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 100,
                ),
                55i32,
            );
            tileNumAdder = 17i16;
        }
        if (status & 7u32) != 0 {
            statusGfxPtr = GetHealthboxElementGfxPtr(GetStatusIconForBattlerId(27u8, battler));
            statusPalId = 2u8;
        } else {
            if (status & 136u32) != 0 {
                statusGfxPtr = GetHealthboxElementGfxPtr(GetStatusIconForBattlerId(21u8, battler));
                statusPalId = 0u8;
            } else {
                if (status & 16u32) != 0 {
                    statusGfxPtr =
                        GetHealthboxElementGfxPtr(GetStatusIconForBattlerId(33u8, battler));
                    statusPalId = 4u8;
                } else {
                    if (status & 32u32) != 0 {
                        statusGfxPtr =
                            GetHealthboxElementGfxPtr(GetStatusIconForBattlerId(30u8, battler));
                        statusPalId = 3u8;
                    } else {
                        if (status & 64u32) != 0 {
                            statusGfxPtr =
                                GetHealthboxElementGfxPtr(GetStatusIconForBattlerId(24u8, battler));
                            statusPalId = 1u8;
                        } else {
                            statusGfxPtr = GetHealthboxElementGfxPtr(39u8);
                            {
                                i = 0i32;
                                'l1: loop {
                                    if !(i < 3i32) {
                                        break 'l1;
                                    }
                                    'l2: {
                                        'l3: loop {
                                            'l4: {
                                                'l5: loop {
                                                    'l6: {
                                                        CpuSet(
                                                            statusGfxPtr,
                                                            (((100728832i32).wrapping_add(
                                                                ((((crate::c::bf_read(
                                                                    (((&raw mut gSprites)
                                                                        .cast::<u8>())
                                                                    .wrapping_offset(
                                                                        ((healthboxSpriteId) as i32)
                                                                            as isize
                                                                            * 68,
                                                                    ))
                                                                    .wrapping_add(4),
                                                                    0,
                                                                    10,
                                                                    false,
                                                                )
                                                                    as u16)
                                                                    as i32)
                                                                    .wrapping_add(
                                                                        ((tileNumAdder) as i32),
                                                                    ))
                                                                .wrapping_add(i))
                                                                .wrapping_mul(crate::c::div_i32(
                                                                    256i32, 8i32,
                                                                )),
                                                            ))
                                                                as usize
                                                                as *mut u8),
                                                            ((67108864i32
                                                                | (crate::c::div_i32(
                                                                    32i32,
                                                                    crate::c::div_i32(32i32, 8i32),
                                                                ) & 2097151i32))
                                                                as u32),
                                                        );
                                                    }
                                                    if !((0i32) != 0) {
                                                        break 'l5;
                                                    }
                                                }
                                            }
                                            if !((0i32) != 0) {
                                                break 'l3;
                                            }
                                        }
                                    }
                                    i = (i).wrapping_add(1);
                                }
                            }
                            if !((crate::c::bf_read(
                                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((battler) as i32) as isize * 4))
                                .wrapping_add(0),
                                4,
                                1,
                                false,
                            ) as u16)
                                != 0)
                            {
                                'l7: loop {
                                    'l8: {
                                        'l9: loop {
                                            'l10: {
                                                CpuSet(
                                                    GetHealthboxElementGfxPtr(1u8),
                                                    (((100728832i32).wrapping_add(
                                                        ((crate::c::bf_read(
                                                            (((&raw mut gSprites).cast::<u8>())
                                                                .wrapping_offset(
                                                                    ((healthBarSpriteId) as i32)
                                                                        as isize
                                                                        * 68,
                                                                ))
                                                            .wrapping_add(4),
                                                            0,
                                                            10,
                                                            false,
                                                        )
                                                            as u16)
                                                            as i32)
                                                            .wrapping_mul(crate::c::div_i32(
                                                                256i32, 8i32,
                                                            )),
                                                    ))
                                                        as usize
                                                        as *mut u8),
                                                    ((67108864i32
                                                        | (crate::c::div_i32(
                                                            64i32,
                                                            crate::c::div_i32(32i32, 8i32),
                                                        ) & 2097151i32))
                                                        as u32),
                                                );
                                            }
                                            if !((0i32) != 0) {
                                                break 'l9;
                                            }
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l7;
                                    }
                                }
                            }
                            TryAddPokeballIconToHealthbox(healthboxSpriteId, 1u8);
                            return;
                        }
                    }
                }
            }
        }
        pltAdder = ((((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(5),
            4,
            4,
            false,
        ) as u16) as i32)
            .wrapping_mul(16i32)) as u32);
        pltAdder = (pltAdder).wrapping_add(((((battler) as i32).wrapping_add(12i32)) as u32));
        FillPalette(
            ((((&raw const sStatusIconColors)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((statusPalId) as i32) as isize))
            .read(),
            (((256u32).wrapping_add(pltAdder)) as u16),
            2u16,
        );
        'l11: loop {
            'l12: {
                'l13: loop {
                    'l14: {
                        CpuSet(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    (((256u32).wrapping_add(pltAdder)) as i32) as isize,
                                ))
                            .cast::<u8>(),
                            (((83886592i32) as usize as *mut u16)
                                .wrapping_offset(((pltAdder) as i32) as isize))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    2u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l13;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l11;
            }
        }
        'l15: loop {
            'l16: {
                'l17: loop {
                    'l18: {
                        CpuSet(
                            statusGfxPtr,
                            (((100728832i32).wrapping_add(
                                (((crate::c::bf_read(
                                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((healthboxSpriteId) as i32) as isize * 68,
                                    ))
                                    .wrapping_add(4),
                                    0,
                                    10,
                                    false,
                                ) as u16) as i32)
                                    .wrapping_add(((tileNumAdder) as i32)))
                                .wrapping_mul(crate::c::div_i32(256i32, 8i32)),
                            )) as usize as *mut u8),
                            ((67108864i32
                                | (crate::c::div_i32(96i32, crate::c::div_i32(32i32, 8i32))
                                    & 2097151i32)) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l17;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l15;
            }
        }
        if (((IsDoubleBattle()) as i32) == 1i32) || (((GetBattlerSide(battler)) as i32) == 1i32) {
            if !((crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 4))
                .wrapping_add(0),
                4,
                1,
                false,
            ) as u16)
                != 0)
            {
                'l19: loop {
                    'l20: {
                        'l21: loop {
                            'l22: {
                                CpuSet(
                                    GetHealthboxElementGfxPtr(0u8),
                                    (((100728832i32).wrapping_add(
                                        ((crate::c::bf_read(
                                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                ((healthBarSpriteId) as i32) as isize * 68,
                                            ))
                                            .wrapping_add(4),
                                            0,
                                            10,
                                            false,
                                        ) as u16) as i32)
                                            .wrapping_mul(crate::c::div_i32(256i32, 8i32)),
                                    )) as usize as *mut u8),
                                    ((67108864i32
                                        | (crate::c::div_i32(
                                            32i32,
                                            crate::c::div_i32(32i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l21;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l19;
                    }
                }
                'l23: loop {
                    'l24: {
                        'l25: loop {
                            'l26: {
                                CpuSet(
                                    GetHealthboxElementGfxPtr(65u8),
                                    (((100728832i32).wrapping_add(
                                        (((crate::c::bf_read(
                                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                ((healthBarSpriteId) as i32) as isize * 68,
                                            ))
                                            .wrapping_add(4),
                                            0,
                                            10,
                                            false,
                                        ) as u16)
                                            as i32)
                                            .wrapping_add(1i32))
                                        .wrapping_mul(crate::c::div_i32(256i32, 8i32)),
                                    )) as usize as *mut u8),
                                    ((67108864i32
                                        | (crate::c::div_i32(
                                            32i32,
                                            crate::c::div_i32(32i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l25;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l23;
                    }
                }
            }
        }
        TryAddPokeballIconToHealthbox(healthboxSpriteId, 0u8);
    }
}
pub(crate) unsafe extern "C" fn GetStatusIconForBattlerId(statusElementId: u8, battler: u8) -> u8 {
    unsafe {
        let mut statusElementId = statusElementId;
        let mut battler = battler;
        let mut ret: u8 = statusElementId;
        'l1: {
            let __sw1 = ((statusElementId) as i32);
            if __sw1 == 21i32 {
                if ((battler) as i32) == 0i32 {
                    ret = 21u8;
                } else {
                    if ((battler) as i32) == 1i32 {
                        ret = 71u8;
                    } else {
                        if ((battler) as i32) == 2i32 {
                            ret = 86u8;
                        } else {
                            ret = 101u8;
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 24i32 {
                if ((battler) as i32) == 0i32 {
                    ret = 24u8;
                } else {
                    if ((battler) as i32) == 1i32 {
                        ret = 74u8;
                    } else {
                        if ((battler) as i32) == 2i32 {
                            ret = 89u8;
                        } else {
                            ret = 104u8;
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 27i32 {
                if ((battler) as i32) == 0i32 {
                    ret = 27u8;
                } else {
                    if ((battler) as i32) == 1i32 {
                        ret = 77u8;
                    } else {
                        if ((battler) as i32) == 2i32 {
                            ret = 92u8;
                        } else {
                            ret = 107u8;
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 30i32 {
                if ((battler) as i32) == 0i32 {
                    ret = 30u8;
                } else {
                    if ((battler) as i32) == 1i32 {
                        ret = 80u8;
                    } else {
                        if ((battler) as i32) == 2i32 {
                            ret = 95u8;
                        } else {
                            ret = 110u8;
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 33i32 {
                if ((battler) as i32) == 0i32 {
                    ret = 33u8;
                } else {
                    if ((battler) as i32) == 1i32 {
                        ret = 83u8;
                    } else {
                        if ((battler) as i32) == 2i32 {
                            ret = 98u8;
                        } else {
                            ret = 113u8;
                        }
                    }
                }
                break 'l1;
            }
        }
        return ret;
    }
}
pub(crate) unsafe extern "C" fn UpdateSafariBallsTextOnHealthbox(healthboxSpriteId: u8) {
    unsafe {
        let mut healthboxSpriteId = healthboxSpriteId;
        let mut windowId: u32 = 0u32;
        let mut spriteTileNum: u32 = 0u32;
        let mut windowTileData: *mut u8 = core::ptr::null_mut();
        windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
            (&raw mut gText_SafariBalls).cast::<u8>(),
            0u32,
            3u32,
            2u32,
            &raw mut windowId,
        );
        spriteTileNum = ((((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(4),
            0,
            10,
            false,
        ) as u16) as i32)
            .wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u32);
        TextIntoHealthboxObject(
            ((100728896i32) as usize as *mut u8)
                .wrapping_offset(((spriteTileNum) as i32) as isize * 1),
            windowTileData,
            6i32,
        );
        TextIntoHealthboxObject(
            ((100730880i32) as usize as *mut u8)
                .wrapping_offset(((spriteTileNum) as i32) as isize * 1),
            (windowTileData).wrapping_offset(192),
            2i32,
        );
        RemoveWindowOnHealthbox(windowId);
    }
}
pub(crate) unsafe extern "C" fn UpdateLeftNoOfBallsTextOnHealthbox(healthboxSpriteId: u8) {
    unsafe {
        let mut healthboxSpriteId = healthboxSpriteId;
        let mut text = crate::ffi::Align4([0u8; 16]);
        let mut txtPtr: *mut u8 = core::ptr::null_mut();
        let mut windowId: u32 = 0u32;
        let mut spriteTileNum: u32 = 0u32;
        let mut windowTileData: *mut u8 = core::ptr::null_mut();
        txtPtr = StringCopy(
            (&raw mut text).cast::<u8>(),
            (&raw mut gText_SafariBallLeft).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            txtPtr,
            ((((&raw mut gNumSafariBalls).cast::<u8>()).read()) as i32),
            0i32,
            2u8,
        );
        windowTileData = AddTextPrinterAndCreateWindowOnHealthbox(
            (&raw mut text).cast::<u8>(),
            ((GetStringRightAlignXOffset(0i32, (&raw mut text).cast::<u8>(), 47i32)) as u32),
            3u32,
            2u32,
            &raw mut windowId,
        );
        spriteTileNum = ((((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(4),
            0,
            10,
            false,
        ) as u16) as i32)
            .wrapping_mul(crate::c::div_i32(256i32, 8i32))) as u32);
        SafariTextIntoHealthboxObject(
            ((100729536i32) as usize as *mut u8)
                .wrapping_offset(((spriteTileNum) as i32) as isize * 1),
            windowTileData,
            2u32,
        );
        SafariTextIntoHealthboxObject(
            ((100731392i32) as usize as *mut u8)
                .wrapping_offset(((spriteTileNum) as i32) as isize * 1),
            (windowTileData).wrapping_offset(64),
            4u32,
        );
        RemoveWindowOnHealthbox(windowId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateHealthboxAttribute(
    healthboxSpriteId: u8,
    mon: *mut u8,
    elementId: u8,
) {
    unsafe {
        let mut healthboxSpriteId = healthboxSpriteId;
        let mut mon = mon;
        let mut elementId = elementId;
        let mut maxHp: i32 = 0i32;
        let mut currHp: i32 = 0i32;
        let mut battler: u8 = ((((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .read()) as u8);
        if (((elementId) as i32) == 0i32) && (!((IsDoubleBattle()) != 0)) {
            GetBattlerSide(battler);
        }
        if ((GetBattlerSide(
            ((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((healthboxSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as u8),
        )) as i32)
            == 0i32
        {
            let mut isDoubles: u8 = 0u8;
            if (((elementId) as i32) == 3i32) || (((elementId) as i32) == 0i32) {
                UpdateLvlInHealthbox(healthboxSpriteId, ((GetMonData2(mon, 56i32)) as u8));
            }
            if (((elementId) as i32) == 1i32) || (((elementId) as i32) == 0i32) {
                UpdateHpTextInHealthbox(healthboxSpriteId, ((GetMonData2(mon, 57i32)) as i16), 0u8);
            }
            if (((elementId) as i32) == 2i32) || (((elementId) as i32) == 0i32) {
                UpdateHpTextInHealthbox(healthboxSpriteId, ((GetMonData2(mon, 58i32)) as i16), 1u8);
            }
            if (((elementId) as i32) == 5i32) || (((elementId) as i32) == 0i32) {
                LoadBattleBarGfx(0u8);
                maxHp = ((GetMonData2(mon, 58i32)) as i32);
                currHp = ((GetMonData2(mon, 57i32)) as i32);
                SetBattleBarStruct(battler, healthboxSpriteId, maxHp, currHp, 0i32);
                MoveBattleBar(battler, healthboxSpriteId, 0u8, 0u8);
            }
            isDoubles = IsDoubleBattle();
            if (!((isDoubles) != 0))
                && ((((elementId) as i32) == 6i32) || (((elementId) as i32) == 0i32))
            {
                let mut species: u16 = 0u16;
                let mut exp: u32 = 0u32;
                let mut currLevelExp: u32 = 0u32;
                let mut currExpBarValue: i32 = 0i32;
                let mut maxExpBarValue: i32 = 0i32;
                let mut level: u8 = 0u8;
                LoadBattleBarGfx(3u8);
                species = ((GetMonData2(mon, 11i32)) as u16);
                level = ((GetMonData2(mon, 56i32)) as u8);
                exp = GetMonData2(mon, 25i32);
                currLevelExp = (((((&raw mut gExperienceTables).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gSpeciesInfo).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 28))
                    .wrapping_add(19))
                    .read()) as i32) as isize
                        * 404,
                ))
                .cast::<u32>())
                .wrapping_offset(((level) as i32) as isize))
                .read();
                currExpBarValue = (((exp).wrapping_sub(currLevelExp)) as i32);
                maxExpBarValue = ((((((((&raw mut gExperienceTables).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut gSpeciesInfo).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 28))
                        .wrapping_add(19))
                        .read()) as i32) as isize
                            * 404,
                    ))
                .cast::<u32>())
                .wrapping_offset((((level) as i32).wrapping_add(1i32)) as isize))
                .read())
                .wrapping_sub(currLevelExp)) as i32);
                SetBattleBarStruct(
                    battler,
                    healthboxSpriteId,
                    maxExpBarValue,
                    currExpBarValue,
                    ((isDoubles) as i32),
                );
                MoveBattleBar(battler, healthboxSpriteId, 1u8, 0u8);
            }
            if (((elementId) as i32) == 4i32) || (((elementId) as i32) == 0i32) {
                UpdateNickInHealthbox(healthboxSpriteId, mon);
            }
            if (((elementId) as i32) == 9i32) || (((elementId) as i32) == 0i32) {
                UpdateStatusIconInHealthbox(healthboxSpriteId);
            }
            if ((elementId) as i32) == 10i32 {
                UpdateSafariBallsTextOnHealthbox(healthboxSpriteId);
            }
            if (((elementId) as i32) == 10i32) || (((elementId) as i32) == 11i32) {
                UpdateLeftNoOfBallsTextOnHealthbox(healthboxSpriteId);
            }
        } else {
            if (((elementId) as i32) == 3i32) || (((elementId) as i32) == 0i32) {
                UpdateLvlInHealthbox(healthboxSpriteId, ((GetMonData2(mon, 56i32)) as u8));
            }
            if (((elementId) as i32) == 5i32) || (((elementId) as i32) == 0i32) {
                LoadBattleBarGfx(0u8);
                maxHp = ((GetMonData2(mon, 58i32)) as i32);
                currHp = ((GetMonData2(mon, 57i32)) as i32);
                SetBattleBarStruct(battler, healthboxSpriteId, maxHp, currHp, 0i32);
                MoveBattleBar(battler, healthboxSpriteId, 0u8, 0u8);
            }
            if (((elementId) as i32) == 4i32) || (((elementId) as i32) == 0i32) {
                UpdateNickInHealthbox(healthboxSpriteId, mon);
            }
            if (((elementId) as i32) == 9i32) || (((elementId) as i32) == 0i32) {
                UpdateStatusIconInHealthbox(healthboxSpriteId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveBattleBar(
    battler: u8,
    healthboxSpriteId: u8,
    whichBar: u8,
    unused: u8,
) -> i32 {
    unsafe {
        let mut battler = battler;
        let mut healthboxSpriteId = healthboxSpriteId;
        let mut whichBar = whichBar;
        let mut unused = unused;
        let mut currentBarValue: i32 = 0i32;
        if ((whichBar) as i32) == 0i32 {
            currentBarValue = CalcNewBarValue(
                (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 20))
                .wrapping_add(4)
                .cast::<i32>())
                .read(),
                (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 20))
                .wrapping_add(8)
                .cast::<i32>())
                .read(),
                (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 20))
                .wrapping_add(12)
                .cast::<i32>())
                .read(),
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 20))
                .wrapping_add(16)
                .cast::<i32>(),
                ((crate::c::div_i32(48i32, 8i32)) as u8),
                1u16,
            );
        } else {
            let mut expFraction: u16 = ((GetScaledExpFraction(
                (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 20))
                .wrapping_add(8)
                .cast::<i32>())
                .read(),
                (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 20))
                .wrapping_add(12)
                .cast::<i32>())
                .read(),
                (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 20))
                .wrapping_add(4)
                .cast::<i32>())
                .read(),
                8u8,
            )) as u16);
            if ((expFraction) as i32) == 0i32 {
                expFraction = 1u16;
            }
            expFraction = ((if crate::c::div_i32(
                (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 20))
                .wrapping_add(12)
                .cast::<i32>())
                .read(),
                ((expFraction) as i32),
            ) < 0i32
            {
                (crate::c::div_i32(
                    (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 20))
                    .wrapping_add(12)
                    .cast::<i32>())
                    .read(),
                    ((expFraction) as i32),
                ))
                .wrapping_neg()
            } else {
                crate::c::div_i32(
                    (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 20))
                    .wrapping_add(12)
                    .cast::<i32>())
                    .read(),
                    ((expFraction) as i32),
                )
            }) as u16);
            currentBarValue = CalcNewBarValue(
                (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 20))
                .wrapping_add(4)
                .cast::<i32>())
                .read(),
                (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 20))
                .wrapping_add(8)
                .cast::<i32>())
                .read(),
                (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 20))
                .wrapping_add(12)
                .cast::<i32>())
                .read(),
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 20))
                .wrapping_add(16)
                .cast::<i32>(),
                ((crate::c::div_i32(64i32, 8i32)) as u8),
                expFraction,
            );
        }
        if (((whichBar) as i32) == 1i32)
            || ((((whichBar) as i32) == 0i32)
                && (!((crate::c::bf_read(
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 4))
                    .wrapping_add(0),
                    4,
                    1,
                    false,
                ) as u16)
                    != 0)))
        {
            MoveBattleBarGraphically(battler, whichBar);
        }
        if currentBarValue == (-1i32) {
            (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((battler) as i32) as isize * 20))
            .wrapping_add(16)
            .cast::<i32>())
            .write(0i32);
        }
        return currentBarValue;
    }
}
pub(crate) unsafe extern "C" fn MoveBattleBarGraphically(battler: u8, whichBar: u8) {
    unsafe {
        let mut battler = battler;
        let mut whichBar = whichBar;
        let mut array = crate::ffi::Align4([0u8; 8]);
        let mut filledPixelsCount: u8 = 0u8;
        let mut level: u8 = 0u8;
        let mut barElementId: u8 = 0u8;
        let mut i: u8 = 0u8;
        'l1: {
            let __sw1 = ((whichBar) as i32);
            if __sw1 == 0i32 {
                filledPixelsCount = CalcBarFilledPixels(
                    (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 20))
                    .wrapping_add(4)
                    .cast::<i32>())
                    .read(),
                    (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 20))
                    .wrapping_add(8)
                    .cast::<i32>())
                    .read(),
                    (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 20))
                    .wrapping_add(12)
                    .cast::<i32>())
                    .read(),
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 20))
                    .wrapping_add(16)
                    .cast::<i32>(),
                    (&raw mut array).cast::<u8>(),
                    ((crate::c::div_i32(48i32, 8i32)) as u8),
                );
                if ((filledPixelsCount) as i32) > crate::c::div_i32(2400i32, 100i32) {
                    barElementId = 3u8;
                } else {
                    if ((filledPixelsCount) as i32) > crate::c::div_i32(960i32, 100i32) {
                        barElementId = 47u8;
                    } else {
                        barElementId = 56u8;
                    }
                }
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < 6i32) {
                            break 'l2;
                        }
                        'l3: {
                            let mut healthbarSpriteId: u8 =
                                ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(12)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((battler) as i32) as isize * 20))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(5))
                                .read()) as u8);
                            if ((i) as i32) < 2i32 {
                                'l4: loop {
                                    'l5: {
                                        'l6: loop {
                                            'l7: {
                                                CpuSet(
                                                    (GetHealthboxElementGfxPtr(barElementId))
                                                        .wrapping_offset(
                                                            ((((((&raw mut array).cast::<u8>())
                                                                .wrapping_offset(
                                                                    ((i) as i32) as isize,
                                                                ))
                                                            .read())
                                                                as i32)
                                                                .wrapping_mul(32i32))
                                                                as isize,
                                                        ),
                                                    (((100728832i32).wrapping_add(
                                                        ((((crate::c::bf_read(
                                                            (((&raw mut gSprites).cast::<u8>())
                                                                .wrapping_offset(
                                                                    ((healthbarSpriteId) as i32)
                                                                        as isize
                                                                        * 68,
                                                                ))
                                                            .wrapping_add(4),
                                                            0,
                                                            10,
                                                            false,
                                                        )
                                                            as u16)
                                                            as i32)
                                                            .wrapping_add(2i32))
                                                        .wrapping_add(((i) as i32)))
                                                        .wrapping_mul(crate::c::div_i32(
                                                            256i32, 8i32,
                                                        )),
                                                    ))
                                                        as usize
                                                        as *mut u8),
                                                    ((67108864i32
                                                        | (crate::c::div_i32(
                                                            32i32,
                                                            crate::c::div_i32(32i32, 8i32),
                                                        ) & 2097151i32))
                                                        as u32),
                                                );
                                            }
                                            if !((0i32) != 0) {
                                                break 'l6;
                                            }
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l4;
                                    }
                                }
                            } else {
                                'l8: loop {
                                    'l9: {
                                        'l10: loop {
                                            'l11: {
                                                CpuSet(
                                                    (GetHealthboxElementGfxPtr(barElementId))
                                                        .wrapping_offset(
                                                            ((((((&raw mut array).cast::<u8>())
                                                                .wrapping_offset(
                                                                    ((i) as i32) as isize,
                                                                ))
                                                            .read())
                                                                as i32)
                                                                .wrapping_mul(32i32))
                                                                as isize,
                                                        ),
                                                    (((100728896i32).wrapping_add(
                                                        (((i) as i32).wrapping_add(
                                                            ((crate::c::bf_read(
                                                                (((&raw mut gSprites)
                                                                    .cast::<u8>())
                                                                .wrapping_offset(
                                                                    ((healthbarSpriteId) as i32)
                                                                        as isize
                                                                        * 68,
                                                                ))
                                                                .wrapping_add(4),
                                                                0,
                                                                10,
                                                                false,
                                                            )
                                                                as u16)
                                                                as i32),
                                                        ))
                                                        .wrapping_mul(crate::c::div_i32(
                                                            256i32, 8i32,
                                                        )),
                                                    ))
                                                        as usize
                                                        as *mut u8),
                                                    ((67108864i32
                                                        | (crate::c::div_i32(
                                                            32i32,
                                                            crate::c::div_i32(32i32, 8i32),
                                                        ) & 2097151i32))
                                                        as u32),
                                                );
                                            }
                                            if !((0i32) != 0) {
                                                break 'l10;
                                            }
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l8;
                                    }
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                CalcBarFilledPixels(
                    (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 20))
                    .wrapping_add(4)
                    .cast::<i32>())
                    .read(),
                    (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 20))
                    .wrapping_add(8)
                    .cast::<i32>())
                    .read(),
                    (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 20))
                    .wrapping_add(12)
                    .cast::<i32>())
                    .read(),
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 20))
                    .wrapping_add(16)
                    .cast::<i32>(),
                    (&raw mut array).cast::<u8>(),
                    ((crate::c::div_i32(64i32, 8i32)) as u8),
                );
                level = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    56i32,
                )) as u8);
                if ((level) as i32) == 100i32 {
                    {
                        i = 0u8;
                        'l12: loop {
                            if !(((i) as i32) < 8i32) {
                                break 'l12;
                            }
                            'l13: {
                                (((&raw mut array).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .write(0u8);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                {
                    i = 0u8;
                    'l14: loop {
                        if !(((i) as i32) < 8i32) {
                            break 'l14;
                        }
                        'l15: {
                            if ((i) as i32) < 4i32 {
                                'l16: loop {
                                    'l17: {
                                        'l18: loop {
                                            'l19: {
                                                CpuSet((GetHealthboxElementGfxPtr(12u8)).wrapping_offset(((((((((&raw mut array).cast::<u8>()).wrapping_offset((((i) as i32)) as isize)).read()) as i32))).wrapping_mul(32i32)) as isize), (((100728832i32).wrapping_add((((((((crate::c::bf_read((((((&raw mut gSprites)).cast::<u8>()).wrapping_offset(((((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).wrapping_add(12).cast::<*mut u8>()).read()).wrapping_offset((((battler) as i32)) as isize * 20))).read()) as i32)) as isize * 68))).wrapping_add(4), 0, 10, false) as u16)) as i32))).wrapping_add(36i32)).wrapping_add((((i) as i32)))).wrapping_mul(crate::c::div_i32(256i32, 8i32)))) as usize as *mut u8), (((67108864i32 | (crate::c::div_i32(32i32, crate::c::div_i32(32i32, 8i32)) & 2097151i32))) as u32));
                                            }
                                            if !((0i32) != 0) {
                                                break 'l18;
                                            }
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l16;
                                    }
                                }
                            } else {
                                'l20: loop {
                                    'l21: {
                                        'l22: loop {
                                            'l23: {
                                                CpuSet((GetHealthboxElementGfxPtr(12u8)).wrapping_offset(((((((((&raw mut array).cast::<u8>()).wrapping_offset((((i) as i32)) as isize)).read()) as i32))).wrapping_mul(32i32)) as isize), (((100731776i32).wrapping_add((((((i) as i32))).wrapping_add(((((crate::c::bf_read((((((&raw mut gSprites)).cast::<u8>()).wrapping_offset(((((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).wrapping_add(12).cast::<*mut u8>()).read()).wrapping_offset((((battler) as i32)) as isize * 20))).read()) as i32)) as isize * 68))).wrapping_add(4), 0, 10, false) as u16)) as i32)))).wrapping_mul(crate::c::div_i32(256i32, 8i32)))) as usize as *mut u8), (((67108864i32 | (crate::c::div_i32(32i32, crate::c::div_i32(32i32, 8i32)) & 2097151i32))) as u32));
                                            }
                                            if !((0i32) != 0) {
                                                break 'l22;
                                            }
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l20;
                                    }
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CalcNewBarValue(
    maxValue: i32,
    oldValue: i32,
    receivedValue: i32,
    currValue: *mut i32,
    scale: u8,
    toAdd: u16,
) -> i32 {
    unsafe {
        let mut maxValue = maxValue;
        let mut oldValue = oldValue;
        let mut receivedValue = receivedValue;
        let mut currValue = currValue;
        let mut scale = scale;
        let mut toAdd = toAdd;
        let mut ret: i32 = 0i32;
        let mut newValue: i32 = 0i32;
        scale = ((((scale) as i32).wrapping_mul(8i32)) as u8);
        if (currValue).read() == (-32768i32) {
            if maxValue < ((scale) as i32) {
                (currValue).write((oldValue << 8));
            } else {
                (currValue).write(oldValue);
            }
        }
        newValue = (oldValue).wrapping_sub(receivedValue);
        if newValue < 0i32 {
            newValue = 0i32;
        } else {
            if newValue > maxValue {
                newValue = maxValue;
            }
        }
        if maxValue < ((scale) as i32) {
            if (newValue == ((currValue).read() >> 8)) && (((currValue).read() & 255i32) == 0i32) {
                return (-1i32);
            }
        } else {
            if newValue == (currValue).read() {
                return (-1i32);
            }
        }
        if maxValue < ((scale) as i32) {
            let mut toAdd: i32 = crate::c::div_i32((maxValue << 8), ((scale) as i32));
            if receivedValue < 0i32 {
                (currValue).write(((currValue).read()).wrapping_add(toAdd));
                ret = ((currValue).read() >> 8);
                if ret >= newValue {
                    (currValue).write((newValue << 8));
                    ret = newValue;
                }
            } else {
                (currValue).write(((currValue).read()).wrapping_sub(toAdd));
                ret = ((currValue).read() >> 8);
                if ((currValue).read() & 255i32) > 0i32 {
                    ret = (ret).wrapping_add(1);
                }
                if ret <= newValue {
                    (currValue).write((newValue << 8));
                    ret = newValue;
                }
            }
        } else {
            if receivedValue < 0i32 {
                (currValue).write(((currValue).read()).wrapping_add(((toAdd) as i32)));
                if (currValue).read() > newValue {
                    (currValue).write(newValue);
                }
                ret = (currValue).read();
            } else {
                (currValue).write(((currValue).read()).wrapping_sub(((toAdd) as i32)));
                if (currValue).read() < newValue {
                    (currValue).write(newValue);
                }
                ret = (currValue).read();
            }
        }
        return ret;
    }
}
pub(crate) unsafe extern "C" fn CalcBarFilledPixels(
    maxValue: i32,
    oldValue: i32,
    receivedValue: i32,
    currValue: *mut i32,
    pixelsArray: *mut u8,
    scale: u8,
) -> u8 {
    unsafe {
        let mut maxValue = maxValue;
        let mut oldValue = oldValue;
        let mut receivedValue = receivedValue;
        let mut currValue = currValue;
        let mut pixelsArray = pixelsArray;
        let mut scale = scale;
        let mut pixels: u8 = 0u8;
        let mut filledPixels: u8 = 0u8;
        let mut totalPixels: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut newValue: i32 = (oldValue).wrapping_sub(receivedValue);
        if newValue < 0i32 {
            newValue = 0i32;
        } else {
            if newValue > maxValue {
                newValue = maxValue;
            }
        }
        totalPixels = ((((scale) as i32).wrapping_mul(8i32)) as u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((scale) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((pixelsArray).wrapping_offset(((i) as i32) as isize)).write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        if maxValue < ((totalPixels) as i32) {
            pixels = ((crate::c::div_i32(
                ((currValue).read()).wrapping_mul(((totalPixels) as i32)),
                maxValue,
            ) >> 8) as u8);
        } else {
            pixels = ((crate::c::div_i32(
                ((currValue).read()).wrapping_mul(((totalPixels) as i32)),
                maxValue,
            )) as u8);
        }
        filledPixels = pixels;
        if (((filledPixels) as i32) == 0i32) && (newValue > 0i32) {
            (pixelsArray).write(1u8);
            filledPixels = 1u8;
        } else {
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < ((scale) as i32)) {
                        break 'l3;
                    }
                    'l4: {
                        if ((pixels) as i32) >= 8i32 {
                            ((pixelsArray).wrapping_offset(((i) as i32) as isize)).write(8u8);
                        } else {
                            ((pixelsArray).wrapping_offset(((i) as i32) as isize)).write(pixels);
                            break 'l3;
                        }
                        pixels = ((((pixels) as i32).wrapping_sub(8i32)) as u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return filledPixels;
    }
}
pub(crate) unsafe extern "C" fn Debug_TestHealthBar(
    barInfo: *mut u8,
    currValue: *mut i32,
    dest: *mut u16,
    unused: i32,
) -> i16 {
    unsafe {
        let mut barInfo = barInfo;
        let mut currValue = currValue;
        let mut dest = dest;
        let mut unused = unused;
        let mut ret: i16 = 0i16;
        let mut var: i16 = 0i16;
        ret = ((CalcNewBarValue(
            ((barInfo).cast::<i32>()).read(),
            ((barInfo).wrapping_add(4).cast::<i32>()).read(),
            ((barInfo).wrapping_add(8).cast::<i32>()).read(),
            currValue,
            ((crate::c::div_i32(48i32, 8i32)) as u8),
            1u16,
        )) as i16);
        Debug_TestHealthBar_Helper(barInfo, currValue, dest);
        if ((barInfo).cast::<i32>()).read() < 48i32 {
            var = (((currValue).read() >> 8) as i16);
        } else {
            var = (((currValue).read()) as i16);
        }
        DummiedOutFunction(((((barInfo).cast::<i32>()).read()) as i16), var, unused);
        return ret;
    }
}
pub(crate) unsafe extern "C" fn Debug_TestHealthBar_Helper(
    barInfo: *mut u8,
    currValue: *mut i32,
    dest: *mut u16,
) {
    unsafe {
        let mut barInfo = barInfo;
        let mut currValue = currValue;
        let mut dest = dest;
        let mut pixels = crate::ffi::Align4([0u8; 6]);
        let mut src = crate::ffi::Align4([0u8; 12]);
        let mut i: u8 = 0u8;
        CalcBarFilledPixels(
            ((barInfo).cast::<i32>()).read(),
            ((barInfo).wrapping_add(4).cast::<i32>()).read(),
            ((barInfo).wrapping_add(8).cast::<i32>()).read(),
            currValue,
            (&raw mut pixels).cast::<u8>(),
            ((crate::c::div_i32(48i32, 8i32)) as u8),
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut src).cast::<u16>()).wrapping_offset(((i) as i32) as isize)).write(
                        ((((crate::c::bf_read((barInfo).wrapping_add(12), 0, 5, false) as u32)
                            << 12)
                            | (((barInfo).wrapping_add(16).cast::<u32>()).read()).wrapping_add(
                                (((((&raw mut pixels).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read()) as u32),
                            )) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        'l3: loop {
            'l4: {
                'l5: loop {
                    'l6: {
                        CpuSet(
                            ((&raw mut src).cast::<u16>()).cast::<u8>(),
                            (dest).cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    12u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l5;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l3;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetScaledExpFraction(
    oldValue: i32,
    receivedValue: i32,
    maxValue: i32,
    scale: u8,
) -> u8 {
    unsafe {
        let mut oldValue = oldValue;
        let mut receivedValue = receivedValue;
        let mut maxValue = maxValue;
        let mut scale = scale;
        let mut newVal: i32 = 0i32;
        let mut result: i32 = 0i32;
        let mut oldToMax: i8 = 0i8;
        let mut newToMax: i8 = 0i8;
        scale = ((((scale) as i32).wrapping_mul(8i32)) as u8);
        newVal = (oldValue).wrapping_sub(receivedValue);
        if newVal < 0i32 {
            newVal = 0i32;
        } else {
            if newVal > maxValue {
                newVal = maxValue;
            }
        }
        oldToMax = ((crate::c::div_i32((oldValue).wrapping_mul(((scale) as i32)), maxValue)) as i8);
        newToMax = ((crate::c::div_i32((newVal).wrapping_mul(((scale) as i32)), maxValue)) as i8);
        result = ((oldToMax) as i32).wrapping_sub(((newToMax) as i32));
        return ((if result < 0i32 {
            (result).wrapping_neg()
        } else {
            result
        }) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetScaledHPFraction(hp: i16, maxhp: i16, scale: u8) -> u8 {
    unsafe {
        let mut hp = hp;
        let mut maxhp = maxhp;
        let mut scale = scale;
        let mut result: u8 = ((crate::c::div_i32(
            ((hp) as i32).wrapping_mul(((scale) as i32)),
            ((maxhp) as i32),
        )) as u8);
        if (((result) as i32) == 0i32) && (((hp) as i32) > 0i32) {
            return 1u8;
        }
        return result;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetHPBarLevel(hp: i16, maxhp: i16) -> u8 {
    unsafe {
        let mut hp = hp;
        let mut maxhp = maxhp;
        let mut result: u8 = 0u8;
        if ((hp) as i32) == ((maxhp) as i32) {
            result = 4u8;
        } else {
            let mut fraction: u8 = GetScaledHPFraction(hp, maxhp, 48u8);
            if ((fraction) as i32) > crate::c::div_i32(2400i32, 100i32) {
                result = 3u8;
            } else {
                if ((fraction) as i32) > crate::c::div_i32(960i32, 100i32) {
                    result = 2u8;
                } else {
                    if ((fraction) as i32) > 0i32 {
                        result = 1u8;
                    } else {
                        result = 0u8;
                    }
                }
            }
        }
        return result;
    }
}
pub(crate) unsafe extern "C" fn AddTextPrinterAndCreateWindowOnHealthbox(
    str: *mut u8,
    x: u32,
    y: u32,
    bgColor: u32,
    windowId: *mut u32,
) -> *mut u8 {
    unsafe {
        let mut str = str;
        let mut x = x;
        let mut y = y;
        let mut bgColor = bgColor;
        let mut windowId = windowId;
        let mut winId: u16 = 0u16;
        let mut color = crate::ffi::Align4([0u8; 3]);
        let mut winTemplate = crate::ffi::Align4([0u8; 8]);
        (&raw mut winTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw const sHealthboxWindowTemplate)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
        winId = AddWindow((&raw mut winTemplate).cast::<u8>());
        FillWindowPixelBuffer(((winId) as u8), ((bgColor | (bgColor << 4)) as u8));
        ((&raw mut color).cast::<u8>()).write(((bgColor) as u8));
        (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(1u8);
        (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(3u8);
        AddTextPrinterParameterized4(
            ((winId) as u8),
            0u8,
            ((x) as u8),
            ((y) as u8),
            0u8,
            0u8,
            (&raw mut color).cast::<u8>(),
            (-1i8),
            str,
        );
        (windowId).write(((winId) as u32));
        return ((GetWindowAttribute(((winId) as u8), 7u8)) as usize as *mut u8);
    }
}
pub(crate) unsafe extern "C" fn RemoveWindowOnHealthbox(windowId: u32) {
    unsafe {
        let mut windowId = windowId;
        RemoveWindow(((windowId) as u8));
    }
}
pub(crate) unsafe extern "C" fn FillHealthboxObject(dest: *mut u8, valMult: u32, numTiles: u32) {
    unsafe {
        let mut dest = dest;
        let mut valMult = valMult;
        let mut numTiles = numTiles;
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile((286331153u32).wrapping_mul(valMult));
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                dest,
                                (83886080u32
                                    | (crate::c::div_u32(
                                        (numTiles).wrapping_mul(
                                            ((crate::c::div_i32(256i32, 8i32)) as u32),
                                        ),
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HpTextIntoHealthboxObject(
    dest: *mut u8,
    windowTileData: *mut u8,
    windowWidth: u32,
) {
    unsafe {
        let mut dest = dest;
        let mut windowTileData = windowTileData;
        let mut windowWidth = windowWidth;
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (windowTileData).wrapping_offset(256),
                            dest,
                            (67108864u32
                                | (crate::c::div_u32(
                                    (windowWidth)
                                        .wrapping_mul(((crate::c::div_i32(256i32, 8i32)) as u32)),
                                    ((crate::c::div_i32(32i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TextIntoHealthboxObject(
    dest: *mut u8,
    windowTileData: *mut u8,
    windowWidth: i32,
) {
    unsafe {
        let mut dest = dest;
        let mut windowTileData = windowTileData;
        let mut windowWidth = windowWidth;
        let mut i: i32 = 0i32;
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (windowTileData).wrapping_offset(256),
                            (dest).wrapping_offset(256),
                            ((67108864i32
                                | (crate::c::div_i32(
                                    (windowWidth).wrapping_mul(crate::c::div_i32(256i32, 8i32)),
                                    crate::c::div_i32(32i32, 8i32),
                                ) & 2097151i32)) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < windowWidth) {
                    break 'l5;
                }
                'l6: {
                    'l7: loop {
                        'l8: {
                            'l9: loop {
                                'l10: {
                                    CpuSet(
                                        (windowTileData).wrapping_offset(20),
                                        (dest).wrapping_offset(20),
                                        ((67108864i32
                                            | (crate::c::div_i32(
                                                12i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l9;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                    dest = (dest).wrapping_offset(32);
                    windowTileData = (windowTileData).wrapping_offset(32);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SafariTextIntoHealthboxObject(
    dest: *mut u8,
    windowTileData: *mut u8,
    windowWidth: u32,
) {
    unsafe {
        let mut dest = dest;
        let mut windowTileData = windowTileData;
        let mut windowWidth = windowWidth;
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            windowTileData,
                            dest,
                            (67108864u32
                                | (crate::c::div_u32(
                                    (windowWidth)
                                        .wrapping_mul(((crate::c::div_i32(256i32, 8i32)) as u32)),
                                    ((crate::c::div_i32(32i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l5: loop {
            'l6: {
                'l7: loop {
                    'l8: {
                        CpuSet(
                            (windowTileData).wrapping_offset(256),
                            (dest).wrapping_offset(256),
                            (67108864u32
                                | (crate::c::div_u32(
                                    (windowWidth)
                                        .wrapping_mul(((crate::c::div_i32(256i32, 8i32)) as u32)),
                                    ((crate::c::div_i32(32i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l7;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
