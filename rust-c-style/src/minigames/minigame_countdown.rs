//! Translated from `src/minigame_countdown.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
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
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): s321Start_Static_Pal s321Start_Static_Gfx sSpriteSheet_321Start_Static sSpritePalette_321Start_Static sAnim_StaticCountdown_Three sAnim_StaticCountdown_Two sAnim_StaticCountdown_One sAnim_StaticCountdown_StartLeft sAnim_StaticCountdown_StartMid sAnim_StaticCountdown_StartRight sAnims_StaticCountdown sSpriteTemplate_StaticCountdown sStaticCountdownFuncs s321Start_Pal s321Start_Gfx sOamData_Numbers sOamData_Start sAnim_Numbers_Three sAnim_Numbers_Two sAnim_Numbers_One sAnimTable_Numbers sAnim_StartLeft sAnim_StartRight sAnimTable_Start sAffineAnim_Numbers_Normal sAffineAnim_Numbers_Squash sAffineAnim_Numbers_Stretch sAffineAnim_Numbers_Land sAffineAnimTable_Numbers

const ANIM_ONE: i16 = 2;
const ANIM_START_LEFT: i16 = 3;
const ANIM_START_MID: u8 = 4;
const ANIM_START_RIGHT: u8 = 5;
const ANIM_THREE: i16 = 0;
const ANIM_TWO: i16 = 1;
const FUNC_FREE: i32 = 1;
const FUNC_INIT: i32 = 0;
const FUNC_RUN: i32 = 3;
const FUNC_START: i32 = 2;
const STATE_END: i16 = 4;
const STATE_IDLE: i16 = 1;
const STATE_RUN: i16 = 3;
const STATE_START: i16 = 2;

static s321Start_Gfx: Table<CArray<u32, 277>> =
    Table((&raw const crate::data::minigame_countdown::s321Start_Gfx).cast());
static s321Start_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::minigame_countdown::s321Start_Pal).cast());
static sAffineAnimTable_Numbers: Table<CArray<*mut AffineAnimCmd, 4>> =
    Table((&raw const crate::data::minigame_countdown::sAffineAnimTable_Numbers).cast());
static sAnimTable_Numbers: Table<CArray<*mut AnimCmd, 3>> =
    Table((&raw const crate::data::minigame_countdown::sAnimTable_Numbers).cast());
static sAnimTable_Start: Table<CArray<*mut AnimCmd, 2>> =
    Table((&raw const crate::data::minigame_countdown::sAnimTable_Start).cast());
static sOamData_Numbers: Table<OamData> =
    Table((&raw const crate::data::minigame_countdown::sOamData_Numbers).cast());
static sOamData_Start: Table<OamData> =
    Table((&raw const crate::data::minigame_countdown::sOamData_Start).cast());
static sSpritePalette_321Start_Static: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::minigame_countdown::sSpritePalette_321Start_Static).cast());
static sSpriteSheet_321Start_Static: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::minigame_countdown::sSpriteSheet_321Start_Static).cast());
static sSpriteTemplate_StaticCountdown: Table<CArray<SpriteTemplate, 1>> =
    Table((&raw const crate::data::minigame_countdown::sSpriteTemplate_StaticCountdown).cast());
static sStaticCountdownFuncs: Table<CArray<CArray<Option<unsafe extern "C" fn(u8)>, 4>, 1>> =
    Table((&raw const crate::data::minigame_countdown::sStaticCountdownFuncs).cast());

unsafe extern "C" {
    static gDummySpriteAffineAnimTable: CArray<*mut AffineAnimCmd, 0>;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRecvCmds: CArray<CArray<u16, 8>, 5>;
    static gSineTable: CArray<i16, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FreeSpriteOamMatrix(a0: *mut Sprite);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetMultiplayerId() -> u8;
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn PlaySE(a0: u16);
    fn Rfu_SendPacket(a0: *mut c_void);
    fn SetSpriteMatrixAnchor(a0: *mut Sprite, a1: i16, a2: i16);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
}

