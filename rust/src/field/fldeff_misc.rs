//! Translated from `src/fldeff_misc.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sSecretPowerCave_Gfx sFiller sSecretPowerCave_Pal sSecretPowerShrub_Gfx sSecretPowerTree_Gfx sSecretPowerPlant_Pal sSandPillar0_Gfx sSandPillar1_Gfx sSandPillar2_Gfx sOam_SecretPower sAnim_SecretPowerCave sAnim_VineDropLeft sAnim_VineRiseLeft sAnim_VineDropRight sAnim_VineRiseRight sAnim_SecretPowerShrub sAnimTable_SecretPowerCave sAnimTable_SecretPowerTree sAnimTable_SecretPowerShrub sPicTable_SecretPowerCave sPicTable_SecretPowerTree sPicTable_SecretPowerShrub sSpriteTemplate_SecretPowerCave sSpriteTemplate_SecretPowerTree sSpriteTemplate_SecretPowerShrub gSpritePalette_SecretPower_Cave gSpritePalette_SecretPower_Plant sOam_SandPillar sAnim_SandPillar sAnimTable_SandPillar sPicTable_SandPillar sSpriteTemplate_SandPillar gSpritePalette_SandPillar sRecordMixLights_Gfx sRecordMixLights_Pal sPicTable_RecordMixLights sSpritePalette_RecordMixLights sAnim_RecordMixLights sAnimTable_RecordMixLights sSpriteTemplate_RecordMixLights
#[allow(unused_imports)]
use crate::data::fldeff_misc::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerFacingPosition: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);

