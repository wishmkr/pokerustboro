//! Translated from `src/mirage_tower.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sMirageTower_Gfx sMirageTowerTilemap sFossil_Pal sFossil_Gfx sMirageTowerCrumbles_Gfx sMirageTowerCrumbles_Palette sCeilingCrumblePositions sCeilingCrumbleSpriteSheets sInvisibleMirageTowerMetatiles sAnim_FallingFossil sOamData_FallingFossil sAnims_FallingFossil sSpriteTemplate_FallingFossil gMirageTowerPulseBlendSettings sAnim_CeilingCrumbleSmall sAnims_CeilingCrumbleSmall sOamData_CeilingCrumbleSmall sSpriteTemplate_CeilingCrumbleSmall sAnim_CeilingCrumbleLarge sAnims_CeilingCrumbleLarge sOamData_CeilingCrumbleLarge sSpriteTemplate_CeilingCrumbleLarge
#[allow(unused_imports)]
use crate::data::mirage_tower::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMirageTowerGfxBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMirageTowerTilemapBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFallingFossil: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFallingTower: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBgShakeOffsets: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMirageTowerPulseBlend: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sDebug_DisintegrationData: crate::ffi::Align4<[u8; 16]> =
    crate::ffi::Align4([0; 16]);

unsafe extern "C" {
    static mut gObjectEvents: u8;
    static mut gPlayerAvatar: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DrawWholeMapView();
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllWindowBuffers();
    fn FreeSpriteTilesByTag(a0: u16);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn InitPulseBlend(a0: *mut u8);
    fn InitPulseBlendPaletteSettings(a0: *mut u8, a1: *mut u8) -> i32;
    fn InitStandardTextBoxWindows();
    fn InstallCameraPanAheadCallback();
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadSpriteSheets(a0: *mut u8);
    fn MapGridSetMetatileIdAt(a0: i32, a1: i32, a2: u16);
    fn MarkUsedPulseBlendPalettes(a0: *mut u8, a1: u16, a2: u8);
    fn PlaySE(a0: u16);
    fn Random() -> u16;
    fn ScriptContext_Enable();
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetCameraPanning(a0: i16, a1: i16);
    fn SetCameraPanningCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn TryGetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8;
    fn UnloadUsedPulseBlendPalettes(a0: *mut u8, a1: u16, a2: u8);
    fn UnmarkUsedPulseBlendPalettes(a0: *mut u8, a1: u16, a2: u8);
    fn UnsetBgTilemapBuffer(a0: u8);
    fn UpdatePulseBlend(a0: *mut u8);
    fn VarGet(a0: u16) -> u16;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMirageTowerVisible() -> u8 {
    unsafe {
        if !(((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            == 0i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 26i32))
        {
            return 0u8;
        }
        return FlagGet(334u16);
    }
}
pub(crate) unsafe extern "C" fn UpdateMirageTowerPulseBlend(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        UpdatePulseBlend(
            (((&raw mut sMirageTowerPulseBlend)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearMirageTowerPulseBlend() {
    unsafe {
        ((&raw mut sMirageTowerPulseBlend)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryStartMirageTowerPulseBlendEffect() {
    unsafe {
        if !(((&raw mut sMirageTowerPulseBlend)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .is_null()
        {
            ((&raw mut sMirageTowerPulseBlend)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
            return;
        }
        if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            != 0i32)
            || ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                != 26i32))
            || (!((FlagGet(334u16)) != 0))
        {
            return;
        }
        ((&raw mut sMirageTowerPulseBlend)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(AllocZeroed(200u32));
        InitPulseBlend(
            (((&raw mut sMirageTowerPulseBlend)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4),
        );
        InitPulseBlendPaletteSettings(
            (((&raw mut sMirageTowerPulseBlend)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4),
            (&raw const gMirageTowerPulseBlendSettings)
                .cast::<u8>()
                .cast_mut(),
        );
        MarkUsedPulseBlendPalettes(
            (((&raw mut sMirageTowerPulseBlend)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4),
            1u16,
            1u8,
        );
        (((&raw mut sMirageTowerPulseBlend)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .write(CreateTask(Some(UpdateMirageTowerPulseBlend), 255u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearMirageTowerPulseBlendEffect() {
    unsafe {
        if ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            != 0i32)
            || ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                != 26i32))
            || (!((FlagGet(334u16)) != 0)))
            || (((((&raw mut sMirageTowerPulseBlend)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read()) as usize)
                == 0usize)
        {
            return;
        }
        if (FuncIsActiveTask(Some(UpdateMirageTowerPulseBlend))) != 0 {
            DestroyTask(
                (((&raw mut sMirageTowerPulseBlend)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read())
                .read(),
            );
        }
        UnmarkUsedPulseBlendPalettes(
            (((&raw mut sMirageTowerPulseBlend)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4),
            1u16,
            1u8,
        );
        UnloadUsedPulseBlendPalettes(
            (((&raw mut sMirageTowerPulseBlend)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4),
            1u16,
            1u8,
        );
        {
            Free(
                ((&raw mut sMirageTowerPulseBlend)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
            );
            ((&raw mut sMirageTowerPulseBlend)
                .cast::<u8>()
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMirageTowerVisibility() {
    unsafe {
        let mut rand: u16 = 0u16;
        let mut visible: u8 = 0u8;
        if (VarGet(16587u16)) != 0 {
            FlagClear(334u16);
            return;
        }
        rand = Random();
        visible = ((((rand) as i32) & 1i32) as u8);
        if ((FlagGet(157u16)) as i32) == 1i32 {
            visible = 1u8;
        }
        if (visible) != 0 {
            FlagSet(334u16);
            TryStartMirageTowerPulseBlendEffect();
            return;
        }
        FlagClear(334u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartPlayerDescendMirageTower() {
    unsafe {
        CreateTask(Some(PlayerDescendMirageTower), 8u8);
    }
}
pub(crate) unsafe extern "C" fn PlayerDescendMirageTower(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut objectEventId: u8 = 0u8;
        let mut fallingPlayer: *mut u8 = core::ptr::null_mut();
        let mut player: *mut u8 = core::ptr::null_mut();
        TryGetObjectEventIdByLocalIdAndMap(
            45u8,
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
            &raw mut objectEventId,
        );
        fallingPlayer = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objectEventId) as i32) as isize * 36);
        let __p1 = (((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((fallingPlayer).wrapping_add(4)).read()) as i32) as isize * 68))
        .wrapping_add(38)
        .cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        player = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((fallingPlayer).wrapping_add(4)).read()) as i32) as isize * 68))
        .wrapping_add(34)
        .cast::<i16>())
        .read()) as i32)
            .wrapping_add(
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((fallingPlayer).wrapping_add(4)).read()) as i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32),
            )
            >= ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((player).wrapping_add(4)).read()) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((player).wrapping_add(4)).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .read()) as i32),
                )
        {
            DestroyTask(taskId);
            ScriptContext_Enable();
        }
    }
}
pub(crate) unsafe extern "C" fn StartScreenShake(
    yShakeOffset: u8,
    xShakeOffset: u8,
    numShakes: u8,
    shakeDelay: u8,
) {
    unsafe {
        let mut yShakeOffset = yShakeOffset;
        let mut xShakeOffset = xShakeOffset;
        let mut numShakes = numShakes;
        let mut shakeDelay = shakeDelay;
        let mut taskId: u8 = CreateTask(Some(DoScreenShake), 9u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((xShakeOffset) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((numShakes) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((shakeDelay) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((yShakeOffset) as i16));
        SetCameraPanningCallback(None);
        PlaySE(214u16);
    }
}
pub(crate) unsafe extern "C" fn DoScreenShake(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = core::ptr::null_mut();
        data = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let __p1 = (data).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if crate::c::rem_i32(
            ((((data).wrapping_offset(1)).read()) as i32),
            ((((data).wrapping_offset(3)).read()) as i32),
        ) == 0i32
        {
            ((data).wrapping_offset(1)).write(0i16);
            let __p2 = (data).wrapping_offset(2);
            (__p2).write(((__p2).read()).wrapping_sub(1));
            (data).write((((((data).read()) as i32).wrapping_neg()) as i16));
            ((data).wrapping_offset(4))
                .write(((((((data).wrapping_offset(4)).read()) as i32).wrapping_neg()) as i16));
            SetCameraPanning((data).read(), ((data).wrapping_offset(4)).read());
            if ((((data).wrapping_offset(2)).read()) as i32) == 0i32 {
                IncrementCeilingCrumbleFinishedCount();
                DestroyTask(taskId);
                InstallCameraPanAheadCallback();
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IncrementCeilingCrumbleFinishedCount() {
    unsafe {
        let mut taskId: u8 = FindTaskIdByFunc(Some(WaitCeilingCrumble));
        if ((taskId) as i32) != 255i32 {
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoMirageTowerCeilingCrumble() {
    unsafe {
        LoadSpriteSheets(
            ((&raw const sCeilingCrumbleSpriteSheets)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        CreateCeilingCrumbleSprites();
        CreateTask(Some(WaitCeilingCrumble), 8u8);
        StartScreenShake(2u8, 1u8, 16u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn WaitCeilingCrumble(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut u16 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u16>();
        let __p1 = (data).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((data).wrapping_offset(1)).read()) as i32) == 1000i32)
            || ((((data).read()) as i32) == 17i32)
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(FinishCeilingCrumbleTask));
        }
    }
}
pub(crate) unsafe extern "C" fn FinishCeilingCrumbleTask(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        FreeSpriteTilesByTag(4000u16);
        DestroyTask(taskId);
        ScriptContext_Enable();
    }
}
pub(crate) unsafe extern "C" fn CreateCeilingCrumbleSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(48u32, 6u32)) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (&raw const sSpriteTemplate_CeilingCrumbleLarge)
                            .cast::<u8>()
                            .cast_mut(),
                        (((((((((&raw const sCeilingCrumblePositions)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 6))
                        .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(120i32)) as i16),
                        ((((((&raw const sCeilingCrumblePositions)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 6))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read(),
                        8u8,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(5),
                        2,
                        2,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(5),
                        4,
                        4,
                        (0u16) as i32,
                    );
                    (((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(((i) as i16));
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(48u32, 6u32)) {
                    break 'l3;
                }
                'l4: {
                    spriteId = CreateSprite(
                        (&raw const sSpriteTemplate_CeilingCrumbleSmall)
                            .cast::<u8>()
                            .cast_mut(),
                        (((((((((&raw const sCeilingCrumblePositions)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 6))
                        .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(115i32)) as i16),
                        ((((((((((&raw const sCeilingCrumblePositions)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 6))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_sub(3i32)) as i16),
                        8u8,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(5),
                        2,
                        2,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(5),
                        4,
                        4,
                        (0u16) as i32,
                    );
                    (((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(((i) as i16));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CeilingCrumble(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
                2i32,
            )) as i16),
        );
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
            > ((((((((&raw const sCeilingCrumblePositions)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 6,
            ))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
        {
            DestroySprite(sprite);
            IncrementCeilingCrumbleFinishedCount();
        }
    }
}
pub(crate) unsafe extern "C" fn SetInvisibleMirageTowerMetatiles() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(72u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    MapGridSetMetatileIdAt(
                        ((((((&raw const sInvisibleMirageTowerMetatiles)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .read()) as i32)
                            .wrapping_add(7i32),
                        (((((((&raw const sInvisibleMirageTowerMetatiles)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(1))
                        .read()) as i32)
                            .wrapping_add(7i32),
                        (((((&raw const sInvisibleMirageTowerMetatiles)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        DrawWholeMapView();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartMirageTowerDisintegration() {
    unsafe {
        CreateTask(Some(DoMirageTowerDisintegration), 9u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartMirageTowerShake() {
    unsafe {
        CreateTask(Some(InitMirageTowerShake), 9u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartMirageTowerFossilFallAndSink() {
    unsafe {
        CreateTask(Some(Task_FossilFallAndSink), 9u8);
    }
}
pub(crate) unsafe extern "C" fn SetBgShakeOffsets() {
    unsafe {
        SetGpuReg(
            16u8,
            ((((&raw mut sBgShakeOffsets).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                .read(),
        );
        SetGpuReg(
            18u8,
            ((((&raw mut sBgShakeOffsets).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateBgShake(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !(((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read())
            != 0)
        {
            ((((&raw mut sBgShakeOffsets).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                .write(
                    ((((((((&raw mut sBgShakeOffsets).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_neg()) as u16),
                );
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(2i16);
            SetBgShakeOffsets();
        } else {
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn InitMirageTowerShake(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut zero: u8 = 0u8;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                FreeAllWindowBuffers();
                SetBgAttribute(0u8, 7u8, 2u8);
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut sMirageTowerGfxBuffer)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(AllocZeroed(2336u32));
                ((&raw mut sMirageTowerTilemapBuffer)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(AllocZeroed(2048u32));
                ChangeBgX(0u8, 0i32, 0u8);
                ChangeBgY(0u8, 0i32, 0u8);
                let __p3 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                'l2: loop {
                    'l3: {
                        CpuSet(
                            ((&raw const sMirageTower_Gfx).cast::<u8>().cast_mut()).cast::<u8>(),
                            ((&raw mut sMirageTowerGfxBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read(),
                            crate::c::div_u32(2336u32, 2u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                LoadBgTiles(
                    0u8,
                    ((&raw mut sMirageTowerGfxBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                    2336u16,
                    0u16,
                );
                let __p4 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetBgTilemapBuffer(
                    0u8,
                    ((&raw mut sMirageTowerTilemapBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                CopyToBgTilemapBufferRect_ChangePalette(
                    0u8,
                    (((&raw const sMirageTowerTilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    12u8,
                    29u8,
                    6u8,
                    12u8,
                    17u8,
                );
                CopyBgTilemapBufferToVram(0u8);
                let __p5 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                ShowBg(0u8);
                let __p6 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                SetInvisibleMirageTowerMetatiles();
                let __p7 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                ((&raw mut sBgShakeOffsets).cast::<u8>().cast::<*mut u8>()).write(Alloc(4u32));
                zero = 0u8;
                ((((&raw mut sBgShakeOffsets).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>())
                .write(2u16);
                ((((&raw mut sBgShakeOffsets).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<u16>())
                .write(((zero) as u16));
                CreateTask(Some(UpdateBgShake), 10u8);
                DestroyTask(taskId);
                ScriptContext_Enable();
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DoMirageTowerDisintegration(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut bgShakeTaskId: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut i: u16 = 0u16;
        let mut index: u8 = 0u8;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 1i32 {
                ((&raw mut sFallingTower).cast::<u8>().cast::<*mut u8>())
                    .write(AllocZeroed(768u32));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    <= 95i32
                {
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        > 1i32
                    {
                        index = ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .read()) as u8);
                        (((((&raw mut sFallingTower).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((index) as i32) as isize * 8))
                        .cast::<*mut u8>())
                        .write(Alloc(48u32));
                        {
                            i = 0u16;
                            'l2: loop {
                                if !(((i) as i32) <= 47i32) {
                                    break 'l2;
                                }
                                'l3: {
                                    (((((((&raw mut sFallingTower)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((index) as i32) as isize * 8))
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .write(((i) as u8));
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        {
                            i = 0u16;
                            'l4: loop {
                                if !(((i) as i32) <= 47i32) {
                                    break 'l4;
                                }
                                'l5: {
                                    let mut rand1: u16 = 0u16;
                                    let mut rand2: u16 = 0u16;
                                    let mut temp: u16 = 0u16;
                                    rand1 =
                                        ((crate::c::rem_i32(((Random()) as i32), 48i32)) as u16);
                                    rand2 =
                                        ((crate::c::rem_i32(((Random()) as i32), 48i32)) as u16);
                                    {
                                        temp = (((((((((&raw mut sFallingTower)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((index) as i32) as isize * 8))
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((rand2) as i32) as isize))
                                        .read())
                                            as u16);
                                        (((((((&raw mut sFallingTower)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((index) as i32) as isize * 8))
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((rand2) as i32) as isize))
                                        .write(
                                            (((((((&raw mut sFallingTower)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_offset(((index) as i32) as isize * 8))
                                            .cast::<*mut u8>())
                                            .read())
                                            .wrapping_offset(((rand1) as i32) as isize))
                                            .read(),
                                        );
                                        (((((((&raw mut sFallingTower)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((index) as i32) as isize * 8))
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((rand1) as i32) as isize))
                                        .write(((temp) as u8));
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        if ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .read()) as i32)
                            <= 95i32
                        {
                            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(3);
                            (__p2).write(((__p2).read()).wrapping_add(1));
                        }
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(0i16);
                    }
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                index = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as u8);
                {
                    i = (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as u8) as u16);
                    'l6: loop {
                        if !(((i) as i32) < ((index) as i32)) {
                            break 'l6;
                        }
                        'l7: {
                            {
                                j = 0u8;
                                'l8: loop {
                                    if !(((j) as i32) < 1i32) {
                                        break 'l8;
                                    }
                                    'l9: {
                                        UpdateDisintegrationEffect(
                                            ((&raw mut sMirageTowerGfxBuffer)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read(),
                                            (((((95i32).wrapping_sub(((i) as i32)))
                                                .wrapping_mul(48i32))
                                            .wrapping_add(
                                                (((((((((&raw mut sFallingTower)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_offset(((i) as i32) as isize * 8))
                                                .cast::<*mut u8>())
                                                .read())
                                                .wrapping_offset(
                                                    (({
                                                        let __p4 = ((((&raw mut sFallingTower)
                                                            .cast::<u8>()
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_offset(
                                                            ((i) as i32) as isize * 8,
                                                        ))
                                                        .wrapping_add(4);
                                                        let __t5 = (__p4).read();
                                                        (__p4)
                                                            .write(((__p4).read()).wrapping_add(1));
                                                        __t5
                                                    })
                                                        as i32)
                                                        as isize,
                                                ))
                                                .read())
                                                    as i32),
                                            )) as u16),
                                            0u8,
                                            48u8,
                                            1u8,
                                        );
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                            if (((((((&raw mut sFallingTower).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(((i) as i32) as isize * 8))
                            .wrapping_add(4))
                            .read()) as i32)
                                > 47i32
                            {
                                {
                                    Free(
                                        (((((&raw mut sFallingTower)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((i) as i32) as isize * 8))
                                        .cast::<*mut u8>())
                                        .read(),
                                    );
                                    (((((&raw mut sFallingTower).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_offset(((i) as i32) as isize * 8))
                                    .cast::<*mut u8>())
                                    .write(core::ptr::null_mut());
                                }
                                let __p6 = (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2);
                                (__p6).write(((__p6).read()).wrapping_add(1));
                                if crate::c::rem_i32(((i) as i32), 2i32) == 1i32 {
                                    let __p7 = (((&raw mut sBgShakeOffsets)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(2)
                                    .cast::<u16>();
                                    (__p7).write(((__p7).read()).wrapping_sub(1));
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                LoadBgTiles(
                    0u8,
                    ((&raw mut sMirageTowerGfxBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                    2336u16,
                    0u16,
                );
                if (((((((&raw mut sFallingTower).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(760))
                .wrapping_add(4))
                .read()) as i32)
                    > 47i32
                {
                    break 'l1;
                }
                return;
            }
            if __sw1 == 4i32 {
                UnsetBgTilemapBuffer(0u8);
                bgShakeTaskId = FindTaskIdByFunc(Some(UpdateBgShake));
                if ((bgShakeTaskId) as i32) != 255i32 {
                    DestroyTask(bgShakeTaskId);
                }
                ((((&raw mut sBgShakeOffsets).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<u16>())
                .write({
                    let __v8 = 0u16;
                    ((((&raw mut sBgShakeOffsets).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .write(__v8);
                    __v8
                });
                SetBgShakeOffsets();
                break 'l1;
            }
            if __sw1 == 5i32 {
                {
                    Free(((&raw mut sBgShakeOffsets).cast::<u8>().cast::<*mut u8>()).read());
                    ((&raw mut sBgShakeOffsets).cast::<u8>().cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                }
                {
                    Free(((&raw mut sFallingTower).cast::<u8>().cast::<*mut u8>()).read());
                    ((&raw mut sFallingTower).cast::<u8>().cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                }
                {
                    Free(
                        ((&raw mut sMirageTowerGfxBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read(),
                    );
                    ((&raw mut sMirageTowerGfxBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .write(core::ptr::null_mut());
                }
                {
                    Free(
                        ((&raw mut sMirageTowerTilemapBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read(),
                    );
                    ((&raw mut sMirageTowerTilemapBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .write(core::ptr::null_mut());
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                SetGpuRegBits(12u8, 2u16);
                SetGpuRegBits(8u8, 0u16);
                SetBgAttribute(0u8, 7u8, 0u8);
                InitStandardTextBoxWindows();
                break 'l1;
            }
            if __sw1 == 7i32 {
                ShowBg(0u8);
                break 'l1;
            }
            if __sw1 == 8i32 {
                DestroyTask(taskId);
                ScriptContext_Enable();
                break 'l1;
            }
        }
        let __p9 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p9).write(((__p9).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Task_FossilFallAndSink(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        let mut buffer: *mut u8 = core::ptr::null_mut();
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 1i32 {
                __fall = true;
                ((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>())
                    .write(AllocZeroed(20u32));
                ((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .write(AllocZeroed(128u32));
                ((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .write(AllocZeroed(8u32));
                ((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<*mut u16>())
                .write((AllocZeroed(512u32)).cast::<u16>());
                ((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16)
                    .cast::<u16>())
                .write(0u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                buffer = ((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read();
                {
                    i = 0u16;
                    'l2: loop {
                        if !(((i) as u32) < 128u32) {
                            break 'l2;
                        }
                        'l3: {
                            (buffer).write(
                                ((((&raw const sFossil_Gfx).cast::<u8>().cast_mut()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                        buffer = (buffer).wrapping_offset(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                ((((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .cast::<*mut u8>())
                .write(
                    ((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read(),
                );
                ((((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4)
                .cast::<u16>())
                .write(128u16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                {
                    let mut fossilTemplate = crate::ffi::Align4([0u8; 24]);
                    (&raw mut fossilTemplate)
                        .cast::<u8>()
                        .cast::<crate::c::Rec4<24>>()
                        .write_unaligned(
                            (&raw const sSpriteTemplate_FallingFossil)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<crate::c::Rec4<24>>()
                                .read_unaligned(),
                        );
                    (((&raw mut fossilTemplate).cast::<u8>())
                        .wrapping_add(12)
                        .cast::<*mut u8>())
                    .write(
                        ((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read(),
                    );
                    ((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8))
                    .write(CreateSprite(
                        (&raw mut fossilTemplate).cast::<u8>(),
                        128i16,
                        (-16i16),
                        1u8,
                    ));
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(40)
                    .cast::<i8>())
                    .write(0i8);
                    (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(8))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(32)
                        .cast::<i16>())
                        .read(),
                    );
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(1i16);
                }
            }
            if __fall || __sw1 == 5i32 {
                __fall = true;
                {
                    i = 0u16;
                    'l4: loop {
                        if !(((i) as i32) < 256i32) {
                            break 'l4;
                        }
                        'l5: {
                            ((((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(12)
                            .cast::<*mut u16>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(i);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                {
                    i = 0u16;
                    'l6: loop {
                        if !(((i) as u32) < 512u32) {
                            break 'l6;
                        }
                        'l7: {
                            let mut rand1: u16 = 0u16;
                            let mut rand2: u16 = 0u16;
                            let mut temp: u16 = 0u16;
                            rand1 = ((crate::c::rem_i32(((Random()) as i32), 256i32)) as u16);
                            rand2 = ((crate::c::rem_i32(((Random()) as i32), 256i32)) as u16);
                            {
                                temp = ((((((&raw mut sFallingFossil)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(12)
                                .cast::<*mut u16>())
                                .read())
                                .wrapping_offset(((rand2) as i32) as isize))
                                .read();
                                ((((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(12)
                                .cast::<*mut u16>())
                                .read())
                                .wrapping_offset(((rand2) as i32) as isize))
                                .write(
                                    ((((((&raw mut sFallingFossil)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(12)
                                    .cast::<*mut u16>())
                                    .read())
                                    .wrapping_offset(((rand1) as i32) as isize))
                                    .read(),
                                );
                                ((((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(12)
                                .cast::<*mut u16>())
                                .read())
                                .wrapping_offset(((rand1) as i32) as isize))
                                .write(temp);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_FallingFossil));
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .read(),
                ) != (SpriteCallbackDummy as *const () as usize)
                {
                    return;
                }
                DestroySprite(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .read()) as i32) as isize
                            * 68,
                    ),
                );
                {
                    Free(
                        (((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12)
                            .cast::<*mut u16>())
                        .read())
                        .cast::<u8>(),
                    );
                    ((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<*mut u16>())
                    .write(core::ptr::null_mut());
                }
                {
                    Free(
                        ((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read(),
                    );
                    ((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .write(core::ptr::null_mut());
                }
                {
                    Free(
                        ((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read(),
                    );
                    ((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .write(core::ptr::null_mut());
                }
                {
                    Free(((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read());
                    ((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                __fall = true;
                ScriptContext_Enable();
                break 'l1;
            }
        }
        let __p2 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FallingFossil(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<u16>())
        .read()) as i32)
            >= 256i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        } else {
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) >= 96i32 {
                let mut i: u8 = 0u8;
                {
                    i = 0u8;
                    'l1: loop {
                        if !(((i) as i32) < 2i32) {
                            break 'l1;
                        }
                        'l2: {
                            UpdateDisintegrationEffect(
                                ((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<*mut u8>())
                                .read(),
                                ((((((&raw mut sFallingFossil).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(12)
                                .cast::<*mut u16>())
                                .read())
                                .wrapping_offset(
                                    (({
                                        let __p1 = (((&raw mut sFallingFossil)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(16)
                                        .cast::<u16>();
                                        let __t2 = (__p1).read();
                                        (__p1).write(((__p1).read()).wrapping_add(1));
                                        __t2
                                    }) as i32) as isize,
                                ))
                                .read(),
                                0u8,
                                16u8,
                                0u8,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                StartSpriteAnim(sprite, 0u8);
            } else {
                let __p3 = (sprite).wrapping_add(34).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateDisintegrationEffect(
    tiles: *mut u8,
    randId: u16,
    c: u8,
    size: u8,
    offset: u8,
) {
    unsafe {
        let mut tiles = tiles;
        let mut randId = randId;
        let mut c = c;
        let mut size = size;
        let mut offset = offset;
        let mut heightTiles: u8 = 0u8;
        let mut height: u8 = 0u8;
        let mut widthTiles: u8 = 0u8;
        let mut width: u8 = 0u8;
        let mut var: u16 = 0u16;
        let mut baseOffset: u16 = 0u16;
        let mut col: u8 = 0u8;
        let mut row: u8 = 0u8;
        let mut flag: u8 = 0u8;
        let mut tileMask: u8 = 0u8;
        height = ((crate::c::div_i32(((randId) as i32), ((size) as i32))) as u8);
        (((&raw mut sDebug_DisintegrationData)
            .cast::<u8>()
            .cast::<u16>())
        .cast::<u16>())
        .write(((height) as u16));
        width = ((crate::c::rem_i32(((randId) as i32), ((size) as i32))) as u8);
        ((((&raw mut sDebug_DisintegrationData)
            .cast::<u8>()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(1))
        .write(((width) as u16));
        row = ((((height) as i32) & 7i32) as u8);
        col = ((((width) as i32) & 7i32) as u8);
        ((((&raw mut sDebug_DisintegrationData)
            .cast::<u8>()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(2))
        .write(((((height) as i32) & 7i32) as u16));
        ((((&raw mut sDebug_DisintegrationData)
            .cast::<u8>()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(3))
        .write(((((width) as i32) & 7i32) as u16));
        widthTiles = ((crate::c::div_i32(((width) as i32), 8i32)) as u8);
        heightTiles = ((crate::c::div_i32(((height) as i32), 8i32)) as u8);
        ((((&raw mut sDebug_DisintegrationData)
            .cast::<u8>()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(4))
        .write(((crate::c::div_i32(((width) as i32), 8i32)) as u16));
        ((((&raw mut sDebug_DisintegrationData)
            .cast::<u8>()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(5))
        .write(((crate::c::div_i32(((height) as i32), 8i32)) as u16));
        var = ((((crate::c::div_i32(((size) as i32), 8i32))
            .wrapping_mul(((heightTiles) as i32).wrapping_mul(64i32)))
        .wrapping_add(((widthTiles) as i32).wrapping_mul(64i32))) as u16);
        ((((&raw mut sDebug_DisintegrationData)
            .cast::<u8>()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(6))
        .write(var);
        baseOffset = ((((var) as i32)
            .wrapping_add((((row) as i32).wrapping_mul(8i32)).wrapping_add(((col) as i32))))
            as u16);
        baseOffset = ((crate::c::div_i32(((baseOffset) as i32), 2i32)) as u16);
        ((((&raw mut sDebug_DisintegrationData)
            .cast::<u8>()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(7))
        .write(
            ((((var) as i32)
                .wrapping_add((((row) as i32).wrapping_mul(8i32)).wrapping_add(((col) as i32))))
                as u16),
        );
        flag = ((crate::c::rem_i32(((randId) as i32), 2i32) ^ 1i32) as u8);
        tileMask = ((crate::c::shl_i32(((c) as i32), ((((flag) as i32) << 2) as u32))
            | crate::c::shl_i32(15i32, (((((flag) as i32) ^ 1i32) << 2) as u32)))
            as u8);
        let __p1 = (tiles).wrapping_offset(
            (((baseOffset) as i32).wrapping_add(((offset) as i32).wrapping_mul(32i32))) as isize,
        );
        (__p1).write((((((__p1).read()) as i32) & ((tileMask) as i32)) as u8));
    }
}