pub(crate) unsafe extern "C" fn CreateStaticCountdownTask(funcSetId: u8, taskPriority: u8) -> u32 {
    let mut taskId: u8 = CreateTask(Some(Task_StaticCountdown), taskPriority);
    let mut task: *mut Task = &raw mut gTasks[taskId];
    (*task).data[0] = STATE_IDLE;
    (*task).data[1] = funcSetId as i16;
    sStaticCountdownFuncs[funcSetId][0].unwrap_unchecked()(taskId);
    return taskId as u32;
}
pub(crate) unsafe extern "C" fn StartStaticCountdown() -> u32 {
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_StaticCountdown));
    if taskId == TASK_NONE {
        return FALSE as u32;
    }
    gTasks[taskId].data[0] = STATE_START;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn IsStaticCountdownRunning() -> u32 {
    return FuncIsActiveTask(Some(Task_StaticCountdown)) as u32;
}
pub(crate) unsafe extern "C" fn Task_StaticCountdown(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        STATE_START => {
            sStaticCountdownFuncs[*data.at(1)][2].unwrap_unchecked()(taskId);
            *data = STATE_RUN;
        }
        STATE_RUN => {
            sStaticCountdownFuncs[*data.at(1)][3].unwrap_unchecked()(taskId);
        }
        STATE_END => {
            sStaticCountdownFuncs[*data.at(1)][1].unwrap_unchecked()(taskId);
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn StaticCountdown_CreateSprites(taskId: u8, mut data: *mut i16) {
    let mut i: u8 = 0;
    let mut sprite: *mut Sprite = null_mut();
    LoadCompressedSpriteSheet((&raw const sSpriteSheet_321Start_Static[*data.at(3)]).cast_mut());
    LoadSpritePalette((&raw const sSpritePalette_321Start_Static[*data.at(4)]).cast_mut());
    i = 0;
    while (i as i16) < *data.at(8) {
        *data.at(13 + i as i32) = CreateSprite(
            (&raw const sSpriteTemplate_StaticCountdown[*data.at(2)]).cast_mut(),
            *data.at(9),
            *data.at(10),
            *data.at(7) as u8,
        ) as i16;
        i += 1;
    }
    i = 0;
    while (i as i16) < *data.at(8) {
        sprite = &raw mut gSprites[*data.at(13 + i as i32)];
        (*sprite).oam.set_priority(*data.at(6) as u16);
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).data[1] = *data.at(5);
        (*sprite).data[3] = taskId as i16;
        (*sprite).data[4] = i as i16;
        (*sprite).data[5] = *data.at(13);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn Task_StaticCountdown_Init(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data.at(2) = 0;
    *data.at(3) = 0;
    *data.at(4) = 0;
    *data.at(5) = 60;
    *data.at(6) = 0;
    *data.at(7) = 0;
    *data.at(8) = 3;
    *data.at(9) = 120;
    *data.at(10) = 88;
    StaticCountdown_CreateSprites(taskId, data);
    StartSpriteAnim(&raw mut gSprites[*data.at(14)], ANIM_START_MID);
    gSprites[*data.at(14)].x2 = -32;
    StartSpriteAnim(&raw mut gSprites[*data.at(15)], ANIM_START_RIGHT);
    gSprites[*data.at(15)].x2 = 32;
}
pub(crate) unsafe extern "C" fn Task_StaticCountdown_Free(taskId: u8) {
    let mut i: u8 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    i = 0;
    while (i as i16) < *data.at(8) {
        DestroySprite(&raw mut gSprites[*data.at(13 + i as i32)]);
        i += 1;
    }
    FreeSpriteTilesByTag(sSpriteSheet_321Start_Static[*data.at(3)].tag);
    FreeSpritePaletteByTag(sSpritePalette_321Start_Static[*data.at(4)].tag);
}
pub(crate) unsafe extern "C" fn SpriteCB_StaticCountdown(sprite: *mut Sprite) {
    let mut data: *mut i16 = gTasks[(*sprite).data[3]].data.as_mut_ptr();
    if rem_i32(*data.at(11) as i32, *data.at(5) as i32) != 0 {
        return;
    }
    if *data.at(11) == *data.at(10) {
        return;
    }
    *data.at(10) = *data.at(11);
    'l1: {
        let sw1: i16 = (*sprite).data[2];
        let mut fall = false;
        if sw1 == ANIM_THREE {
            fall = true;
            (*sprite).set_invisible(FALSE as u16);
        }
        if fall || sw1 == ANIM_TWO || sw1 == ANIM_ONE {
            fall = true;
            PlaySE(SE_BALL_BOUNCE_1);
            StartSpriteAnim(sprite, (*sprite).data[2] as u8);
            break 'l1;
        }
        if sw1 == ANIM_START_LEFT {
            fall = true;
            PlaySE(SE_PIN);
            StartSpriteAnim(sprite, (*sprite).data[2] as u8);
            gSprites[*data.at(14)].set_invisible(FALSE as u16);
            gSprites[*data.at(15)].set_invisible(FALSE as u16);
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            (*sprite).set_invisible(TRUE as u16);
            gSprites[*data.at(14)].set_invisible(1);
            gSprites[*data.at(15)].set_invisible(TRUE as u16);
            *data = STATE_END;
            return;
        }
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe extern "C" fn Task_StaticCountdown_Start(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    PlaySE(SE_BALL_BOUNCE_1);
    gSprites[*data.at(13)].callback = Some(SpriteCB_StaticCountdown);
    gSprites[*data.at(13)].set_invisible(0);
    gTasks[taskId].data[0] = STATE_RUN;
}
pub(crate) unsafe extern "C" fn Task_StaticCountdown_Run(taskId: u8) {
    let mut packet: CArray<u16, 6> = zeroed();
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if gReceivedRemoteLinkPlayers != 0 {
        if gRecvCmds[0][1] == LINKCMD_COUNTDOWN {
            *data.at(11) = gRecvCmds[0][2] as i16;
        }
        if GetMultiplayerId() == 0 {
            *data.at(12) += 1;
            memset(packet.as_mut_ptr() as *mut u8, 0, 12);
            packet[0] = LINKCMD_COUNTDOWN;
            packet[1] = *data.at(12) as u16;
            Rfu_SendPacket(packet.as_mut_ptr() as *mut c_void);
        }
    } else {
        *data.at(11) += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartMinigameCountdown(
    tilesTag: u16,
    palTag: u16,
    x: i16,
    y: i16,
    subpriority: u8,
) {
    let mut taskId: u8 = CreateTask(Some(Task_MinigameCountdown), 80);
    gTasks[taskId].data[2] = tilesTag as i16;
    gTasks[taskId].data[3] = palTag as i16;
    gTasks[taskId].data[4] = x;
    gTasks[taskId].data[5] = y;
    gTasks[taskId].data[6] = subpriority as i16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMinigameCountdownRunning() -> u32 {
    return FuncIsActiveTask(Some(Task_MinigameCountdown)) as u32;
}
pub(crate) unsafe extern "C" fn Task_MinigameCountdown(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            Load321StartGfx(*data.at(2) as u16, *data.at(3) as u16);
            *data.at(7) = CreateNumberSprite(
                *data.at(2) as u16,
                *data.at(3) as u16,
                *data.at(4),
                *data.at(5),
                *data.at(6) as u8,
            ) as i16;
            CreateStartSprite(
                *data.at(2) as u16,
                *data.at(3) as u16,
                *data.at(4),
                *data.at(5),
                *data.at(6) as u8,
                data.at(8),
                data.at(9),
            );
            *data += 1;
        }
        1 => {
            if RunMinigameCountdownDigitsAnim(*data.at(7) as u8) == 0 {
                InitStartGraphic(*data.at(7) as u8, *data.at(8) as u8, *data.at(9) as u8);
                FreeSpriteOamMatrix(&raw mut gSprites[*data.at(7)]);
                DestroySprite(&raw mut gSprites[*data.at(7)]);
                *data += 1;
            }
        }
        2 => {
            if IsStartGraphicAnimRunning(*data.at(8) as u8) == 0 {
                DestroySprite(&raw mut gSprites[*data.at(8)]);
                DestroySprite(&raw mut gSprites[*data.at(9)]);
                FreeSpriteTilesByTag(*data.at(2) as u16);
                FreeSpritePaletteByTag(*data.at(3) as u16);
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn RunMinigameCountdownDigitsAnim(spriteId: u8) -> u32 {
    let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            SetSpriteMatrixAnchor(sprite, NO_ANCHOR, 26);
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if (*sprite).data[2] == 0 {
                PlaySE(SE_BALL_BOUNCE_2);
            }
            if ({
                (*sprite).data[2] += 1;
                (*sprite).data[2]
            }) >= 20
            {
                (*sprite).data[2] = 0;
                StartSpriteAffineAnim(sprite, 1);
                (*sprite).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if (*sprite).affineAnimEnded() != 0 {
                (*sprite).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            if ({
                (*sprite).data[2] += 1;
                (*sprite).data[2]
            }) >= 4
            {
                (*sprite).data[2] = 0;
                (*sprite).data[0] += 1;
                StartSpriteAffineAnim(sprite, 2);
            }
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            (*sprite).y -= 4;
            if ({
                (*sprite).data[2] += 1;
                (*sprite).data[2]
            }) >= 8
            {
                if (*sprite).data[4] < 2 {
                    StartSpriteAnim(sprite, (*sprite).data[4] as u8 + 1);
                    (*sprite).data[2] = 0;
                    (*sprite).data[0] += 1;
                } else {
                    (*sprite).data[0] = 7;
                    return FALSE as u32;
                }
            }
            break 'l1;
        }
        if sw1 == 5 {
            fall = true;
            (*sprite).y += 4;
            if ({
                (*sprite).data[2] += 1;
                (*sprite).data[2]
            }) >= 8
            {
                (*sprite).data[2] = 0;
                StartSpriteAffineAnim(sprite, 3);
                (*sprite).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 6 {
            fall = true;
            if (*sprite).affineAnimEnded() != 0 {
                (*sprite).data[4] += 1;
                (*sprite).data[0] = 1;
            }
            break 'l1;
        }
        if sw1 == 7 {
            fall = true;
            return FALSE as u32;
        }
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn InitStartGraphic(spriteId1: u8, spriteId2: u8, spriteId3: u8) {
    gSprites[spriteId2].y2 = -40;
    gSprites[spriteId3].y2 = -40;
    gSprites[spriteId2].set_invisible(FALSE as u16);
    gSprites[spriteId3].set_invisible(FALSE as u16);
    gSprites[spriteId2].callback = Some(SpriteCB_Start);
    gSprites[spriteId3].callback = Some(SpriteCB_Start);
}
pub(crate) unsafe extern "C" fn IsStartGraphicAnimRunning(spriteId: u8) -> u32 {
    return (gSprites[spriteId].callback
        == Some(SpriteCB_Start as unsafe extern "C" fn(*mut Sprite))) as u32;
}
pub(crate) unsafe extern "C" fn SpriteCB_Start(sprite: *mut Sprite) {
    let mut y: i32 = 0;
    let mut data: *mut i16 = (*sprite).data.as_mut_ptr();
    'l1: {
        let sw1: i16 = *data;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            *data.at(4) = 64;
            *data.at(5) = (*sprite).y2 << 4;
            *data += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            *data.at(5) += *data.at(4);
            *data.at(4) += 1;
            (*sprite).y2 = *data.at(5) >> 4;
            if (*sprite).y2 >= 0 {
                PlaySE(SE_BALL_BOUNCE_2);
                (*sprite).y2 = 0;
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            *data.at(1) += 12;
            if *data.at(1) >= 128 {
                PlaySE(SE_BALL_BOUNCE_2);
                *data.at(1) = 0;
                *data += 1;
            }
            y = gSineTable[*data.at(1)] as i32;
            (*sprite).y2 = -((y >> 4) as i16);
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            *data.at(1) += 16;
            if *data.at(1) >= 128 {
                PlaySE(SE_BALL_BOUNCE_2);
                *data.at(1) = 0;
                *data += 1;
            }
            (*sprite).y2 = -(gSineTable[*data.at(1)] >> 5);
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            if ({
                *data.at(1) += 1;
                *data.at(1)
            }) > 40
            {
                (*sprite).callback = Some(SpriteCallbackDummy);
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn Load321StartGfx(tileTag: u16, palTag: u16) {
    let mut spriteSheet: CompressedSpriteSheet = zeroed();
    spriteSheet.data = s321Start_Gfx.as_ptr().cast_mut();
    spriteSheet.size = 0xE00;
    spriteSheet.tag = 0;
    let mut spritePalette: SpritePalette = zeroed();
    spritePalette.data = s321Start_Pal.as_ptr().cast_mut();
    spritePalette.tag = 0;
    spriteSheet.tag = tileTag;
    spritePalette.tag = palTag;
    LoadCompressedSpriteSheet(&raw mut spriteSheet);
    LoadSpritePalette(&raw mut spritePalette);
}
pub(crate) unsafe extern "C" fn CreateNumberSprite(
    tileTag: u16,
    palTag: u16,
    x: i16,
    y: i16,
    subpriority: u8,
) -> u8 {
    let mut spriteTemplate: SpriteTemplate = zeroed();
    spriteTemplate.oam = (&raw const *sOamData_Numbers).cast_mut();
    spriteTemplate.anims = sAnimTable_Numbers.as_ptr().cast_mut();
    spriteTemplate.affineAnims = sAffineAnimTable_Numbers.as_ptr().cast_mut();
    spriteTemplate.callback = Some(SpriteCallbackDummy);
    spriteTemplate.tileTag = tileTag;
    spriteTemplate.paletteTag = palTag;
    return CreateSprite(&raw mut spriteTemplate, x, y, subpriority);
}
pub(crate) unsafe extern "C" fn CreateStartSprite(
    tileTag: u16,
    palTag: u16,
    x: i16,
    y: i16,
    subpriority: u8,
    spriteId1: *mut i16,
    spriteId2: *mut i16,
) {
    let mut spriteTemplate: SpriteTemplate = zeroed();
    spriteTemplate.oam = (&raw const *sOamData_Start).cast_mut();
    spriteTemplate.anims = sAnimTable_Start.as_ptr().cast_mut();
    spriteTemplate.affineAnims = gDummySpriteAffineAnimTable.as_ptr().cast_mut();
    spriteTemplate.callback = Some(SpriteCallbackDummy);
    spriteTemplate.tileTag = tileTag;
    spriteTemplate.paletteTag = palTag;
    *spriteId1 = CreateSprite(&raw mut spriteTemplate, x - 32, y, subpriority) as i16;
    *spriteId2 = CreateSprite(&raw mut spriteTemplate, x + 32, y, subpriority) as i16;
    gSprites[*spriteId1].set_invisible(TRUE as u16);
    gSprites[*spriteId2].set_invisible(TRUE as u16);
    StartSpriteAnim(&raw mut gSprites[*spriteId2], 1);
}