unsafe extern "C" {
    static mut SecretBase_EventScript_CaveUseSecretPower: u8;
    static mut SecretBase_EventScript_ShrubUseSecretPower: u8;
    static mut SecretBase_EventScript_TreeUseSecretPower: u8;
    static mut gFieldCallback2: u8;
    static mut gFieldEffectArguments: u8;
    static mut gFieldEffectObjectTemplatePointers: u8;
    static mut gMapHeader: u8;
    static mut gObjectEvents: u8;
    static mut gPlayerAvatar: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPostMenuFieldCallback: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gTasks: u8;
    static mut gText_Gold: u8;
    static mut gText_Silver: u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn CheckPlayerHasSecretBase();
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CreateFieldMoveTask() -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateSpriteAtEnd(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CurMapIsSecretBase() -> u8;
    fn CurrentMapDrawMetatileAt(a0: i32, a1: i32);
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn FieldCallback_PrepareFadeInFromMenu() -> u8;
    fn FieldEffectActiveListRemove(a0: u8);
    fn FieldEffectStart(a0: u8) -> u32;
    fn FieldEffectStop(a0: *mut u8, a1: u8);
    fn FreeSpritePalette(a0: *mut u8);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetCursorSelectionMonId() -> u8;
    fn GetGpuReg(a0: u8) -> u16;
    fn GetMapCoordsFromSpritePos(a0: i16, a1: i16, a2: *mut i16, a3: *mut i16);
    fn GetPlayerAvatarFlags() -> u8;
    fn GetPlayerFacingDirection() -> u8;
    fn GetWalkInPlaceNormalMovementAction(a0: u32) -> u8;
    fn GetXYCoordsOneStepInFrontOfPlayer(a0: *mut i16, a1: *mut i16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LockPlayerFieldControls();
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MapGridGetMetatileIdAt(a0: i32, a1: i32) -> i32;
    fn MapGridSetMetatileIdAt(a0: i32, a1: i32, a2: u16);
    fn MetatileBehavior_IsSecretBaseCave(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseShrub(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseTree(a0: u8) -> u8;
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut u8) -> u8;
    fn ObjectEventIsMovementOverridden(a0: *mut u8) -> u8;
    fn ObjectEventSetHeldMovement(a0: *mut u8, a1: u8) -> u8;
    fn PlaySE(a0: u16);
    fn ScriptContext_Enable();
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SetCurSecretBaseIdFromPosition(a0: *mut u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetPlayerAvatarTransitionFlags(a0: u16);
    fn SetPlayerAvatarWatering(a0: u8);
    fn SetSpritePosToOffsetMapCoords(a0: *mut i16, a1: *mut i16, a2: i16, a3: i16);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn ToggleSecretBaseEntranceMetatile();
    fn TrySetCurSecretBaseIndex();
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ComputerScreenOpenEffect(increment: u16, unused: u16, priority: u8) {
    unsafe {
        let mut increment = increment;
        let mut unused = unused;
        let mut priority = priority;
        CreateComputerScreenEffectTask(
            Some(Task_ComputerScreenOpenEffect),
            increment,
            unused,
            priority,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ComputerScreenCloseEffect(increment: u16, unused: u16, priority: u8) {
    unsafe {
        let mut increment = increment;
        let mut unused = unused;
        let mut priority = priority;
        CreateComputerScreenEffectTask(
            Some(Task_ComputerScreenCloseEffect),
            increment,
            unused,
            priority,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsComputerScreenOpenEffectActive() -> u8 {
    unsafe {
        return FuncIsActiveTask(Some(Task_ComputerScreenOpenEffect));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsComputerScreenCloseEffectActive() -> u8 {
    unsafe {
        return FuncIsActiveTask(Some(Task_ComputerScreenCloseEffect));
    }
}
pub(crate) unsafe extern "C" fn CreateComputerScreenEffectTask(
    func: Option<unsafe extern "C" fn(u8)>,
    increment: u16,
    unused: u16,
    priority: u8,
) {
    unsafe {
        let mut func = func;
        let mut increment = increment;
        let mut unused = unused;
        let mut priority = priority;
        let mut taskId: u8 = CreateTask(func, priority);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            ((if ((increment) as i32) == 0i32 {
                16i32
            } else {
                ((increment) as i32)
            }) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((if ((increment) as i32) == 0i32 {
                20i32
            } else {
                ((increment) as i32)
            }) as i16),
        );
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read())
        .unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_ComputerScreenOpenEffect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
                    .write(((crate::c::div_i32(240i32, 2i32)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                    .write(((crate::c::div_i32(240i32, 2i32)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                    .write(((crate::c::div_i32(160i32, 2i32)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                    .write((((crate::c::div_i32(160i32, 2i32)).wrapping_add(1i32)) as i16));
                SetGpuRegBits(0u8, 8192u16);
                SetGpuReg(
                    64u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32)) as u16),
                );
                SetGpuReg(
                    68u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32)) as u16),
                );
                SetGpuReg(72u8, 63u16);
                SetGpuReg(74u8, 0u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7))
                    .write(((GetGpuReg(80u8)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8))
                    .write(((GetGpuReg(84u8)) as i16));
                SetGpuReg(80u8, 191u16);
                SetGpuReg(84u8, 16u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as i16),
                );
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as i16),
                );
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    < 1i32)
                    || (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        > 239i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(240i16);
                    SetGpuReg(84u8, 0u16);
                    SetGpuReg(
                        80u8,
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                            as u16),
                    );
                    BlendPalettes(4294967295u32, 0u8, 0u16);
                    (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).write(0u16);
                }
                SetGpuReg(
                    64u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32)) as u16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    != 0i32
                {
                    return;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                    )) as i16),
                );
                let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                    )) as i16),
                );
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    < 1i32)
                    || (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        > 159i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(160i16);
                    ClearGpuRegBits(0u8, 8192u16);
                }
                SetGpuReg(
                    68u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32)) as u16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    != 0i32
                {
                    return;
                }
                break 'l1;
            }
            if !__matched {
                SetGpuReg(
                    80u8,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as u16),
                );
                DestroyTask(taskId);
                return;
            }
        }
        let __p6 = ((task).wrapping_add(8)).cast::<i16>();
        (__p6).write(((__p6).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Task_ComputerScreenCloseEffect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 {
                (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).write(0u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(240i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(160i16);
                SetGpuRegBits(0u8, 8192u16);
                SetGpuReg(
                    64u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32)) as u16),
                );
                SetGpuReg(
                    68u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32)) as u16),
                );
                SetGpuReg(72u8, 63u16);
                SetGpuReg(74u8, 0u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                    )) as i16),
                );
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                    )) as i16),
                );
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    >= crate::c::div_i32(160i32, 2i32))
                    || (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        <= (crate::c::div_i32(160i32, 2i32)).wrapping_add(1i32))
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                        .write(((crate::c::div_i32(160i32, 2i32)) as i16));
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                        .write((((crate::c::div_i32(160i32, 2i32)).wrapping_add(1i32)) as i16));
                    SetGpuReg(80u8, 191u16);
                    SetGpuReg(84u8, 16u16);
                }
                SetGpuReg(
                    68u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32)) as u16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    != crate::c::div_i32(160i32, 2i32)
                {
                    return;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as i16),
                );
                let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as i16),
                );
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    >= crate::c::div_i32(240i32, 2i32))
                    || (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        <= crate::c::div_i32(240i32, 2i32))
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
                        .write(((crate::c::div_i32(240i32, 2i32)) as i16));
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                        .write(((crate::c::div_i32(240i32, 2i32)) as i16));
                    BlendPalettes(4294967295u32, 16u8, 0u16);
                    (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).write(0u16);
                }
                SetGpuReg(
                    64u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32)) as u16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    != crate::c::div_i32(240i32, 2i32)
                {
                    return;
                }
                break 'l1;
            }
            if !__matched {
                ClearGpuRegBits(0u8, 8192u16);
                SetGpuReg(84u8, 0u16);
                SetGpuReg(80u8, 0u16);
                DestroyTask(taskId);
                return;
            }
        }
        let __p6 = ((task).wrapping_add(8)).cast::<i16>();
        (__p6).write(((__p6).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn SetCurrentSecretBase() {
    unsafe {
        SetCurSecretBaseIdFromPosition(
            (&raw mut gPlayerFacingPosition).cast::<u8>(),
            (((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read(),
        );
        TrySetCurSecretBaseIndex();
    }
}
pub(crate) unsafe extern "C" fn AdjustSecretPowerSpritePixelOffsets() {
    unsafe {
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 6i32) != 0 {
            'l1: {
                let __sw1 = ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(1))
                .read();
                if __sw1 == 1i32 {
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(5))
                    .write(16i32);
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(6))
                    .write(40i32);
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(5))
                    .write(16i32);
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(6))
                    .write(8i32);
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(5))
                    .write((-8i32));
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(6))
                    .write(24i32);
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(5))
                    .write(24i32);
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(6))
                    .write(24i32);
                    break 'l1;
                }
            }
        } else {
            'l2: {
                let __sw2 = ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(1))
                .read();
                if __sw2 == 1i32 {
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(5))
                    .write(8i32);
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(6))
                    .write(40i32);
                    break 'l2;
                }
                if __sw2 == 2i32 {
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(5))
                    .write(8i32);
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(6))
                    .write(8i32);
                    break 'l2;
                }
                if __sw2 == 3i32 {
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(5))
                    .write((-8i32));
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(6))
                    .write(24i32);
                    break 'l2;
                }
                if __sw2 == 4i32 {
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(5))
                    .write(24i32);
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(6))
                    .write(24i32);
                    break 'l2;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpFieldMove_SecretPower() -> u8 {
    unsafe {
        let mut mb: u8 = 0u8;
        CheckPlayerHasSecretBase();
        if (((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 1i32)
            || (((GetPlayerFacingDirection()) as i32) != 2i32)
        {
            return 0u8;
        }
        GetXYCoordsOneStepInFrontOfPlayer(
            ((&raw mut gPlayerFacingPosition).cast::<u8>()).cast::<i16>(),
            ((&raw mut gPlayerFacingPosition).cast::<u8>())
                .wrapping_add(2)
                .cast::<i16>(),
        );
        mb = ((MapGridGetMetatileBehaviorAt(
            (((((&raw mut gPlayerFacingPosition).cast::<u8>()).cast::<i16>()).read()) as i32),
            (((((&raw mut gPlayerFacingPosition).cast::<u8>())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32),
        )) as u8);
        if ((MetatileBehavior_IsSecretBaseCave(mb)) as i32) == 1i32 {
            SetCurrentSecretBase();
            ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(Some(FieldCallback_PrepareFadeInFromMenu));
            ((&raw mut gPostMenuFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCallback_SecretBaseCave));
            return 1u8;
        }
        if ((MetatileBehavior_IsSecretBaseTree(mb)) as i32) == 1i32 {
            SetCurrentSecretBase();
            ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(Some(FieldCallback_PrepareFadeInFromMenu));
            ((&raw mut gPostMenuFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCallback_SecretBaseTree));
            return 1u8;
        }
        if ((MetatileBehavior_IsSecretBaseShrub(mb)) as i32) == 1i32 {
            SetCurrentSecretBase();
            ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(Some(FieldCallback_PrepareFadeInFromMenu));
            ((&raw mut gPostMenuFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCallback_SecretBaseShrub));
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_SecretBaseCave() {
    unsafe {
        (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .write(((GetCursorSelectionMonId()) as i32));
        ScriptContext_SetupScript(
            (&raw mut SecretBase_EventScript_CaveUseSecretPower).cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseSecretPowerCave() -> u8 {
    unsafe {
        let mut taskId: u8 = CreateFieldMoveTask();
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write((((StartSecretBaseCaveFieldEffect as *const () as usize as u32) >> 16) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .write(((StartSecretBaseCaveFieldEffect as *const () as usize as u32) as i16));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartSecretBaseCaveFieldEffect() {
    unsafe {
        FieldEffectActiveListRemove(11u8);
        FieldEffectStart(55u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_SecretPowerCave() -> u8 {
    unsafe {
        AdjustSecretPowerSpritePixelOffsets();
        CreateSprite(
            (&raw const sSpriteTemplate_SecretPowerCave)
                .cast::<u8>()
                .cast_mut(),
            (((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(2),
                0,
                9,
                false,
            ) as u32)
                .wrapping_add(
                    ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(5))
                    .read()) as u32),
                )) as i16),
            (((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(0),
                0,
                8,
                false,
            ) as u32)
                .wrapping_add(
                    ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(6))
                    .read()) as u32),
                )) as i16),
            148u8,
        );
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CaveEntranceInit(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        PlaySE(131u16);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_CaveEntranceOpen));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CaveEntranceOpen(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 40i32 {
            if (({
                let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 20i32
            {
                ToggleSecretBaseEntranceMetatile();
            }
        } else {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_CaveEntranceEnd));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CaveEntranceEnd(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        FieldEffectStop(sprite, 55u8);
        ScriptContext_Enable();
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_SecretBaseTree() {
    unsafe {
        (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .write(((GetCursorSelectionMonId()) as i32));
        ScriptContext_SetupScript(
            (&raw mut SecretBase_EventScript_TreeUseSecretPower).cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseSecretPowerTree() -> u8 {
    unsafe {
        let mut taskId: u8 = CreateFieldMoveTask();
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write((((StartSecretBaseTreeFieldEffect as *const () as usize as u32) >> 16) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .write(((StartSecretBaseTreeFieldEffect as *const () as usize as u32) as i16));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartSecretBaseTreeFieldEffect() {
    unsafe {
        FieldEffectActiveListRemove(26u8);
        FieldEffectStart(56u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_SecretPowerTree() -> u8 {
    unsafe {
        let mut mb: i16 = ((MapGridGetMetatileBehaviorAt(
            (((((&raw mut gPlayerFacingPosition).cast::<u8>()).cast::<i16>()).read()) as i32),
            (((((&raw mut gPlayerFacingPosition).cast::<u8>())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32),
        ) & 4095i32) as i16);
        if ((mb) as i32) == 150i32 {
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(7))
                .write(0i32);
        }
        if ((mb) as i32) == 156i32 {
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(7))
                .write(2i32);
        }
        AdjustSecretPowerSpritePixelOffsets();
        CreateSprite(
            (&raw const sSpriteTemplate_SecretPowerTree)
                .cast::<u8>()
                .cast_mut(),
            (((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(2),
                0,
                9,
                false,
            ) as u32)
                .wrapping_add(
                    ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(5))
                    .read()) as u32),
                )) as i16),
            (((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(0),
                0,
                8,
                false,
            ) as u32)
                .wrapping_add(
                    ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(6))
                    .read()) as u32),
                )) as i16),
            148u8,
        );
        if (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(7))
            .read()
            == 1i32)
            || (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                .wrapping_offset(7))
            .read()
                == 3i32)
        {
            ToggleSecretBaseEntranceMetatile();
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TreeEntranceInit(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        PlaySE(155u16);
        ((sprite).wrapping_add(42)).write(
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(7))
                .read()) as u8),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_TreeEntranceOpen));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TreeEntranceOpen(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >= 40i32 {
            if (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                .wrapping_offset(7))
            .read()
                == 0i32)
                || (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(7))
                .read()
                    == 2i32)
            {
                ToggleSecretBaseEntranceMetatile();
            }
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_TreeEntranceEnd));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TreeEntranceEnd(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        FieldEffectStop(sprite, 56u8);
        ScriptContext_Enable();
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_SecretBaseShrub() {
    unsafe {
        (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .write(((GetCursorSelectionMonId()) as i32));
        ScriptContext_SetupScript(
            (&raw mut SecretBase_EventScript_ShrubUseSecretPower).cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseSecretPowerShrub() -> u8 {
    unsafe {
        let mut taskId: u8 = CreateFieldMoveTask();
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write((((StartSecretBaseShrubFieldEffect as *const () as usize as u32) >> 16) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .write(((StartSecretBaseShrubFieldEffect as *const () as usize as u32) as i16));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartSecretBaseShrubFieldEffect() {
    unsafe {
        FieldEffectActiveListRemove(27u8);
        FieldEffectStart(57u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_SecretPowerShrub() -> u8 {
    unsafe {
        AdjustSecretPowerSpritePixelOffsets();
        CreateSprite(
            (&raw const sSpriteTemplate_SecretPowerShrub)
                .cast::<u8>()
                .cast_mut(),
            (((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(2),
                0,
                9,
                false,
            ) as u32)
                .wrapping_add(
                    ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(5))
                    .read()) as u32),
                )) as i16),
            (((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(0),
                0,
                8,
                false,
            ) as u32)
                .wrapping_add(
                    ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(6))
                    .read()) as u32),
                )) as i16),
            148u8,
        );
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ShrubEntranceInit(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        PlaySE(169u16);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_ShrubEntranceOpen));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ShrubEntranceOpen(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 40i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 20i32 {
                ToggleSecretBaseEntranceMetatile();
            }
        } else {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_ShrubEntranceEnd));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ShrubEntranceEnd(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        FieldEffectStop(sprite, 57u8);
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_SecretBasePCTurnOn() -> u8 {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut taskId: u8 = 0u8;
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        taskId = CreateTask(Some(Task_SecretBasePCTurnOn), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(x);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(y);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_SecretBasePCTurnOn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = ((((data).wrapping_offset(2)).read()) as i32);
            if __sw1 == 4i32 || __sw1 == 12i32 {
                MapGridSetMetatileIdAt(
                    (((data).read()) as i32),
                    ((((data).wrapping_offset(1)).read()) as i32),
                    548u16,
                );
                CurrentMapDrawMetatileAt(
                    (((data).read()) as i32),
                    ((((data).wrapping_offset(1)).read()) as i32),
                );
                break 'l1;
            }
            if __sw1 == 8i32 || __sw1 == 16i32 {
                MapGridSetMetatileIdAt(
                    (((data).read()) as i32),
                    ((((data).wrapping_offset(1)).read()) as i32),
                    544u16,
                );
                CurrentMapDrawMetatileAt(
                    (((data).read()) as i32),
                    ((((data).wrapping_offset(1)).read()) as i32),
                );
                break 'l1;
            }
            if __sw1 == 20i32 {
                MapGridSetMetatileIdAt(
                    (((data).read()) as i32),
                    ((((data).wrapping_offset(1)).read()) as i32),
                    548u16,
                );
                CurrentMapDrawMetatileAt(
                    (((data).read()) as i32),
                    ((((data).wrapping_offset(1)).read()) as i32),
                );
                FieldEffectActiveListRemove(61u8);
                ScriptContext_Enable();
                DestroyTask(taskId);
                return;
            }
        }
        let __p2 = (data).wrapping_offset(2);
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoSecretBasePCTurnOffEffect() {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        PlaySE(3u16);
        if !((VarGet(16468u16)) != 0) {
            MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 3616u16);
        } else {
            MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 3617u16);
        }
        CurrentMapDrawMetatileAt(((x) as i32), ((y) as i32));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PopSecretBaseBalloon(metatileId: i16, x: i16, y: i16) {
    unsafe {
        let mut metatileId = metatileId;
        let mut x = x;
        let mut y = y;
        let mut taskId: u8 = CreateTask(Some(Task_PopSecretBaseBalloon), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(metatileId);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(x);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(y);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(1i16);
    }
}
pub(crate) unsafe extern "C" fn Task_PopSecretBaseBalloon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((((data).wrapping_offset(3)).read()) as i32) == 6i32 {
            ((data).wrapping_offset(3)).write(0i16);
        } else {
            let __p1 = (data).wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if ((((data).wrapping_offset(3)).read()) as i32) == 0i32 {
            if ((((data).wrapping_offset(4)).read()) as i32) == 2i32 {
                DoBalloonSoundEffect((data).read());
            }
            MapGridSetMetatileIdAt(
                ((((data).wrapping_offset(1)).read()) as i32),
                ((((data).wrapping_offset(2)).read()) as i32),
                (((((data).read()) as i32)
                    .wrapping_add(((((data).wrapping_offset(4)).read()) as i32)))
                    as u16),
            );
            CurrentMapDrawMetatileAt(
                ((((data).wrapping_offset(1)).read()) as i32),
                ((((data).wrapping_offset(2)).read()) as i32),
            );
            if ((((data).wrapping_offset(4)).read()) as i32) == 3i32 {
                DestroyTask(taskId);
            } else {
                let __p2 = (data).wrapping_offset(4);
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DoBalloonSoundEffect(metatileId: i16) {
    unsafe {
        let mut metatileId = metatileId;
        'l1: {
            let __sw1 = ((metatileId) as i32);
            if __sw1 == 824i32 {
                PlaySE(74u16);
                break 'l1;
            }
            if __sw1 == 828i32 {
                PlaySE(75u16);
                break 'l1;
            }
            if __sw1 == 832i32 {
                PlaySE(76u16);
                break 'l1;
            }
            if __sw1 == 552i32 {
                PlaySE(78u16);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_Nop47() -> u8 {
    unsafe {
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_Nop48() -> u8 {
    unsafe {
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DoSecretBaseBreakableDoorEffect(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        PlaySE(77u16);
        MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 630u16);
        MapGridSetMetatileIdAt(((x) as i32), ((y) as i32).wrapping_sub(1i32), 622u16);
        CurrentMapDrawMetatileAt(((x) as i32), ((y) as i32));
        CurrentMapDrawMetatileAt(((x) as i32), ((y) as i32).wrapping_sub(1i32));
    }
}
pub(crate) unsafe extern "C" fn Task_ShatterSecretBaseBreakableDoor(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 7i32
        {
            DoSecretBaseBreakableDoorEffect(
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read(),
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read(),
            );
            DestroyTask(taskId);
        } else {
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShatterSecretBaseBreakableDoor(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut dir: u8 = GetPlayerFacingDirection();
        if ((dir) as i32) == 1i32 {
            DoSecretBaseBreakableDoorEffect(x, y);
        } else {
            if ((dir) as i32) == 2i32 {
                let mut taskId: u8 = CreateTask(Some(Task_ShatterSecretBaseBreakableDoor), 5u8);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(x);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(y);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SecretBaseMusicNoteMatSound(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == 7i32
        {
            'l1: {
                let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32);
                if __sw1 == 632i32 {
                    PlaySE(62u16);
                    break 'l1;
                }
                if __sw1 == 633i32 {
                    PlaySE(63u16);
                    break 'l1;
                }
                if __sw1 == 634i32 {
                    PlaySE(64u16);
                    break 'l1;
                }
                if __sw1 == 635i32 {
                    PlaySE(65u16);
                    break 'l1;
                }
                if __sw1 == 636i32 {
                    PlaySE(66u16);
                    break 'l1;
                }
                if __sw1 == 637i32 {
                    PlaySE(67u16);
                    break 'l1;
                }
                if __sw1 == 638i32 {
                    PlaySE(68u16);
                    break 'l1;
                }
                if __sw1 == 691i32 {
                    PlaySE(69u16);
                    break 'l1;
                }
            }
            DestroyTask(taskId);
        } else {
            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlaySecretBaseMusicNoteMatSound(metatileId: i16) {
    unsafe {
        let mut metatileId = metatileId;
        let mut taskId: u8 = CreateTask(Some(Task_SecretBaseMusicNoteMatSound), 5u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(metatileId);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_GlitterMatSparkle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 8i32 {
            PlaySE(195u16);
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >= 32i32 {
            DestroySprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoSecretBaseGlitterMatSparkle() {
    unsafe {
        let mut x: i16 = (((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        ))
        .wrapping_add(16))
        .cast::<i16>())
        .read();
        let mut y: i16 = (((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        ))
        .wrapping_add(16))
        .wrapping_add(2)
        .cast::<i16>())
        .read();
        let mut spriteId: u8 = 0u8;
        SetSpritePosToOffsetMapCoords(&raw mut x, &raw mut y, 8i16, 4i16);
        spriteId = CreateSpriteAtEnd(
            ((((&raw mut gFieldEffectObjectTemplatePointers).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(22))
            .read(),
            x,
            y,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                1,
                1,
                (1u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                (1u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                4,
                4,
                (5u16) as i32,
            );
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_GlitterMatSparkle));
            (((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .write(0i16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_SandPillar() -> u8 {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        LockPlayerFieldControls();
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(5))
            .write(((x) as i32));
        ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(6))
            .write(((y) as i32));
        'l1: {
            let __sw1 = ((GetPlayerFacingDirection()) as i32);
            if __sw1 == 1i32 {
                CreateSprite(
                    (&raw const sSpriteTemplate_SandPillar)
                        .cast::<u8>()
                        .cast_mut(),
                    (((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(2),
                        0,
                        9,
                        false,
                    ) as u32)
                        .wrapping_add(8u32)) as i16),
                    (((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(0),
                        0,
                        8,
                        false,
                    ) as u32)
                        .wrapping_add(32u32)) as i16),
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                CreateSprite(
                    (&raw const sSpriteTemplate_SandPillar)
                        .cast::<u8>()
                        .cast_mut(),
                    (((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(2),
                        0,
                        9,
                        false,
                    ) as u32)
                        .wrapping_add(8u32)) as i16),
                    ((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(0),
                        0,
                        8,
                        false,
                    ) as u32) as i16),
                    148u8,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                CreateSprite(
                    (&raw const sSpriteTemplate_SandPillar)
                        .cast::<u8>()
                        .cast_mut(),
                    (((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(2),
                        0,
                        9,
                        false,
                    ) as u32)
                        .wrapping_sub(8u32)) as i16),
                    (((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(0),
                        0,
                        8,
                        false,
                    ) as u32)
                        .wrapping_add(16u32)) as i16),
                    148u8,
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                CreateSprite(
                    (&raw const sSpriteTemplate_SandPillar)
                        .cast::<u8>()
                        .cast_mut(),
                    (((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(2),
                        0,
                        9,
                        false,
                    ) as u32)
                        .wrapping_add(24u32)) as i16),
                    (((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(0),
                        0,
                        8,
                        false,
                    ) as u32)
                        .wrapping_add(16u32)) as i16),
                    148u8,
                );
                break 'l1;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_SandPillar_BreakTop(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        PlaySE(131u16);
        if MapGridGetMetatileIdAt(
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(5))
                .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(6))
                .read())
            .wrapping_sub(1i32),
        ) == 646i32
        {
            MapGridSetMetatileIdAt(
                ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(5))
                .read(),
                (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(6))
                .read())
                .wrapping_sub(1i32),
                3586u16,
            );
        } else {
            MapGridSetMetatileIdAt(
                ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(5))
                .read(),
                (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(6))
                .read())
                .wrapping_sub(1i32),
                644u16,
            );
        }
        MapGridSetMetatileIdAt(
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(5))
                .read(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(6))
                .read(),
            522u16,
        );
        CurrentMapDrawMetatileAt(
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(5))
                .read(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(6))
                .read())
            .wrapping_sub(1i32),
        );
        CurrentMapDrawMetatileAt(
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(5))
                .read(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(6))
                .read(),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_SandPillar_BreakBase));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_SandPillar_BreakBase(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 18i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            MapGridSetMetatileIdAt(
                ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(5))
                .read(),
                ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(6))
                .read(),
                3724u16,
            );
            CurrentMapDrawMetatileAt(
                ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(5))
                .read(),
                ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                    .wrapping_offset(6))
                .read(),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_SandPillar_End));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_SandPillar_End(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        FieldEffectStop(sprite, 52u8);
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InteractWithShieldOrTVDecoration() {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut metatileId: i32 = 0i32;
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        metatileId = MapGridGetMetatileIdAt(((x) as i32), ((y) as i32));
        'l1: {
            let __sw1 = metatileId;
            if __sw1 == 822i32 {
                ConvertIntToDecimalStringN((&raw mut gStringVar1).cast::<u8>(), 100i32, 0i32, 3u8);
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (&raw mut gText_Gold).cast::<u8>(),
                );
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                if !((VarGet(16468u16)) != 0) {
                    return;
                }
                VarSet(16622u16, ((((VarGet(16622u16)) as i32) | 16i32) as u16));
                break 'l1;
            }
            if __sw1 == 734i32 {
                ConvertIntToDecimalStringN((&raw mut gStringVar1).cast::<u8>(), 50i32, 0i32, 2u8);
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (&raw mut gText_Silver).cast::<u8>(),
                );
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                if !((VarGet(16468u16)) != 0) {
                    return;
                }
                VarSet(16622u16, ((((VarGet(16622u16)) as i32) | 32i32) as u16));
                break 'l1;
            }
            if __sw1 == 756i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                if !((VarGet(16468u16)) != 0) {
                    return;
                }
                VarSet(16622u16, ((((VarGet(16622u16)) as i32) | 128i32) as u16));
                break 'l1;
            }
            if __sw1 == 757i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(2u16);
                if !((VarGet(16468u16)) != 0) {
                    return;
                }
                VarSet(16622u16, ((((VarGet(16622u16)) as i32) | 128i32) as u16));
                break 'l1;
            }
            if __sw1 == 758i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(3u16);
                if !((VarGet(16468u16)) != 0) {
                    return;
                }
                VarSet(16622u16, ((((VarGet(16622u16)) as i32) | 128i32) as u16));
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLargeBreakableDecoration(metatileId: u16, checkBase: u8) -> u8 {
    unsafe {
        let mut metatileId = metatileId;
        let mut checkBase = checkBase;
        if !((CurMapIsSecretBase()) != 0) {
            return 0u8;
        }
        if !((checkBase) != 0) {
            if (((metatileId) as i32) == 645i32) || (((metatileId) as i32) == 646i32) {
                return 1u8;
            }
            if ((metatileId) as i32) == 567i32 {
                return 1u8;
            }
        } else {
            if ((metatileId) as i32) == 653i32 {
                return 1u8;
            }
            if ((metatileId) as i32) == 575i32 {
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_FieldPoisonEffect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (data).wrapping_offset(1);
                (__p2).write((((((__p2).read()) as i32).wrapping_add(2i32)) as i16));
                if ((((data).wrapping_offset(1)).read()) as i32) > 8i32 {
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = (data).wrapping_offset(1);
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(2i32)) as i16));
                if ((((data).wrapping_offset(1)).read()) as i32) == 0i32 {
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                DestroyTask(taskId);
                return;
            }
        }
        SetGpuReg(
            76u8,
            (((((((data).wrapping_offset(1)).read()) as i32) << 4)
                | ((((data).wrapping_offset(1)).read()) as i32)) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEffPoison_Start() {
    unsafe {
        PlaySE(79u16);
        CreateTask(Some(Task_FieldPoisonEffect), 80u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEffPoison_IsActive() -> u32 {
    unsafe {
        return ((FuncIsActiveTask(Some(Task_FieldPoisonEffect))) as u32);
    }
}
pub(crate) unsafe extern "C" fn Task_WateringBerryTreeAnim(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_WateringBerryTreeAnim_Start));
    }
}
pub(crate) unsafe extern "C" fn Task_WateringBerryTreeAnim_Start(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (!((ObjectEventIsMovementOverridden(playerObjEvent)) != 0))
            || ((ObjectEventClearHeldMovementIfFinished(playerObjEvent)) != 0)
        {
            SetPlayerAvatarWatering(GetPlayerFacingDirection());
            ObjectEventSetHeldMovement(
                playerObjEvent,
                GetWalkInPlaceNormalMovementAction(((GetPlayerFacingDirection()) as u32)),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_WateringBerryTreeAnim_Continue));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WateringBerryTreeAnim_Continue(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (ObjectEventClearHeldMovementIfFinished(playerObjEvent)) != 0 {
            let mut value: i16 = {
                let __p1 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_add(1));
                __t2
            };
            if ((value) as i32) < 10i32 {
                ObjectEventSetHeldMovement(
                    playerObjEvent,
                    GetWalkInPlaceNormalMovementAction(((GetPlayerFacingDirection()) as u32)),
                );
            } else {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_WateringBerryTreeAnim_End));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WateringBerryTreeAnim_End(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetPlayerAvatarTransitionFlags(((GetPlayerAvatarFlags()) as u16));
        DestroyTask(taskId);
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoWateringBerryTreeAnim() {
    unsafe {
        CreateTask(Some(Task_WateringBerryTreeAnim), 80u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateRecordMixingLights() -> u8 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        LoadSpritePalette(
            (&raw const sSpritePalette_RecordMixLights)
                .cast::<u8>()
                .cast_mut(),
        );
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_RecordMixLights)
                .cast::<u8>()
                .cast_mut(),
            0i16,
            0i16,
            82u8,
        );
        if ((spriteId) as i32) == 64i32 {
            return 64u8;
        } else {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            GetMapCoordsFromSpritePos(
                16i16,
                13i16,
                (sprite).wrapping_add(32).cast::<i16>(),
                (sprite).wrapping_add(34).cast::<i16>(),
            );
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(16i32)) as i16));
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(2i32)) as i16));
        }
        return spriteId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyRecordMixingLights() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 64i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset((i) as isize * 68))
                        .wrapping_add(20)
                        .cast::<*mut u8>())
                    .read()) as usize)
                        == (((&raw const sSpriteTemplate_RecordMixLights)
                            .cast::<u8>()
                            .cast_mut()) as usize)
                    {
                        FreeSpritePalette(
                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset((i) as isize * 68),
                        );
                        DestroySprite(
                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset((i) as isize * 68),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
