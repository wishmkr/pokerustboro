//! Translated from `src/battle_transition.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sBigPokeball_Tileset sPokeballTrail_Tileset sPokeball_Gfx sEliteFour_Tileset sUnusedBrendan_Gfx sUnusedLass_Gfx sShrinkingBoxTileset sEvilTeam_Palette sTeamAqua_Tileset sTeamAqua_Tilemap sTeamMagma_Tileset sTeamMagma_Tilemap sRegis_Tileset sRegice_Palette sRegisteel_Palette sRegirock_Palette sRegice_Tilemap sRegisteel_Tilemap sRegirock_Tilemap sUnused_Palette sKyogre_Tileset sKyogre_Tilemap sGroudon_Tileset sGroudon_Tilemap sKyogre1_Palette sKyogre2_Palette sGroudon1_Palette sGroudon2_Palette sRayquaza_Palette sRayquaza_Tileset sRayquaza_Tilemap sFrontierLogo_Palette sFrontierLogo_Tileset sFrontierLogo_Tilemap sFrontierSquares_Palette sFrontierSquares_FilledBg_Tileset sFrontierSquares_EmptyBg_Tileset sFrontierSquares_Shrink1_Tileset sFrontierSquares_Shrink2_Tileset sFrontierSquares_Tilemap sTasks_Intro sTasks_Main sTaskHandlers sBlur_Funcs sSwirl_Funcs sShuffle_Funcs sAqua_Funcs sMagma_Funcs sBigPokeball_Funcs sRegice_Funcs sRegisteel_Funcs sRegirock_Funcs sKyogre_Funcs sPokeballsTrail_Funcs sPokeballsTrail_StartXCoords sPokeballsTrail_Delays sPokeballsTrail_Speeds sClockwiseWipe_Funcs sRipple_Funcs sWave_Funcs sMugshot_Funcs sMugshotsTrainerPicIDsTable sMugshotsOpponentRotationScales sMugshotsOpponentCoords sMugshotTrainerPicFuncs sTrainerPicSlideSpeeds sTrainerPicSlideAccels sSlice_Funcs sShredSplit_Funcs sShredSplit_SectionYCoords sShredSplit_SectionMoveDirs sBlackhole_Funcs sBlackholePulsate_Funcs sBlackhole_Vibrations sRectangularSpiral_Funcs sRectangularSpiral_Major_InwardRight sRectangularSpiral_Major_InwardLeft sRectangularSpiral_Major_InwardUp sRectangularSpiral_Major_InwardDown sRectangularSpiral_Minor_InwardRight sRectangularSpiral_Minor_InwardLeft sRectangularSpiral_Minor_InwardUp sRectangularSpiral_Minor_InwardDown sRectangularSpiral_Minor_OutwardRight sRectangularSpiral_Minor_OutwardLeft sRectangularSpiral_Minor_OutwardUp sRectangularSpiral_Minor_OutwardDown sRectangularSpiral_Major_OutwardRight sRectangularSpiral_Major_OutwardLeft sRectangularSpiral_Major_OutwardUp sRectangularSpiral_Major_OutwardDown sRectangularSpiral_MoveDataTable_MajorDiagonal sRectangularSpiral_MoveDataTable_MinorDiagonal sRectangularSpiral_MoveDataTables sGroudon_Funcs sRayquaza_Funcs sWhiteBarsFade_Funcs sWhiteBarsFade_StartDelays sGridSquares_Funcs sAngledWipes_Funcs sAngledWipes_MoveData sAngledWipes_EndDelays sTransitionIntroFuncs sSpriteImage_Pokeball sSpriteAnim_Pokeball sSpriteAnimTable_Pokeball sSpriteAffineAnim_Pokeball1 sSpriteAffineAnim_Pokeball2 sSpriteAffineAnimTable_Pokeball sSpriteTemplate_Pokeball sOam_UnusedBrendanLass sImageTable_UnusedBrendan sImageTable_UnusedLass sSpriteAnim_UnusedBrendanLass sSpriteAnimTable_UnusedBrendanLass sSpriteTemplate_UnusedBrendan sSpriteTemplate_UnusedLass sFieldEffectPal_Pokeball gSpritePalette_Pokeball sMugshotPal_Sidney sMugshotPal_Phoebe sMugshotPal_Glacia sMugshotPal_Drake sMugshotPal_Champion sMugshotPal_Brendan sMugshotPal_May sOpponentMugshotsPals sPlayerMugshotsPals sUnusedTrainerPalette sSpritePalette_UnusedTrainer sBigPokeball_Tilemap sMugshotsTilemap sFrontierLogoWiggle_Funcs sFrontierLogoWave_Funcs sFrontierSquares_Funcs sFrontierSquaresSpiral_Funcs sFrontierSquaresScroll_Funcs sFrontierSquaresSpiral_Positions sFrontierSquaresScroll_Positions
#[allow(unused_imports)]
use crate::data::battle_transition::*;

pub(crate) static mut sDebug_RectangularSpiralData: i16 = 0i16;
pub(crate) static mut sTestingTransitionId: u8 = 0u8;
pub(crate) static mut sTestingTransitionState: u8 = 0u8;
pub(crate) static mut sRectangularSpiralLines: crate::ffi::Align4<[u8; 48]> =
    crate::ffi::Align4([0; 48]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTransitionData: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gBattle_BG0_X: u8;
    static mut gBattle_BG0_Y: u8;
    static mut gDecompressionBuffer: u8;
    static mut gFieldEffectArguments: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn AllocOamMatrix() -> u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_OverworldBasic();
    fn CB2_ReturnToField();
    fn CalcCenterToCornerVec(a0: *mut u8, a1: u8, a2: u8, a3: u8);
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyRectToBgTilemapBufferRect(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: u8,
        a8: u8,
        a9: u8,
        a10: u8,
        a11: i16,
        a12: i16,
    );
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateInvisibleSprite(a0: Option<unsafe extern "C" fn(*mut u8)>) -> u8;
    fn CreateSpriteAtEnd(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTrainerSprite(a0: u8, a1: i16, a2: i16, a3: u8, a4: *mut u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn EnableInterrupts(a0: u16);
    fn FieldEffectActiveListContains(a0: u8) -> u8;
    fn FieldEffectStart(a0: u8) -> u32;
    fn FieldEffectStop(a0: *mut u8, a1: u8);
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Free(a0: *mut u8);
    fn GetCameraOffsetWithPan(a0: *mut i16, a1: *mut i16);
    fn InitSpriteAffineAnim(a0: *mut u8);
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn PlaySE(a0: u16);
    fn PlayerGenderToFrontTrainerPicId(a0: u8) -> u16;
    fn ProcessSpriteCopyRequests();
    fn Random() -> u16;
    fn RunTasks();
    fn ScanlineEffect_Clear();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetOamMatrixRotationScaling(a0: u8, a1: i16, a2: i16, a3: u16);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWeatherScreenFadeOut();
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

pub(crate) unsafe extern "C" fn CB2_TestBattleTransition() {
    unsafe {
        'l1: {
            let __sw1 =
                ((((&raw mut sTestingTransitionState).cast::<u8>().cast::<u8>()).read()) as i32);
            if __sw1 == 0i32 {
                LaunchBattleTransitionTask(
                    ((&raw mut sTestingTransitionId).cast::<u8>().cast::<u8>()).read(),
                );
                let __p2 = (&raw mut sTestingTransitionState).cast::<u8>().cast::<u8>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsBattleTransitionDone()) != 0 {
                    ((&raw mut sTestingTransitionState).cast::<u8>().cast::<u8>()).write(0u8);
                    SetMainCallback2(Some(CB2_ReturnToField));
                }
                break 'l1;
            }
        }
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn TestBattleTransition(transitionId: u8) {
    unsafe {
        let mut transitionId = transitionId;
        ((&raw mut sTestingTransitionId).cast::<u8>().cast::<u8>()).write(transitionId);
        SetMainCallback2(Some(CB2_TestBattleTransition));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleTransition_StartOnField(transitionId: u8) {
    unsafe {
        let mut transitionId = transitionId;
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_OverworldBasic));
        LaunchBattleTransitionTask(transitionId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleTransition_Start(transitionId: u8) {
    unsafe {
        let mut transitionId = transitionId;
        LaunchBattleTransitionTask(transitionId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBattleTransitionDone() -> u8 {
    unsafe {
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_BattleTransition));
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .read())
            != 0
        {
            DestroyTask(taskId);
            {
                Free(((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
                ((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>())
                    .write(core::ptr::null_mut());
            }
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn LaunchBattleTransitionTask(transitionId: u8) {
    unsafe {
        let mut transitionId = transitionId;
        let mut taskId: u8 = CreateTask(Some(Task_BattleTransition), 2u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((transitionId) as i16));
        ((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(60u32));
    }
}
pub(crate) unsafe extern "C" fn Task_BattleTransition(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sTaskHandlers)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Transition_StartIntro(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        SetWeatherScreenFadeOut();
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .cast::<u8>(),
                            (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .cast::<u8>(),
                            ((67108864i32
                                | (crate::c::div_i32(1024i32, crate::c::div_i32(32i32, 8i32))
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
        if core::mem::transmute::<_, usize>(
            ((((&raw const sTasks_Intro)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .cast::<Option<unsafe extern "C" fn(u8)>>())
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize,
            ))
            .read(),
        ) != 0usize
        {
            CreateTask(
                ((((&raw const sTasks_Intro)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                        as isize,
                ))
                .read(),
                4u8,
            );
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            return 0u8;
        } else {
            (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Transition_WaitForIntro(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((FindTaskIdByFunc(
            ((((&raw const sTasks_Intro)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .cast::<Option<unsafe extern "C" fn(u8)>>())
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize,
            ))
            .read(),
        )) as i32)
            == 255i32
        {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Transition_StartMain(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        CreateTask(
            ((((&raw const sTasks_Main)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .cast::<Option<unsafe extern "C" fn(u8)>>())
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize,
            ))
            .read(),
            0u8,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Transition_WaitForMain(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(0i16);
        if ((FindTaskIdByFunc(
            ((((&raw const sTasks_Main)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .cast::<Option<unsafe extern "C" fn(u8)>>())
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize,
            ))
            .read(),
        )) as i32)
            == 255i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(1i16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_Intro(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            CreateIntroTask(0i16, 0i16, 3i16, 2i16, 2i16);
        } else {
            if (IsIntroTaskDone()) != 0 {
                DestroyTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Blur(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sBlur_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Blur_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        SetGpuReg(76u8, 0u16);
        SetGpuRegBits(10u8, 64u16);
        SetGpuRegBits(12u8, 64u16);
        SetGpuRegBits(14u8, 64u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Blur_Main(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) != 0i32 {
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(4i16);
            if (({
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == 10i32
            {
                BeginNormalPaletteFade(4294967295u32, (-1i8), 0u8, 16u8, 0u16);
            }
            SetGpuReg(
                76u8,
                (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    & 15i32)
                    .wrapping_mul(17i32)) as u16),
            );
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 14i32
            {
                let __p4 = ((task).wrapping_add(8)).cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Blur_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            let mut taskId: u8 = FindTaskIdByFunc(Some(Task_Blur));
            DestroyTask(taskId);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_Swirl(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sSwirl_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Swirl_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        InitTransitionData();
        ScanlineEffect_Clear();
        BeginNormalPaletteFade(4294967295u32, 4i8, 0u8, 16u8, 0u16);
        SetSinWave(
            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                .cast::<u16>())
            .cast::<i16>(),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<i16>())
            .read(),
            0i16,
            2i16,
            0i16,
            160i16,
        );
        SetVBlankCallback(Some(VBlankCB_Swirl));
        SetHBlankCallback(Some(HBlankCB_Swirl));
        EnableInterrupts(3u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Swirl_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
        SetSinWave(
            (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).cast::<i16>(),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<i16>())
            .read(),
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
            2i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read(),
            160i16,
        );
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            let mut taskId: u8 = FindTaskIdByFunc(Some(Task_Swirl));
            DestroyTask(taskId);
        }
        let __p3 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p3, ((__p3).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_Swirl() {
    unsafe {
        VBlankCB_BattleTransition();
        if ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).read_volatile())
            != 0
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(
                                    dmaRegs,
                                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .cast::<u16>())
                                        as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .wrapping_offset(1920))
                                    .cast::<u16>()) as usize
                                        as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2147483648i32)
                                        | crate::c::div_i32(320i32, crate::c::div_i32(16i32, 8i32)))
                                        as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
}
pub(crate) unsafe extern "C" fn HBlankCB_Swirl() {
    unsafe {
        let mut var: u16 = (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
            .wrapping_offset(1920))
        .cast::<u16>())
        .wrapping_offset(((((67108870i32) as usize as *mut u16).read_volatile()) as i32) as isize))
        .read();
        crate::c::volatile_write(((67108884i32) as usize as *mut u16), var);
        crate::c::volatile_write(((67108888i32) as usize as *mut u16), var);
        crate::c::volatile_write(((67108892i32) as usize as *mut u16), var);
    }
}
pub(crate) unsafe extern "C" fn Task_Shuffle(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sShuffle_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Shuffle_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        InitTransitionData();
        ScanlineEffect_Clear();
        BeginNormalPaletteFade(4294967295u32, 4i8, 0u8, 16u8, 0u16);
        crate::c::memset(
            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                .cast::<u16>())
            .cast::<u8>(),
            ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(22)
                .cast::<i16>())
            .read()) as i32),
            320u32,
        );
        SetVBlankCallback(Some(VBlankCB_Shuffle));
        SetHBlankCallback(Some(HBlankCB_Shuffle));
        EnableInterrupts(3u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Shuffle_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u8 = 0u8;
        let mut amplitude: u16 = 0u16;
        let mut sinVal: u16 = 0u16;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        sinVal = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16);
        amplitude = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            >> 8) as u16);
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4224i32)) as i16));
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(384i32)) as i16));
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    let mut sinIndex: u16 = ((crate::c::div_i32(((sinVal) as i32), 256i32)) as u16);
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(22)
                            .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(
                                ((Sin(((sinIndex) as i16), ((amplitude) as i16))) as i32),
                            )) as u16),
                    );
                }
                i = (i).wrapping_add(1);
                sinVal = ((((sinVal) as i32).wrapping_add(4224i32)) as u16);
            }
        }
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DestroyTask(FindTaskIdByFunc(Some(Task_Shuffle)));
        }
        let __p3 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p3, ((__p3).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_Shuffle() {
    unsafe {
        VBlankCB_BattleTransition();
        if ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).read_volatile())
            != 0
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(
                                    dmaRegs,
                                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .cast::<u16>())
                                        as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .wrapping_offset(1920))
                                    .cast::<u16>()) as usize
                                        as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2147483648i32)
                                        | crate::c::div_i32(320i32, crate::c::div_i32(16i32, 8i32)))
                                        as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
}
pub(crate) unsafe extern "C" fn HBlankCB_Shuffle() {
    unsafe {
        let mut var: u16 = (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
            .wrapping_offset(1920))
        .cast::<u16>())
        .wrapping_offset(((((67108870i32) as usize as *mut u16).read_volatile()) as i32) as isize))
        .read();
        crate::c::volatile_write(((67108886i32) as usize as *mut u16), var);
        crate::c::volatile_write(((67108890i32) as usize as *mut u16), var);
        crate::c::volatile_write(((67108894i32) as usize as *mut u16), var);
    }
}
pub(crate) unsafe extern "C" fn Task_BigPokeball(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sBigPokeball_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Aqua(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sAqua_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Magma(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sMagma_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Regice(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sRegice_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Registeel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sRegisteel_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Regirock(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sRegirock_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Kyogre(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sKyogre_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitPatternWeaveTransition(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut i: i32 = 0i32;
        InitTransitionData();
        ScanlineEffect_Clear();
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(16i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(16384i16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .write(63u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<u16>())
        .write(240u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .write(160u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14)
            .cast::<u16>())
        .write(16193u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<u16>())
        .write(
            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                << 8)
                | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32))
                as u16),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 160i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(240u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        SetVBlankCallback(Some(VBlankCB_PatternWeave));
    }
}
pub(crate) unsafe extern "C" fn Aqua_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(60i16);
        InitPatternWeaveTransition(task);
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (tilemap).cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
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
        LZ77UnCompVram(
            ((&raw const sTeamAqua_Tileset)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tileset).cast::<u8>(),
        );
        LoadPalette(
            (((&raw const sEvilTeam_Palette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Magma_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(60i16);
        InitPatternWeaveTransition(task);
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (tilemap).cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
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
        LZ77UnCompVram(
            ((&raw const sTeamMagma_Tileset)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tileset).cast::<u8>(),
        );
        LoadPalette(
            (((&raw const sEvilTeam_Palette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Regi_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(60i16);
        InitPatternWeaveTransition(task);
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (tilemap).cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
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
        'l5: loop {
            'l6: {
                'l7: loop {
                    'l8: {
                        CpuSet(
                            (((&raw const sRegis_Tileset)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .cast::<u8>(),
                            (tileset).cast::<u8>(),
                            ((0i32
                                | (crate::c::div_i32(8192i32, crate::c::div_i32(16i32, 8i32))
                                    & 2097151i32)) as u32),
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
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn BigPokeball_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        InitPatternWeaveTransition(task);
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (tilemap).cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
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
        'l5: loop {
            'l6: {
                'l7: loop {
                    'l8: {
                        CpuSet(
                            (((&raw const sBigPokeball_Tileset)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .cast::<u8>(),
                            (tileset).cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    1408u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
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
        LoadPalette(
            (((&raw const sFieldEffectPal_Pokeball)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn BigPokeball_SetGfx(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: i16 = 0i16;
        let mut j: i16 = 0i16;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        let mut bigPokeballMap: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        bigPokeballMap = ((&raw const sBigPokeball_Tilemap)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>();
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i16;
                        'l3: loop {
                            if !(((j) as i32) < 30i32) {
                                break 'l3;
                            }
                            'l4: {
                                let mut index: u32 = (((((i) as i32).wrapping_mul(32i32))
                                    .wrapping_add(((j) as i32)))
                                    as u32);
                                ((tilemap).wrapping_offset(((index) as i32) as isize)).write(
                                    (((((bigPokeballMap).read()) as i32) | 61440i32) as u16),
                                );
                            }
                            j = (j).wrapping_add(1);
                            bigPokeballMap = (bigPokeballMap).wrapping_offset(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        SetSinWave(
            (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).cast::<i16>(),
            0i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
            132i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read(),
            160i16,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Aqua_SetGfx(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        LZ77UnCompVram(
            ((&raw const sTeamAqua_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tilemap).cast::<u8>(),
        );
        SetSinWave(
            (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).cast::<i16>(),
            0i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
            132i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read(),
            160i16,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Magma_SetGfx(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        LZ77UnCompVram(
            ((&raw const sTeamMagma_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tilemap).cast::<u8>(),
        );
        SetSinWave(
            (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).cast::<i16>(),
            0i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
            132i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read(),
            160i16,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Regice_SetGfx(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        LoadPalette(
            (((&raw const sRegice_Palette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (((&raw const sRegice_Tilemap)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .cast::<u8>(),
                            (tilemap).cast::<u8>(),
                            ((0i32
                                | (crate::c::div_i32(1280i32, crate::c::div_i32(16i32, 8i32))
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
        SetSinWave(
            (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).cast::<i16>(),
            0i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
            132i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read(),
            160i16,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Registeel_SetGfx(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        LoadPalette(
            (((&raw const sRegisteel_Palette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (((&raw const sRegisteel_Tilemap)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .cast::<u8>(),
                            (tilemap).cast::<u8>(),
                            ((0i32
                                | (crate::c::div_i32(1280i32, crate::c::div_i32(16i32, 8i32))
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
        SetSinWave(
            (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).cast::<i16>(),
            0i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
            132i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read(),
            160i16,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Regirock_SetGfx(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        LoadPalette(
            (((&raw const sRegirock_Palette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (((&raw const sRegirock_Tilemap)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .cast::<u8>(),
                            (tilemap).cast::<u8>(),
                            ((0i32
                                | (crate::c::div_i32(1280i32, crate::c::div_i32(16i32, 8i32))
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
        SetSinWave(
            (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).cast::<i16>(),
            0i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
            132i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read(),
            160i16,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Kyogre_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (tilemap).cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
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
        LZ77UnCompVram(
            ((&raw const sKyogre_Tileset)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tileset).cast::<u8>(),
        );
        LZ77UnCompVram(
            ((&raw const sKyogre_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tilemap).cast::<u8>(),
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Kyogre_PaletteFlash(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if crate::c::rem_i32(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            3i32,
        ) == 0i32
        {
            let mut offset: u16 = ((crate::c::rem_i32(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
                30i32,
            )) as u16);
            offset = ((crate::c::div_i32(((offset) as i32), 3i32)) as u16);
            LoadPalette(
                ((((&raw const sKyogre1_Palette)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset((((offset) as i32).wrapping_mul(16i32)) as isize))
                .cast::<u8>(),
                240u16,
                32u16,
            );
        }
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 58i32
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Kyogre_PaletteBrighten(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if crate::c::rem_i32(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            5i32,
        ) == 0i32
        {
            let mut offset: i16 = ((crate::c::div_i32(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
                5i32,
            )) as i16);
            LoadPalette(
                ((((&raw const sKyogre2_Palette)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset((((offset) as i32).wrapping_mul(16i32)) as isize))
                .cast::<u8>(),
                240u16,
                32u16,
            );
        }
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 68i32
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(30i16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WeatherDuo_FadeOut(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        BeginNormalPaletteFade(4294934528u32, 1i8, 0u8, 16u8, 0u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WeatherDuo_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            {
                let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
                let __p1 = (dmaRegs).wrapping_offset(5);
                crate::c::volatile_write(
                    __p1,
                    (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
                );
                let __p2 = (dmaRegs).wrapping_offset(5);
                crate::c::volatile_write(
                    __p2,
                    (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
                );
                let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
            }
            FadeScreenBlack();
            DestroyTask(FindTaskIdByFunc(
                ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).read(),
            ));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PatternWeave_Blend1(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) == 0i32)
            || ((({
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32)
        {
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(2i16);
        }
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<u16>())
        .write(
            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                << 8)
                | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32))
                as u16),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) > 15i32 {
            let __p4 = ((task).wrapping_add(8)).cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
        }
        let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
        (__p5).write((((((__p5).read()) as i32).wrapping_add(8i32)) as i16));
        let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
        (__p6).write((((((__p6).read()) as i32).wrapping_sub(256i32)) as i16));
        SetSinWave(
            (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).cast::<i16>(),
            0i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
            132i16,
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32) >> 8)
                as i16),
            160i16,
        );
        let __p7 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p7, ((__p7).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PatternWeave_Blend2(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) == 0i32)
            || ((({
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32)
        {
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            (__p3).write(((__p3).read()).wrapping_sub(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(2i16);
        }
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<u16>())
        .write(
            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                << 8)
                | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32))
                as u16),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32 {
            let __p4 = ((task).wrapping_add(8)).cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
        }
        let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
        (__p5).write((((((__p5).read()) as i32).wrapping_add(8i32)) as i16));
        let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
        (__p6).write((((((__p6).read()) as i32).wrapping_sub(256i32)) as i16));
        SetSinWave(
            (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).cast::<i16>(),
            0i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
            132i16,
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32) >> 8)
                as i16),
            160i16,
        );
        let __p7 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p7, ((__p7).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PatternWeave_FinishAppear(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
        (__p2).write((((((__p2).read()) as i32).wrapping_sub(256i32)) as i16));
        SetSinWave(
            (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).cast::<i16>(),
            0i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
            132i16,
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32) >> 8)
                as i16),
            160i16,
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32) <= 0i32 {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(160i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(256i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        }
        let __p4 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p4, ((__p4).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FramesCountdown(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WeatherTrio_BgFadeBlack(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        BeginNormalPaletteFade(65535u32, 1i8, 0u8, 16u8, 0u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WeatherTrio_WaitFade(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PatternWeave_CircularMask(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) < 1024i32
        {
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(128i32)) as i16));
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) != 0i32 {
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_sub(
                    (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        >> 8),
                )) as i16),
            );
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                < 0i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            }
        }
        SetCircularMask(
            ((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>(),
            ((crate::c::div_i32(240i32, 2i32)) as i16),
            ((crate::c::div_i32(160i32, 2i32)) as i16),
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32 {
            SetVBlankCallback(None);
            {
                let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
                let __p3 = (dmaRegs).wrapping_offset(5);
                crate::c::volatile_write(
                    __p3,
                    (((((__p3).read_volatile()) as i32) & (-14849i32)) as u16),
                );
                let __p4 = (dmaRegs).wrapping_offset(5);
                crate::c::volatile_write(
                    __p4,
                    (((((__p4).read_volatile()) as i32) & (-32769i32)) as u16),
                );
                let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
            }
            FadeScreenBlack();
            DestroyTask(FindTaskIdByFunc(
                ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).read(),
            ));
        } else {
            if !((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) != 0) {
                let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                (__p5).write(((__p5).read()).wrapping_add(1));
                SetVBlankCallback(Some(VBlankCB_CircularMask));
            }
            let __p6 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
            crate::c::volatile_write(__p6, ((__p6).read_volatile()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_SetWinAndBlend() {
    unsafe {
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        VBlankCB_BattleTransition();
        if ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).read_volatile())
            != 0
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(
                                    dmaRegs,
                                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .cast::<u16>())
                                        as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .wrapping_offset(1920))
                                    .cast::<u16>()) as usize
                                        as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2147483648i32)
                                        | crate::c::div_i32(320i32, crate::c::div_i32(16i32, 8i32)))
                                        as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
        crate::c::volatile_write(
            ((67108936i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108938i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108932i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108944i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108946i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_PatternWeave() {
    unsafe {
        VBlankCB_SetWinAndBlend();
        'l1: loop {
            'l2: {
                {
                    let mut dmaRegs: *mut u32 = ((67109040i32) as usize as *mut u32);
                    crate::c::volatile_write(
                        dmaRegs,
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>()) as usize as u32),
                    );
                    crate::c::volatile_write(
                        (dmaRegs).wrapping_offset(1),
                        (((67108880i32) as usize as *mut u16) as usize as u32),
                    );
                    crate::c::volatile_write((dmaRegs).wrapping_offset(2), 2722103297u32);
                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_CircularMask() {
    unsafe {
        VBlankCB_SetWinAndBlend();
        'l1: loop {
            'l2: {
                {
                    let mut dmaRegs: *mut u32 = ((67109040i32) as usize as *mut u32);
                    crate::c::volatile_write(
                        dmaRegs,
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>()) as usize as u32),
                    );
                    crate::c::volatile_write(
                        (dmaRegs).wrapping_offset(1),
                        (((67108928i32) as usize as *mut u16) as usize as u32),
                    );
                    crate::c::volatile_write((dmaRegs).wrapping_offset(2), 2722103297u32);
                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PokeballsTrail(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sPokeballsTrail_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PokeballsTrail_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        'l1: loop {
            'l2: {
                CpuSet(
                    (((&raw const sPokeballTrail_Tileset)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    (tileset).cast::<u8>(),
                    32u32,
                );
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l3: loop {
            'l4: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l5: loop {
                        'l6: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (tilemap).cast::<u8>(),
                                ((83886080i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
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
        LoadPalette(
            (((&raw const sFieldEffectPal_Pokeball)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PokeballsTrail_Main(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: i16 = 0i16;
        let mut side: i16 = 0i16;
        let mut startX = crate::ffi::Align4([0u8; 4]);
        let mut delays = crate::ffi::Align4([0u8; 10]);
        crate::c::memcpy(
            ((&raw mut startX).cast::<i16>()).cast::<u8>(),
            (((&raw const sPokeballsTrail_StartXCoords)
                .cast::<u8>()
                .cast_mut()
                .cast::<i16>())
            .cast::<i16>())
            .cast::<u8>(),
            4u32,
        );
        crate::c::memcpy(
            ((&raw mut delays).cast::<i16>()).cast::<u8>(),
            (((&raw const sPokeballsTrail_Delays)
                .cast::<u8>()
                .cast_mut()
                .cast::<i16>())
            .cast::<i16>())
            .cast::<u8>(),
            10u32,
        );
        side = ((((Random()) as i32) & 1i32) as i16);
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).write(
                        (((((&raw mut startX).cast::<i16>())
                            .wrapping_offset(((side) as i32) as isize))
                        .read()) as i32),
                    );
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(1))
                    .write((((i) as i32).wrapping_mul(32i32)).wrapping_add(16i32));
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(2))
                    .write(((side) as i32));
                    ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                        .wrapping_offset(3))
                    .write(
                        (((((&raw mut delays).cast::<i16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32),
                    );
                    FieldEffectStart(45u8);
                }
                i = (i).wrapping_add(1);
                side = ((((side) as i32) ^ 1i32) as i16);
            }
        }
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PokeballsTrail_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((FieldEffectActiveListContains(45u8)) != 0) {
            FadeScreenBlack();
            DestroyTask(FindTaskIdByFunc(Some(Task_PokeballsTrail)));
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_PokeballTrail() -> u8 {
    unsafe {
        let mut spriteId: u8 = CreateSpriteAtEnd(
            (&raw const sSpriteTemplate_Pokeball)
                .cast::<u8>()
                .cast_mut(),
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
            0u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            (1u32) as i32,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(3))
                .read()) as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write((-1i16));
        InitSpriteAffineAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        StartSpriteAffineAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as u8),
        );
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FldEffPokeballTrail(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut speeds = crate::ffi::Align4([0u8; 4]);
        crate::c::memcpy(
            ((&raw mut speeds).cast::<i16>()).cast::<u8>(),
            (((&raw const sPokeballsTrail_Speeds)
                .cast::<u8>()
                .cast_mut()
                .cast::<i16>())
            .cast::<i16>())
            .cast::<u8>(),
            4u32,
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            != 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            if (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >= 0i32)
                && (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) <= 240i32)
            {
                let mut posX: i16 =
                    ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) >> 3) as i16);
                let mut posY: i16 =
                    ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) >> 3) as i16);
                if ((posX) as i32)
                    != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                {
                    let mut var: u32 = 0u32;
                    let mut ptr: *mut u16 = core::ptr::null_mut();
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(posX);
                    var = ((((((((67108872i32) as usize as *mut u16).read_volatile()) as i32)
                        >> 8)
                        & 31i32)
                        << 11) as u32);
                    ptr = (((100663296u32).wrapping_add(var)) as usize as *mut u16);
                    {
                        let mut index: u32 = ((((((posY) as i32).wrapping_sub(2i32))
                            .wrapping_mul(32i32))
                        .wrapping_add(((posX) as i32)))
                            as u32);
                        ((ptr).wrapping_offset(((index) as i32) as isize)).write(61441u16);
                    }
                    {
                        let mut index: u32 = ((((((posY) as i32).wrapping_sub(1i32))
                            .wrapping_mul(32i32))
                        .wrapping_add(((posX) as i32)))
                            as u32);
                        ((ptr).wrapping_offset(((index) as i32) as isize)).write(61441u16);
                    }
                    {
                        let mut index: u32 = ((((((posY) as i32).wrapping_sub(0i32))
                            .wrapping_mul(32i32))
                        .wrapping_add(((posX) as i32)))
                            as u32);
                        ((ptr).wrapping_offset(((index) as i32) as isize)).write(61441u16);
                    }
                    {
                        let mut index: u32 = ((((((posY) as i32).wrapping_add(1i32))
                            .wrapping_mul(32i32))
                        .wrapping_add(((posX) as i32)))
                            as u32);
                        ((ptr).wrapping_offset(((index) as i32) as isize)).write(61441u16);
                    }
                }
            }
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    (((((&raw mut speeds).cast::<i16>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
                    ))
                    .read()) as i32),
                )) as i16),
            );
            if (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < (-15i32))
                || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > 255i32)
            {
                FieldEffectStop(sprite, 45u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ClockwiseWipe(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sClockwiseWipe_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ClockwiseWipe_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u16 = 0u16;
        InitTransitionData();
        ScanlineEffect_Clear();
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .write(63u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<u16>())
        .write(61681u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .write(160u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(62452u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        SetVBlankCallback(Some(VBlankCB_ClockwiseWipe));
        ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
            .cast::<i16>())
        .wrapping_offset(4))
        .write(((crate::c::div_i32(240i32, 2i32)) as i16));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ClockwiseWipe_TopRight(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        InitBlackWipe(
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
                .cast::<i16>(),
            ((crate::c::div_i32(240i32, 2i32)) as i16),
            ((crate::c::div_i32(160i32, 2i32)) as i16),
            ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<i16>())
            .wrapping_offset(4))
            .read(),
            0i16,
            1i16,
            1i16,
        );
        'l1: loop {
            'l2: {
                ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                    .wrapping_offset(
                        ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(36))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .read()) as i32) as isize,
                    ))
                .write(
                    ((((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        .wrapping_add(1i32)
                        | (crate::c::div_i32(240i32, 2i32) << 8)) as u16),
                );
            }
            if !(!((UpdateBlackWipe(
                ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .cast::<i16>(),
                1u8,
                1u8,
            )) != 0))
            {
                break 'l1;
            }
        }
        let __p1 = (((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36))
        .cast::<i16>())
        .wrapping_offset(4);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(16i32)) as i16));
        if ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as i32)
            >= 240i32
        {
            ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(0i16);
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        let __p3 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p3, ((__p3).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ClockwiseWipe_Right(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut start: i16 = 0i16;
        let mut end: i16 = 0i16;
        let mut finished: u8 = 0u8;
        (&raw mut finished).write_volatile(0u8);
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        InitBlackWipe(
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
                .cast::<i16>(),
            ((crate::c::div_i32(240i32, 2i32)) as i16),
            ((crate::c::div_i32(160i32, 2i32)) as i16),
            240i16,
            ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<i16>())
            .wrapping_offset(5))
            .read(),
            1i16,
            1i16,
        );
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            start = ((crate::c::div_i32(240i32, 2i32)) as i16);
            end = ((((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
                .wrapping_add(1i32)) as i16);
            if ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as i32)
                >= crate::c::div_i32(160i32, 2i32)
            {
                start = ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .cast::<i16>())
                .wrapping_offset(2))
                .read();
                end = 240i16;
            }
            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).wrapping_offset(
                ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32) as isize,
            ))
            .write(((((end) as i32) | (((start) as i32) << 8)) as u16));
            if ((&raw mut finished).read_volatile()) != 0 {
                break 'l1;
            }
            crate::c::volatile_write(
                (&raw mut finished),
                UpdateBlackWipe(
                    ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .cast::<i16>(),
                    1u8,
                    1u8,
                ),
            );
        }
        let __p1 = (((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36))
        .cast::<i16>())
        .wrapping_offset(5);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        if ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36))
        .cast::<i16>())
        .wrapping_offset(5))
        .read()) as i32)
            >= 160i32
        {
            ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(240i16);
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        } else {
            'l2: loop {
                if !(((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    < ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32))
                {
                    break 'l2;
                }
                ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                    .wrapping_offset(
                        (({
                            let __p3 =
                                (((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(36))
                                .cast::<i16>())
                                .wrapping_offset(3);
                            let __t4 = ((__p3).read()).wrapping_add(1);
                            (__p3).write(__t4);
                            __t4
                        }) as i32) as isize,
                    ))
                .write(((((end) as i32) | (((start) as i32) << 8)) as u16));
            }
        }
        let __p5 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p5, ((__p5).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ClockwiseWipe_Bottom(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        InitBlackWipe(
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
                .cast::<i16>(),
            ((crate::c::div_i32(240i32, 2i32)) as i16),
            ((crate::c::div_i32(160i32, 2i32)) as i16),
            ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<i16>())
            .wrapping_offset(4))
            .read(),
            160i16,
            1i16,
            1i16,
        );
        'l1: loop {
            'l2: {
                ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                    .wrapping_offset(
                        ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(36))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .read()) as i32) as isize,
                    ))
                .write(
                    (((((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        << 8)
                        | 240i32) as u16),
                );
            }
            if !(!((UpdateBlackWipe(
                ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .cast::<i16>(),
                1u8,
                1u8,
            )) != 0))
            {
                break 'l1;
            }
        }
        let __p1 = (((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36))
        .cast::<i16>())
        .wrapping_offset(4);
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(16i32)) as i16));
        if ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as i32)
            <= 0i32
        {
            ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(160i16);
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        let __p3 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p3, ((__p3).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ClockwiseWipe_Left(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut end: i16 = 0i16;
        let mut start: i16 = 0i16;
        let mut temp: i16 = 0i16;
        let mut finished: u8 = 0u8;
        (&raw mut finished).write_volatile(0u8);
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        InitBlackWipe(
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
                .cast::<i16>(),
            ((crate::c::div_i32(240i32, 2i32)) as i16),
            ((crate::c::div_i32(160i32, 2i32)) as i16),
            0i16,
            ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<i16>())
            .wrapping_offset(5))
            .read(),
            1i16,
            1i16,
        );
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            end = ((((((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                .wrapping_offset(
                    ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32) as isize,
                ))
            .read()) as i32)
                & 255i32) as i16);
            start = ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<i16>())
            .wrapping_offset(2))
            .read();
            if ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as i32)
                <= crate::c::div_i32(160i32, 2i32)
            {
                start = ((crate::c::div_i32(240i32, 2i32)) as i16);
                end = ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .cast::<i16>())
                .wrapping_offset(2))
                .read();
            }
            temp = ((((end) as i32) | (((start) as i32) << 8)) as i16);
            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).wrapping_offset(
                ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32) as isize,
            ))
            .write(((temp) as u16));
            if ((&raw mut finished).read_volatile()) != 0 {
                break 'l1;
            }
            crate::c::volatile_write(
                (&raw mut finished),
                UpdateBlackWipe(
                    ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .cast::<i16>(),
                    1u8,
                    1u8,
                ),
            );
        }
        let __p1 = (((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36))
        .cast::<i16>())
        .wrapping_offset(5);
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(8i32)) as i16));
        if ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36))
        .cast::<i16>())
        .wrapping_offset(5))
        .read()) as i32)
            <= 0i32
        {
            ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(0i16);
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        } else {
            'l2: loop {
                if !(((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    > ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as i32))
                {
                    break 'l2;
                }
                ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                    .wrapping_offset(
                        (({
                            let __p3 =
                                (((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(36))
                                .cast::<i16>())
                                .wrapping_offset(3);
                            let __t4 = ((__p3).read()).wrapping_sub(1);
                            (__p3).write(__t4);
                            __t4
                        }) as i32) as isize,
                    ))
                .write(((((end) as i32) | (((start) as i32) << 8)) as u16));
            }
        }
        let __p5 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p5, ((__p5).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ClockwiseWipe_TopLeft(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        InitBlackWipe(
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
                .cast::<i16>(),
            ((crate::c::div_i32(240i32, 2i32)) as i16),
            ((crate::c::div_i32(160i32, 2i32)) as i16),
            ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36))
            .cast::<i16>())
            .wrapping_offset(4))
            .read(),
            0i16,
            1i16,
            1i16,
        );
        'l1: loop {
            'l2: {
                let mut start: i16 = 0i16;
                let mut end: i16 = 0i16;
                start = ((crate::c::div_i32(240i32, 2i32)) as i16);
                end = ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .cast::<i16>())
                .wrapping_offset(2))
                .read();
                if ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    >= crate::c::div_i32(240i32, 2i32)
                {
                    start = 0i16;
                    end = 240i16;
                }
                ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                    .wrapping_offset(
                        ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(36))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .read()) as i32) as isize,
                    ))
                .write(((((end) as i32) | (((start) as i32) << 8)) as u16));
            }
            if !(!((UpdateBlackWipe(
                ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36))
                .cast::<i16>(),
                1u8,
                1u8,
            )) != 0))
            {
                break 'l1;
            }
        }
        let __p1 = (((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36))
        .cast::<i16>())
        .wrapping_offset(4);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(16i32)) as i16));
        if ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            > crate::c::div_i32(240i32, 2i32)
        {
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        let __p3 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p3, ((__p3).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ClockwiseWipe_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        FadeScreenBlack();
        DestroyTask(FindTaskIdByFunc(Some(Task_ClockwiseWipe)));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_ClockwiseWipe() {
    unsafe {
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        VBlankCB_BattleTransition();
        if (((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).read_volatile())
            as i32)
            != 0i32
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(
                                    dmaRegs,
                                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .cast::<u16>())
                                        as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .wrapping_offset(1920))
                                    .cast::<u16>()) as usize
                                        as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2147483648i32)
                                        | crate::c::div_i32(320i32, crate::c::div_i32(16i32, 8i32)))
                                        as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
        crate::c::volatile_write(
            ((67108936i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108938i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108932i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108928i32) as usize as *mut u16),
            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                .cast::<u16>())
            .read(),
        );
        'l5: loop {
            'l6: {
                {
                    let mut dmaRegs: *mut u32 = ((67109040i32) as usize as *mut u32);
                    crate::c::volatile_write(
                        dmaRegs,
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>()) as usize as u32),
                    );
                    crate::c::volatile_write(
                        (dmaRegs).wrapping_offset(1),
                        (((67108928i32) as usize as *mut u16) as usize as u32),
                    );
                    crate::c::volatile_write((dmaRegs).wrapping_offset(2), 2722103297u32);
                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Ripple(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sRipple_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Ripple_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u8 = 0u8;
        InitTransitionData();
        ScanlineEffect_Clear();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(22)
                            .cast::<i16>())
                        .read()) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        SetVBlankCallback(Some(VBlankCB_Ripple));
        SetHBlankCallback(Some(HBlankCB_Ripple));
        EnableInterrupts(2u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Ripple_Main(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u8 = 0u8;
        let mut amplitude: i16 = 0i16;
        let mut sinVal: u16 = 0u16;
        let mut speed: u16 = 0u16;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        amplitude = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            >> 8) as i16);
        sinVal = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16);
        speed = 384u16;
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(1024i32)) as i16));
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            <= 8191i32
        {
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(384i32)) as i16));
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    let mut sinIndex: i16 = ((((sinVal) as i32) >> 8) as i16);
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(22)
                            .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(
                                ((Sin(((((sinIndex) as i32) & 65535i32) as i16), amplitude))
                                    as i32),
                            )) as u16),
                    );
                }
                i = (i).wrapping_add(1);
                sinVal = ((((sinVal) as i32).wrapping_add(((speed) as i32))) as u16);
            }
        }
        if (({
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 81i32
        {
            let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            (__p5).write(((__p5).read()).wrapping_add(1));
            BeginNormalPaletteFade(4294967295u32, (-2i8), 0u8, 16u8, 0u16);
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) != 0)
            && (!((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0))
        {
            DestroyTask(FindTaskIdByFunc(Some(Task_Ripple)));
        }
        let __p6 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p6, ((__p6).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_Ripple() {
    unsafe {
        VBlankCB_BattleTransition();
        if ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).read_volatile())
            != 0
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(
                                    dmaRegs,
                                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .cast::<u16>())
                                        as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .wrapping_offset(1920))
                                    .cast::<u16>()) as usize
                                        as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2147483648i32)
                                        | crate::c::div_i32(320i32, crate::c::div_i32(16i32, 8i32)))
                                        as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
}
pub(crate) unsafe extern "C" fn HBlankCB_Ripple() {
    unsafe {
        let mut var: u16 = (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
            .wrapping_offset(1920))
        .cast::<u16>())
        .wrapping_offset(((((67108870i32) as usize as *mut u16).read_volatile()) as i32) as isize))
        .read();
        crate::c::volatile_write(((67108886i32) as usize as *mut u16), var);
        crate::c::volatile_write(((67108890i32) as usize as *mut u16), var);
        crate::c::volatile_write(((67108894i32) as usize as *mut u16), var);
    }
}
pub(crate) unsafe extern "C" fn Task_Wave(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sWave_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Wave_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u8 = 0u8;
        InitTransitionData();
        ScanlineEffect_Clear();
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .write(63u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<u16>())
        .write(240u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .write(160u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(242u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        SetVBlankCallback(Some(VBlankCB_Wave));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Wave_Main(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u8 = 0u8;
        let mut sinIndex: u8 = 0u8;
        let mut toStore: *mut u16 = core::ptr::null_mut();
        let mut finished: u8 = 0u8;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        toStore = ((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>();
        sinIndex = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u8);
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(16i32)) as i16));
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
        {
            i = 0u8;
            finished = 1u8;
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    let mut x: i16 = ((((((((task).wrapping_add(8)).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_add(((Sin(((sinIndex) as i16), 40i16)) as i32)))
                        as i16);
                    if ((x) as i32) < 0i32 {
                        x = 0i16;
                    }
                    if ((x) as i32) > 240i32 {
                        x = 240i16;
                    }
                    (toStore).write((((((x) as i32) << 8) | 241i32) as u16));
                    if ((x) as i32) < 240i32 {
                        finished = 0u8;
                    }
                }
                i = (i).wrapping_add(1);
                sinIndex = ((((sinIndex) as i32).wrapping_add(4i32)) as u8);
                toStore = (toStore).wrapping_offset(1);
            }
        }
        if (finished) != 0 {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        let __p4 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p4, ((__p4).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Wave_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        FadeScreenBlack();
        DestroyTask(FindTaskIdByFunc(Some(Task_Wave)));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_Wave() {
    unsafe {
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        VBlankCB_BattleTransition();
        if (((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).read_volatile())
            as i32)
            != 0i32
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(
                                    dmaRegs,
                                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .cast::<u16>())
                                        as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .wrapping_offset(1920))
                                    .cast::<u16>()) as usize
                                        as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2147483648i32)
                                        | crate::c::div_i32(320i32, crate::c::div_i32(16i32, 8i32)))
                                        as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
        crate::c::volatile_write(
            ((67108936i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108938i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108932i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read(),
        );
        'l5: loop {
            'l6: {
                {
                    let mut dmaRegs: *mut u32 = ((67109040i32) as usize as *mut u32);
                    crate::c::volatile_write(
                        dmaRegs,
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>()) as usize as u32),
                    );
                    crate::c::volatile_write(
                        (dmaRegs).wrapping_offset(1),
                        (((67108928i32) as usize as *mut u16) as usize as u32),
                    );
                    crate::c::volatile_write((dmaRegs).wrapping_offset(2), 2722103297u32);
                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_Sidney(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(0i16);
        DoMugshotTransition(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_Phoebe(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(1i16);
        DoMugshotTransition(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_Glacia(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(2i16);
        DoMugshotTransition(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_Drake(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(3i16);
        DoMugshotTransition(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_Champion(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(4i16);
        DoMugshotTransition(taskId);
    }
}
pub(crate) unsafe extern "C" fn DoMugshotTransition(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sMugshot_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Mugshot_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u8 = 0u8;
        InitTransitionData();
        ScanlineEffect_Clear();
        Mugshots_CreateTrainerPics(task);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(1i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(239i16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .write(63u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .write(62u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .write(160u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(61681u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        SetVBlankCallback(Some(VBlankCB_Mugshots));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Mugshot_SetGfx(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: i16 = 0i16;
        let mut j: i16 = 0i16;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        let mut mugshotsMap: *mut u16 = core::ptr::null_mut();
        mugshotsMap = ((&raw const sMugshotsTilemap)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        'l1: loop {
            'l2: {
                CpuSet(
                    (((&raw const sEliteFour_Tileset)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    (tileset).cast::<u8>(),
                    240u32,
                );
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        LoadPalette(
            (((((&raw const sOpponentMugshotsPals)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize,
            ))
            .read())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        LoadPalette(
            (((((&raw const sPlayerMugshotsPals)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u16>())
            .cast::<*mut u16>())
            .wrapping_offset(
                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                    as i32) as isize,
            ))
            .read())
            .cast::<u8>(),
            250u16,
            12u16,
        );
        {
            i = 0i16;
            'l3: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l3;
                }
                'l4: {
                    {
                        j = 0i16;
                        'l5: loop {
                            if !(((j) as i32) < 32i32) {
                                break 'l5;
                            }
                            'l6: {
                                let mut index: u32 = (((((i) as i32).wrapping_mul(32i32))
                                    .wrapping_add(((j) as i32)))
                                    as u32);
                                ((tilemap).wrapping_offset(((index) as i32) as isize))
                                    .write((((((mugshotsMap).read()) as i32) | 61440i32) as u16));
                            }
                            j = (j).wrapping_add(1);
                            mugshotsMap = (mugshotsMap).wrapping_offset(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        EnableInterrupts(2u16);
        SetHBlankCallback(Some(HBlankCB_Mugshots));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Mugshot_ShowBanner(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u8 = 0u8;
        let mut sinIndex: u8 = 0u8;
        let mut toStore: *mut u16 = core::ptr::null_mut();
        let mut x: i16 = 0i16;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        toStore = ((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>();
        sinIndex = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8);
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(16i32)) as i16));
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < crate::c::div_i32(160i32, 2i32)) {
                    break 'l1;
                }
                'l2: {
                    x = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        .wrapping_add(((Sin(((sinIndex) as i16), 16i16)) as i32)))
                        as i16);
                    if ((x) as i32) < 0i32 {
                        x = 1i16;
                    }
                    if ((x) as i32) > 240i32 {
                        x = 240i16;
                    }
                    (toStore).write(((x) as u16));
                }
                i = (i).wrapping_add(1);
                toStore = (toStore).wrapping_offset(1);
                sinIndex = ((((sinIndex) as i32).wrapping_add(16i32)) as u8);
            }
        }
        {
            'l3: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l3;
                }
                'l4: {
                    x = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        .wrapping_sub(((Sin(((sinIndex) as i16), 16i16)) as i32)))
                        as i16);
                    if ((x) as i32) < 0i32 {
                        x = 0i16;
                    }
                    if ((x) as i32) > 239i32 {
                        x = 239i16;
                    }
                    (toStore).write((((((x) as i32) << 8) | 240i32) as u16));
                }
                i = (i).wrapping_add(1);
                toStore = (toStore).wrapping_offset(1);
                sinIndex = ((((sinIndex) as i32).wrapping_add(16i32)) as u8);
            }
        }
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
        let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
        (__p3).write((((((__p3).read()) as i32).wrapping_sub(8i32)) as i16));
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) > 240i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(240i16);
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) < 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        }
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 240i32)
            && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                == 0i32)
        {
            let __p4 = ((task).wrapping_add(8)).cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
        }
        let __p5 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<i16>();
        (__p5).write((((((__p5).read()) as i32).wrapping_sub(8i32)) as i16));
        let __p6 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(26)
            .cast::<i16>();
        (__p6).write((((((__p6).read()) as i32).wrapping_add(8i32)) as i16));
        let __p7 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p7, ((__p7).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Mugshot_StartOpponentSlide(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u8 = 0u8;
        let mut toStore: *mut u16 = core::ptr::null_mut();
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        {
            i = 0u8;
            toStore = ((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>();
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    (toStore).write(240u16);
                }
                i = (i).wrapping_add(1);
                toStore = (toStore).wrapping_offset(1);
            }
        }
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        let __p2 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_sub(8i32)) as i16));
        let __p3 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(26)
            .cast::<i16>();
        (__p3).write((((((__p3).read()) as i32).wrapping_add(8i32)) as i16));
        SetTrainerPicSlideDirection(
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read(),
            0i16,
        );
        SetTrainerPicSlideDirection(
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read(),
            1i16,
        );
        IncrementTrainerPicState(
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read(),
        );
        PlaySE(104u16);
        let __p4 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p4, ((__p4).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Mugshot_WaitStartPlayerSlide(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let __p1 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(8i32)) as i16));
        let __p2 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(26)
            .cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
        if (IsTrainerPicSlideDone(
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read(),
        )) != 0
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            IncrementTrainerPicState(
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read(),
            );
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Mugshot_WaitPlayerSlide(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let __p1 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(8i32)) as i16));
        let __p2 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(26)
            .cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
        if (IsTrainerPicSlideDone(
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read(),
        )) != 0
        {
            crate::c::volatile_write(
                (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
                0u8,
            );
            SetVBlankCallback(None);
            {
                let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
                let __p3 = (dmaRegs).wrapping_offset(5);
                crate::c::volatile_write(
                    __p3,
                    (((((__p3).read_volatile()) as i32) & (-14849i32)) as u16),
                );
                let __p4 = (dmaRegs).wrapping_offset(5);
                crate::c::volatile_write(
                    __p4,
                    (((((__p4).read_volatile()) as i32) & (-32769i32)) as u16),
                );
                let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
            }
            crate::c::memset(
                (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).cast::<u8>(),
                0i32,
                320u32,
            );
            crate::c::memset(
                ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                    .cast::<u16>())
                .cast::<u8>(),
                0i32,
                320u32,
            );
            SetGpuReg(64u8, 240u16);
            SetGpuReg(84u8, 0u16);
            let __p5 = ((task).wrapping_add(8)).cast::<i16>();
            (__p5).write(((__p5).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<u16>())
            .write(191u16);
            SetVBlankCallback(Some(VBlankCB_MugshotsFadeOut));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Mugshot_GradualWhiteFade(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut active: u32 = 0u32;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        active = 1u32;
        let __p1 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(8i32)) as i16));
        let __p2 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(26)
            .cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            < crate::c::div_i32(160i32, 2i32)
        {
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as i16));
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            > crate::c::div_i32(160i32, 2i32)
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                .write(((crate::c::div_i32(160i32, 2i32)) as i16));
        }
        if ((({
            let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
            let __t5 = ((__p4).read()).wrapping_add(1);
            (__p4).write(__t5);
            __t5
        }) as i32)
            & 1i32)
            != 0
        {
            let mut i: i16 = 0i16;
            {
                i = 0i16;
                active = 0u32;
                'l1: loop {
                    if !(((i) as i32)
                        <= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        let mut index1: i16 =
                            (((crate::c::div_i32(160i32, 2i32)).wrapping_sub(((i) as i32))) as i16);
                        let mut index2: i16 =
                            (((crate::c::div_i32(160i32, 2i32)).wrapping_add(((i) as i32))) as i16);
                        if ((((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                            .wrapping_offset(((index1) as i32) as isize))
                        .read()) as i32)
                            <= 15i32
                        {
                            active = 1u32;
                            let __p6 = (((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                .cast::<u16>())
                            .wrapping_offset(((index1) as i32) as isize);
                            (__p6).write(((__p6).read()).wrapping_add(1));
                        }
                        if ((((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                            .wrapping_offset(((index2) as i32) as isize))
                        .read()) as i32)
                            <= 15i32
                        {
                            active = 1u32;
                            let __p7 = (((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                .cast::<u16>())
                            .wrapping_offset(((index2) as i32) as isize);
                            (__p7).write(((__p7).read()).wrapping_add(1));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            == crate::c::div_i32(160i32, 2i32))
            && (!((active) != 0))
        {
            let __p8 = ((task).wrapping_add(8)).cast::<i16>();
            (__p8).write(((__p8).read()).wrapping_add(1));
        }
        let __p9 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p9, ((__p9).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Mugshot_InitFadeWhiteToBlack(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        BlendPalettes(4294967295u32, 16u8, 32767u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14)
            .cast::<u16>())
        .write(255u16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Mugshot_FadeToBlack(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
        (__p1).write(((__p1).read()).wrapping_add(1));
        crate::c::memset(
            (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).cast::<u8>(),
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
            320u32,
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) > 15i32 {
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        let __p3 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p3, ((__p3).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Mugshot_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        FadeScreenBlack();
        DestroyTask(FindTaskIdByFunc(
            ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).read(),
        ));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_Mugshots() {
    unsafe {
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        VBlankCB_BattleTransition();
        if (((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).read_volatile())
            as i32)
            != 0i32
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(
                                    dmaRegs,
                                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .cast::<u16>())
                                        as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .wrapping_offset(1920))
                                    .cast::<u16>()) as usize
                                        as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2147483648i32)
                                        | crate::c::div_i32(320i32, crate::c::div_i32(16i32, 8i32)))
                                        as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
        crate::c::volatile_write(
            ((67108882i32) as usize as *mut u16),
            ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<i16>())
            .read()) as u16),
        );
        crate::c::volatile_write(
            ((67108936i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108938i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108932i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read(),
        );
        'l5: loop {
            'l6: {
                {
                    let mut dmaRegs: *mut u32 = ((67109040i32) as usize as *mut u32);
                    crate::c::volatile_write(
                        dmaRegs,
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>()) as usize as u32),
                    );
                    crate::c::volatile_write(
                        (dmaRegs).wrapping_offset(1),
                        (((67108928i32) as usize as *mut u16) as usize as u32),
                    );
                    crate::c::volatile_write((dmaRegs).wrapping_offset(2), 2722103297u32);
                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_MugshotsFadeOut() {
    unsafe {
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        VBlankCB_BattleTransition();
        if (((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).read_volatile())
            as i32)
            != 0i32
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(
                                    dmaRegs,
                                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .cast::<u16>())
                                        as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .wrapping_offset(1920))
                                    .cast::<u16>()) as usize
                                        as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2147483648i32)
                                        | crate::c::div_i32(320i32, crate::c::div_i32(16i32, 8i32)))
                                        as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
        crate::c::volatile_write(
            ((67108944i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<u16>())
            .read(),
        );
        'l5: loop {
            'l6: {
                {
                    let mut dmaRegs: *mut u32 = ((67109040i32) as usize as *mut u32);
                    crate::c::volatile_write(
                        dmaRegs,
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>()) as usize as u32),
                    );
                    crate::c::volatile_write(
                        (dmaRegs).wrapping_offset(1),
                        (((67108948i32) as usize as *mut u16) as usize as u32),
                    );
                    crate::c::volatile_write((dmaRegs).wrapping_offset(2), 2722103297u32);
                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HBlankCB_Mugshots() {
    unsafe {
        if ((((67108870i32) as usize as *mut u16).read_volatile()) as i32)
            < crate::c::div_i32(160i32, 2i32)
        {
            crate::c::volatile_write(
                ((67108880i32) as usize as *mut u16),
                ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<i16>())
                .read()) as u16),
            );
        } else {
            crate::c::volatile_write(
                ((67108880i32) as usize as *mut u16),
                ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(26)
                    .cast::<i16>())
                .read()) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Mugshots_CreateTrainerPics(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut opponentSprite: *mut u8 = core::ptr::null_mut();
        let mut playerSprite: *mut u8 = core::ptr::null_mut();
        let mut mugshotId: i16 =
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read();
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
            ((CreateTrainerSprite(
                ((((&raw const sMugshotsTrainerPicIDsTable)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((mugshotId) as i32) as isize))
                .read(),
                (((((((((&raw const sMugshotsOpponentCoords).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(((mugshotId) as i32) as isize * 4))
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(32i32)) as i16),
                ((((((((((&raw const sMugshotsOpponentCoords).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(((mugshotId) as i32) as isize * 4))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_add(42i32)) as i16),
                0u8,
                (&raw mut gDecompressionBuffer).cast::<u8>(),
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
            ((CreateTrainerSprite(
                ((PlayerGenderToFrontTrainerPicId(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read(),
                )) as u8),
                272i16,
                106i16,
                0u8,
                (&raw mut gDecompressionBuffer).cast::<u8>(),
            )) as i16),
        );
        opponentSprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as i32)
                as isize
                * 68,
        );
        playerSprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as i32)
                as isize
                * 68,
        );
        ((opponentSprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_MugshotTrainerPic));
        ((playerSprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_MugshotTrainerPic));
        crate::c::bf_write((opponentSprite).wrapping_add(1), 0, 2, (3u32) as i32);
        crate::c::bf_write((playerSprite).wrapping_add(1), 0, 2, (3u32) as i32);
        crate::c::bf_write(
            (opponentSprite).wrapping_add(3),
            1,
            5,
            ((AllocOamMatrix()) as u32) as i32,
        );
        crate::c::bf_write(
            (playerSprite).wrapping_add(3),
            1,
            5,
            ((AllocOamMatrix()) as u32) as i32,
        );
        crate::c::bf_write((opponentSprite).wrapping_add(1), 6, 2, (1u32) as i32);
        crate::c::bf_write((playerSprite).wrapping_add(1), 6, 2, (1u32) as i32);
        crate::c::bf_write((opponentSprite).wrapping_add(3), 6, 2, (3u32) as i32);
        crate::c::bf_write((playerSprite).wrapping_add(3), 6, 2, (3u32) as i32);
        CalcCenterToCornerVec(opponentSprite, 1u8, 3u8, 3u8);
        CalcCenterToCornerVec(playerSprite, 1u8, 3u8, 3u8);
        SetOamMatrixRotationScaling(
            ((crate::c::bf_read((opponentSprite).wrapping_add(3), 1, 5, false) as u32) as u8),
            (((((&raw const sMugshotsOpponentRotationScales)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((mugshotId) as i32) as isize * 4))
            .cast::<i16>())
            .read(),
            ((((((&raw const sMugshotsOpponentRotationScales)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((mugshotId) as i32) as isize * 4))
            .cast::<i16>())
            .wrapping_offset(1))
            .read(),
            0u16,
        );
        SetOamMatrixRotationScaling(
            ((crate::c::bf_read((playerSprite).wrapping_add(3), 1, 5, false) as u32) as u8),
            (-512i16),
            512i16,
            0u16,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MugshotTrainerPic(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: loop {
            if !(((((((&raw const sMugshotTrainerPicFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(sprite))
                != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn MugshotTrainerPic_Pause(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn MugshotTrainerPic_Init(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        let mut speeds = crate::ffi::Align4([0u8; 4]);
        let mut accels = crate::ffi::Align4([0u8; 4]);
        crate::c::memcpy(
            ((&raw mut speeds).cast::<i16>()).cast::<u8>(),
            (((&raw const sTrainerPicSlideSpeeds)
                .cast::<u8>()
                .cast_mut()
                .cast::<i16>())
            .cast::<i16>())
            .cast::<u8>(),
            4u32,
        );
        crate::c::memcpy(
            ((&raw mut accels).cast::<i16>()).cast::<u8>(),
            (((&raw const sTrainerPicSlideAccels)
                .cast::<u8>()
                .cast_mut()
                .cast::<i16>())
            .cast::<i16>())
            .cast::<u8>(),
            4u32,
        );
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            (((&raw mut speeds).cast::<i16>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                    as isize,
            ))
            .read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            (((&raw mut accels).cast::<i16>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                    as isize,
            ))
            .read(),
        );
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn MugshotTrainerPic_Slide(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) != 0)
            && (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < 133i32)
        {
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        } else {
            if (!((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) != 0))
                && (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > 103i32)
            {
                let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn MugshotTrainerPic_SlideSlow(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(32).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 0i32
        {
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    .wrapping_neg()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(1i16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn MugshotTrainerPic_SlideOffscreen(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(32).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        if (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < (-31i32))
            || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > 271i32)
        {
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SetTrainerPicSlideDirection(spriteId: i16, dirId: i16) {
    unsafe {
        let mut spriteId = spriteId;
        let mut dirId = dirId;
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(dirId);
    }
}
pub(crate) unsafe extern "C" fn IncrementTrainerPicState(spriteId: i16) {
    unsafe {
        let mut spriteId = spriteId;
        let __p1 = ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn IsTrainerPicSlideDone(spriteId: i16) -> i16 {
    unsafe {
        let mut spriteId = spriteId;
        return ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .read();
    }
}
pub(crate) unsafe extern "C" fn Task_Slice(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sSlice_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Slice_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u16 = 0u16;
        InitTransitionData();
        ScanlineEffect_Clear();
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(256i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .write(63u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .write(160u16);
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<i16>())
                        .read()) as u16),
                    );
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((160i32).wrapping_add(((i) as i32))) as isize))
                    .write(240u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        EnableInterrupts(2u16);
        SetGpuRegBits(4u8, 16u16);
        SetVBlankCallback(Some(VBlankCB_Slice));
        SetHBlankCallback(Some(HBlankCB_Slice));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Slice_Main(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u16 = 0u16;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    >> 8),
            )) as i16),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) > 240i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(240i16);
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            <= 4095i32
        {
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
                )) as i16),
            );
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) < 128i32
        {
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
            (__p3).write((((((__p3).read()) as i32) << 1) as i16));
        }
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    let mut storeLoc1: *mut u16 =
                        (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize);
                    let mut storeLoc2: *mut u16 =
                        (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                            .wrapping_offset((((i) as i32).wrapping_add(160i32)) as isize);
                    if (crate::c::rem_i32(((i) as i32), 2i32)) != 0 {
                        (storeLoc1).write(
                            ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(20)
                            .cast::<i16>())
                            .read()) as i32)
                                .wrapping_add(
                                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                        .read()) as i32),
                                )) as u16),
                        );
                        (storeLoc2).write(
                            (((240i32).wrapping_sub(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                    .read()) as i32),
                            )) as u16),
                        );
                    } else {
                        (storeLoc1).write(
                            ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(20)
                            .cast::<i16>())
                            .read()) as i32)
                                .wrapping_sub(
                                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                        .read()) as i32),
                                )) as u16),
                        );
                        (storeLoc2).write(
                            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                .read()) as i32)
                                << 8)
                                | 241i32) as u16),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) >= 240i32
        {
            let __p4 = ((task).wrapping_add(8)).cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
        }
        let __p5 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p5, ((__p5).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Slice_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        FadeScreenBlack();
        DestroyTask(FindTaskIdByFunc(Some(Task_Slice)));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_Slice() {
    unsafe {
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        VBlankCB_BattleTransition();
        crate::c::volatile_write(
            ((67108936i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108938i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108932i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read(),
        );
        if ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).read_volatile())
            != 0
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(
                                    dmaRegs,
                                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .cast::<u16>())
                                        as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .wrapping_offset(1920))
                                    .cast::<u16>()) as usize
                                        as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2147483648i32)
                                        | crate::c::div_i32(640i32, crate::c::div_i32(16i32, 8i32)))
                                        as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
        'l5: loop {
            'l6: {
                {
                    let mut dmaRegs: *mut u32 = ((67109040i32) as usize as *mut u32);
                    crate::c::volatile_write(
                        dmaRegs,
                        ((((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>())
                        .wrapping_offset(160)) as usize as u32),
                    );
                    crate::c::volatile_write(
                        (dmaRegs).wrapping_offset(1),
                        (((67108928i32) as usize as *mut u16) as usize as u32),
                    );
                    crate::c::volatile_write((dmaRegs).wrapping_offset(2), 2722103297u32);
                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HBlankCB_Slice() {
    unsafe {
        if ((((67108870i32) as usize as *mut u16).read_volatile()) as i32) < 160i32 {
            let mut var: u16 = (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                .wrapping_offset(1920))
            .cast::<u16>())
            .wrapping_offset(
                ((((67108870i32) as usize as *mut u16).read_volatile()) as i32) as isize,
            ))
            .read();
            crate::c::volatile_write(((67108884i32) as usize as *mut u16), var);
            crate::c::volatile_write(((67108888i32) as usize as *mut u16), var);
            crate::c::volatile_write(((67108892i32) as usize as *mut u16), var);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShredSplit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sShredSplit_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShredSplit_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u16 = 0u16;
        InitTransitionData();
        ScanlineEffect_Clear();
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .write(63u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .write(160u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<i16>())
                        .read()) as u16),
                    );
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((160i32).wrapping_add(((i) as i32))) as isize))
                    .write(240u16);
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(20)
                            .cast::<i16>())
                        .read()) as u16),
                    );
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((160i32).wrapping_add(((i) as i32))) as isize))
                    .write(240u16);
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((320i32).wrapping_add(((i) as i32))) as isize))
                    .write(0u16);
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((480i32).wrapping_add(((i) as i32))) as isize))
                    .write(256u16);
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((640i32).wrapping_add(((i) as i32))) as isize))
                    .write(1u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(7i16);
        EnableInterrupts(2u16);
        SetVBlankCallback(Some(VBlankCB_Slice));
        SetHBlankCallback(Some(HBlankCB_Slice));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ShredSplit_Main(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut k: u16 = 0u16;
        let mut baseY = crate::ffi::Align4([0u8; 2]);
        let mut moveDirs = crate::ffi::Align4([0u8; 4]);
        let mut linesFinished: u8 = 0u8;
        let mut ptr4: *mut u16 = core::ptr::null_mut();
        let mut ptr3: *mut u16 = core::ptr::null_mut();
        let mut ptr1: *mut u16 = core::ptr::null_mut();
        let mut ptr2: *mut u16 = core::ptr::null_mut();
        let mut y: i16 = 0i16;
        crate::c::memcpy(
            (&raw mut baseY).cast::<u8>(),
            ((&raw const sShredSplit_SectionYCoords)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            2u32,
        );
        crate::c::memcpy(
            ((&raw mut moveDirs).cast::<i16>()).cast::<u8>(),
            (((&raw const sShredSplit_SectionMoveDirs)
                .cast::<u8>()
                .cast_mut()
                .cast::<i16>())
            .cast::<i16>())
            .cast::<u8>(),
            4u32,
        );
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        linesFinished = 0u8;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    <= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32))
                {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u16;
                        'l3: loop {
                            if !(((j) as i32) < 2i32) {
                                break 'l3;
                            }
                            'l4: {
                                {
                                    k = 0u16;
                                    'l5: loop {
                                        if !(((k) as i32) < 2i32) {
                                            break 'l5;
                                        }
                                        'l6: {
                                            y = (((((((&raw mut baseY).cast::<u8>())
                                                .wrapping_offset(((j) as i32) as isize))
                                            .read())
                                                as i32)
                                                .wrapping_add(
                                                    ((((((&raw mut moveDirs).cast::<i16>())
                                                        .wrapping_offset(((k) as i32) as isize))
                                                    .read())
                                                        as i32)
                                                        .wrapping_mul(((i) as i32).wrapping_neg()))
                                                    .wrapping_mul(2i32),
                                                ))
                                                as i16);
                                            if (((y) as i32) >= 0i32)
                                                && ((((y) as i32)
                                                    != (crate::c::div_i32(160i32, 2i32))
                                                        .wrapping_sub(1i32))
                                                    || (((j) as i32) != 1i32))
                                            {
                                                ptr4 = (((&raw mut gScanlineEffectRegBuffers)
                                                    .cast::<u8>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    (((y) as i32).wrapping_add(320i32)) as isize,
                                                );
                                                ptr3 = (((&raw mut gScanlineEffectRegBuffers)
                                                    .cast::<u8>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    (((y) as i32).wrapping_add(480i32)) as isize,
                                                );
                                                ptr1 = (((&raw mut gScanlineEffectRegBuffers)
                                                    .cast::<u8>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    (((y) as i32).wrapping_add(640i32)) as isize,
                                                );
                                                if (((ptr4).read()) as i32) >= 240i32 {
                                                    (ptr4).write(240u16);
                                                    linesFinished = (linesFinished).wrapping_add(1);
                                                } else {
                                                    (ptr4).write(
                                                        (((((ptr4).read()) as i32).wrapping_add(
                                                            ((((ptr3).read()) as i32) >> 8),
                                                        ))
                                                            as u16),
                                                    );
                                                    if (((ptr1).read()) as i32) <= 127i32 {
                                                        (ptr1).write(
                                                            (((((ptr1).read()) as i32)
                                                                .wrapping_mul(2i32))
                                                                as u16),
                                                        );
                                                    }
                                                    if (((ptr3).read()) as i32) <= 4095i32 {
                                                        (ptr3).write(
                                                            (((((ptr3).read()) as i32)
                                                                .wrapping_add(
                                                                    (((ptr1).read()) as i32),
                                                                ))
                                                                as u16),
                                                        );
                                                    }
                                                }
                                                ptr2 = (((&raw mut gScanlineEffectRegBuffers)
                                                    .cast::<u8>())
                                                .cast::<u16>())
                                                .wrapping_offset(((y) as i32) as isize);
                                                ptr3 = (((&raw mut gScanlineEffectRegBuffers)
                                                    .cast::<u8>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    (((y) as i32).wrapping_add(160i32)) as isize,
                                                );
                                                (ptr2).write(
                                                    ((((((((&raw mut sTransitionData)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(20)
                                                    .cast::<i16>())
                                                    .read())
                                                        as i32)
                                                        .wrapping_add((((ptr4).read()) as i32)))
                                                        as u16),
                                                );
                                                (ptr3).write(
                                                    (((240i32)
                                                        .wrapping_sub((((ptr4).read()) as i32)))
                                                        as u16),
                                                );
                                                if ((i) as i32) == 0i32 {
                                                    break 'l5;
                                                }
                                            }
                                        }
                                        k = (k).wrapping_add(1);
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    {
                        j = 0u16;
                        'l7: loop {
                            if !(((j) as i32) < 2i32) {
                                break 'l7;
                            }
                            'l8: {
                                {
                                    k = 0u16;
                                    'l9: loop {
                                        if !(((k) as i32) < 2i32) {
                                            break 'l9;
                                        }
                                        'l10: {
                                            y = ((((((((&raw mut baseY).cast::<u8>())
                                                .wrapping_offset(((j) as i32) as isize))
                                            .read())
                                                as i32)
                                                .wrapping_add(1i32))
                                            .wrapping_add(
                                                ((((((&raw mut moveDirs).cast::<i16>())
                                                    .wrapping_offset(((k) as i32) as isize))
                                                .read())
                                                    as i32)
                                                    .wrapping_mul(((i) as i32).wrapping_neg()))
                                                .wrapping_mul(2i32),
                                            ))
                                                as i16);
                                            if (((y) as i32) <= 160i32)
                                                && ((((y) as i32)
                                                    != crate::c::div_i32(160i32, 2i32))
                                                    || (((j) as i32) != 1i32))
                                            {
                                                ptr4 = (((&raw mut gScanlineEffectRegBuffers)
                                                    .cast::<u8>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    (((y) as i32).wrapping_add(320i32)) as isize,
                                                );
                                                ptr3 = (((&raw mut gScanlineEffectRegBuffers)
                                                    .cast::<u8>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    (((y) as i32).wrapping_add(480i32)) as isize,
                                                );
                                                ptr1 = (((&raw mut gScanlineEffectRegBuffers)
                                                    .cast::<u8>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    (((y) as i32).wrapping_add(640i32)) as isize,
                                                );
                                                if (((ptr4).read()) as i32) >= 240i32 {
                                                    (ptr4).write(240u16);
                                                    linesFinished = (linesFinished).wrapping_add(1);
                                                } else {
                                                    (ptr4).write(
                                                        (((((ptr4).read()) as i32).wrapping_add(
                                                            ((((ptr3).read()) as i32) >> 8),
                                                        ))
                                                            as u16),
                                                    );
                                                    if (((ptr1).read()) as i32) <= 127i32 {
                                                        (ptr1).write(
                                                            (((((ptr1).read()) as i32)
                                                                .wrapping_mul(2i32))
                                                                as u16),
                                                        );
                                                    }
                                                    if (((ptr3).read()) as i32) <= 4095i32 {
                                                        (ptr3).write(
                                                            (((((ptr3).read()) as i32)
                                                                .wrapping_add(
                                                                    (((ptr1).read()) as i32),
                                                                ))
                                                                as u16),
                                                        );
                                                    }
                                                }
                                                ptr2 = (((&raw mut gScanlineEffectRegBuffers)
                                                    .cast::<u8>())
                                                .cast::<u16>())
                                                .wrapping_offset(((y) as i32) as isize);
                                                ptr3 = (((&raw mut gScanlineEffectRegBuffers)
                                                    .cast::<u8>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    (((y) as i32).wrapping_add(160i32)) as isize,
                                                );
                                                (ptr2).write(
                                                    ((((((((&raw mut sTransitionData)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(20)
                                                    .cast::<i16>())
                                                    .read())
                                                        as i32)
                                                        .wrapping_sub((((ptr4).read()) as i32)))
                                                        as u16),
                                                );
                                                (ptr3).write(
                                                    ((((((ptr4).read()) as i32) << 8) | 241i32)
                                                        as u16),
                                                );
                                                if ((i) as i32) == 0i32 {
                                                    break 'l9;
                                                }
                                            }
                                        }
                                        k = (k).wrapping_add(1);
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            < 0i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        }
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) <= 0i32)
            && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                .wrapping_add(1i32)
                <= crate::c::div_i32(160i32, 8i32))
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read());
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if ((linesFinished) as i32) >= 160i32 {
            let __p4 = ((task).wrapping_add(8)).cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
        }
        let __p5 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p5, ((__p5).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ShredSplit_BrokenCheck(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u16 = 0u16;
        let mut done: u32 = 1u32;
        let mut checkVar2: u16 = 65296u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                        .wrapping_offset(1920))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 240i32)
                        && ((((((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            != ((checkVar2) as i32))
                    {
                        done = 0u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if done == 1u32 {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ShredSplit_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        FadeScreenBlack();
        DestroyTask(FindTaskIdByFunc(Some(Task_ShredSplit)));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_Blackhole(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sBlackhole_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_BlackholePulsate(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sBlackholePulsate_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Blackhole_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: i32 = 0i32;
        InitTransitionData();
        ScanlineEffect_Clear();
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .write(63u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<u16>())
        .write(240u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .write(160u16);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 160i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        SetVBlankCallback(Some(VBlankCB_CircularMask));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(1i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(256i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Blackhole_GrowEnd(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32) == 1i32 {
            {
                let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
                let __p1 = (dmaRegs).wrapping_offset(5);
                crate::c::volatile_write(
                    __p1,
                    (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
                );
                let __p2 = (dmaRegs).wrapping_offset(5);
                crate::c::volatile_write(
                    __p2,
                    (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
                );
                let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
            }
            SetVBlankCallback(None);
            DestroyTask(FindTaskIdByFunc(
                ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).read(),
            ));
        } else {
            crate::c::volatile_write(
                (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
                0u8,
            );
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                < 1024i32
            {
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(128i32)) as i16));
            }
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                < 160i32
            {
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_add(
                        (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            >> 8),
                    )) as i16),
                );
            }
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                > 160i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(160i16);
            }
            SetCircularMask(
                ((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>(),
                ((crate::c::div_i32(240i32, 2i32)) as i16),
                ((crate::c::div_i32(160i32, 2i32)) as i16),
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
            );
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == 160i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(1i16);
                FadeScreenBlack();
            } else {
                let __p5 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
                crate::c::volatile_write(__p5, ((__p5).read_volatile()).wrapping_add(1));
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Blackhole_Vibrate(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32) == 0i32 {
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(48i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        }
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((&raw const sBlackhole_Vibrations)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<i16>())
                .cast::<i16>())
                .wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                        as isize,
                ))
                .read()) as i32),
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((crate::c::rem_i32(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    .wrapping_add(1i32),
                ((crate::c::div_u32(4u32, 2u32)) as i32),
            )) as i16),
        );
        SetCircularMask(
            ((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>(),
            ((crate::c::div_i32(240i32, 2i32)) as i16),
            ((crate::c::div_i32(160i32, 2i32)) as i16),
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) < 9i32 {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        let __p4 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p4, ((__p4).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn BlackholePulsate_Main(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut index: u16 = 0u16;
        let mut amplitude: i16 = 0i16;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32) == 0i32 {
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(2i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(2i16);
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) > 160i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(160i16);
        }
        SetCircularMask(
            ((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>(),
            ((crate::c::div_i32(240i32, 2i32)) as i16),
            ((crate::c::div_i32(160i32, 2i32)) as i16),
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read(),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 160i32
        {
            {
                let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
                let __p2 = (dmaRegs).wrapping_offset(5);
                crate::c::volatile_write(
                    __p2,
                    (((((__p2).read_volatile()) as i32) & (-14849i32)) as u16),
                );
                let __p3 = (dmaRegs).wrapping_offset(5);
                crate::c::volatile_write(
                    __p3,
                    (((((__p3).read_volatile()) as i32) & (-32769i32)) as u16),
                );
                let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
            }
            FadeScreenBlack();
            DestroyTask(FindTaskIdByFunc(
                ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).read(),
            ));
        }
        index = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as u16);
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            & 255i32)
            <= 128i32
        {
            amplitude = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read();
            let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
            (__p4).write((((((__p4).read()) as i32).wrapping_add(8i32)) as i16));
        } else {
            amplitude = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                as i32)
                .wrapping_sub(1i32)) as i16);
            let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
            (__p5).write((((((__p5).read()) as i32).wrapping_add(16i32)) as i16));
        }
        let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p6).write(
            (((((__p6).read()) as i32)
                .wrapping_add(((Sin(((((index) as i32) & 255i32) as i16), amplitude)) as i32)))
                as i16),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) <= 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(1i16);
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32) >= 255i32
        {
            let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
            (__p7).write((((((__p7).read()) as i32) >> 8) as i16));
            let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
            (__p8).write(((__p8).read()).wrapping_add(1));
        }
        let __p9 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p9, ((__p9).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_RectangularSpiral(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sRectangularSpiral_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RectangularSpiral_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (((&raw const sShrinkingBoxTileset)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .cast::<u8>(),
                            (tileset).cast::<u8>(),
                            ((0i32
                                | (crate::c::div_i32(32i32, crate::c::div_i32(16i32, 8i32))
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
        'l5: loop {
            'l6: {
                'l7: loop {
                    'l8: {
                        CpuSet(
                            ((((&raw const sShrinkingBoxTileset)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .wrapping_offset(112))
                            .cast::<u8>(),
                            ((tileset).wrapping_offset(32)).cast::<u8>(),
                            ((0i32
                                | (crate::c::div_i32(32i32, crate::c::div_i32(16i32, 8i32))
                                    & 2097151i32)) as u32),
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
        'l9: loop {
            'l10: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(61440u16);
                    'l11: loop {
                        'l12: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (tilemap).cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l11;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l9;
            }
        }
        LoadPalette(
            (((&raw const sFieldEffectPal_Pokeball)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(1i16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        (((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).write(0u8);
        ((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>())
            .wrapping_add(2)
            .cast::<i16>())
        .write((-1i16));
        ((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_add(4))
            .write(1u8);
        ((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>())
            .wrapping_add(6)
            .cast::<i16>())
        .write(308i16);
        ((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_add(8))
            .write(0u8);
        ((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_offset(12))
            .write(0u8);
        (((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_offset(12))
            .wrapping_add(2)
            .cast::<i16>())
        .write((-1i16));
        (((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_offset(12))
            .wrapping_add(4))
        .write(1u8);
        (((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_offset(12))
            .wrapping_add(6)
            .cast::<i16>())
        .write(308i16);
        (((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_offset(12))
            .wrapping_add(8))
        .write(0u8);
        ((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_offset(24))
            .write(0u8);
        (((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_offset(24))
            .wrapping_add(2)
            .cast::<i16>())
        .write((-3i16));
        (((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_offset(24))
            .wrapping_add(4))
        .write(1u8);
        (((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_offset(24))
            .wrapping_add(6)
            .cast::<i16>())
        .write(307i16);
        (((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_offset(24))
            .wrapping_add(8))
        .write(0u8);
        ((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_offset(36))
            .write(0u8);
        (((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_offset(36))
            .wrapping_add(2)
            .cast::<i16>())
        .write((-3i16));
        (((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_offset(36))
            .wrapping_add(4))
        .write(1u8);
        (((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_offset(36))
            .wrapping_add(6)
            .cast::<i16>())
        .write(307i16);
        (((((&raw mut sRectangularSpiralLines).cast::<u8>()).cast::<u8>()).wrapping_offset(36))
            .wrapping_add(8))
        .write(0u8);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn RectangularSpiral_Main(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        let mut j: u16 = 0u16;
        let mut done: u32 = 1u32;
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u16;
                        'l3: loop {
                            if !(((j) as u32) < crate::c::div_u32(48u32, 12u32)) {
                                break 'l3;
                            }
                            'l4: {
                                let mut position: i16 = 0i16;
                                let mut x: i16 = 0i16;
                                let mut y: i16 = 0i16;
                                if (UpdateRectangularSpiralLine(
                                    ((((&raw const sRectangularSpiral_MoveDataTables)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<*mut *mut i16>())
                                    .cast::<*mut *mut i16>())
                                    .wrapping_offset(
                                        (crate::c::div_i32(((j) as i32), 2i32)) as isize,
                                    ))
                                    .read(),
                                    (((&raw mut sRectangularSpiralLines).cast::<u8>())
                                        .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 12),
                                )) != 0
                                {
                                    done = 0u32;
                                    position = (((((&raw mut sRectangularSpiralLines)
                                        .cast::<u8>())
                                    .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 12))
                                    .wrapping_add(2)
                                    .cast::<i16>())
                                    .read();
                                    if crate::c::rem_i32(((j) as i32), 2i32) == 1i32 {
                                        position =
                                            (((637i32).wrapping_sub(((position) as i32))) as i16);
                                    }
                                    x = ((crate::c::rem_i32(((position) as i32), 32i32)) as i16);
                                    y = ((crate::c::div_i32(((position) as i32), 32i32)) as i16);
                                    {
                                        let mut index: u32 = (((((y) as i32).wrapping_mul(32i32))
                                            .wrapping_add(((x) as i32)))
                                            as u32);
                                        ((tilemap).wrapping_offset(((index) as i32) as isize))
                                            .write(61442u16);
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if done == 1u32 {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn RectangularSpiral_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        FadeScreenBlack();
        DestroyTask(FindTaskIdByFunc(
            ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).read(),
        ));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn UpdateRectangularSpiralLine(
    moveDataTable: *mut *mut i16,
    line: *mut u8,
) -> u16 {
    unsafe {
        let mut moveDataTable = moveDataTable;
        let mut line = line;
        let mut moveData: *mut i16 =
            ((moveDataTable).wrapping_offset((((line).read()) as i32) as isize)).read();
        if ((((moveData).wrapping_offset(((((line).wrapping_add(4)).read()) as i32) as isize))
            .read()) as i32)
            == (-1i32)
        {
            return 0u16;
        }
        ((&raw mut sDebug_RectangularSpiralData)
            .cast::<u8>()
            .cast::<i16>())
        .write((moveData).read());
        ((&raw mut sDebug_RectangularSpiralData)
            .cast::<u8>()
            .cast::<i16>())
        .write(((moveData).wrapping_offset(1)).read());
        ((&raw mut sDebug_RectangularSpiralData)
            .cast::<u8>()
            .cast::<i16>())
        .write(((moveData).wrapping_offset(2)).read());
        ((&raw mut sDebug_RectangularSpiralData)
            .cast::<u8>()
            .cast::<i16>())
        .write(((moveData).wrapping_offset(3)).read());
        'l1: {
            let __sw1 = (((moveData).read()) as i32);
            if __sw1 == 1i32 {
                let __p2 = (line).wrapping_add(2).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add(1i32)) as i16));
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p3 = (line).wrapping_add(2).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(1i32)) as i16));
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p4 = (line).wrapping_add(2).cast::<i16>();
                (__p4).write((((((__p4).read()) as i32).wrapping_sub(32i32)) as i16));
                break 'l1;
            }
            if __sw1 == 4i32 {
                let __p5 = (line).wrapping_add(2).cast::<i16>();
                (__p5).write((((((__p5).read()) as i32).wrapping_add(32i32)) as i16));
                break 'l1;
            }
        }
        if (((((line).wrapping_add(2).cast::<i16>()).read()) as i32) >= 640i32)
            || (((((moveData).wrapping_offset(((((line).wrapping_add(4)).read()) as i32) as isize))
                .read()) as i32)
                == (-1i32))
        {
            return 0u16;
        }
        if (!((((line).wrapping_add(8)).read()) != 0))
            && (((((moveData).wrapping_offset(((((line).wrapping_add(4)).read()) as i32) as isize))
                .read()) as i32)
                == (-2i32))
        {
            ((line).wrapping_add(8)).write(1u8);
            ((line).wrapping_add(4)).write(1u8);
            ((line).wrapping_add(2).cast::<i16>())
                .write(((line).wrapping_add(6).cast::<i16>()).read());
            (line).write(4u8);
        }
        if ((((line).wrapping_add(2).cast::<i16>()).read()) as i32)
            == ((((moveData).wrapping_offset(((((line).wrapping_add(4)).read()) as i32) as isize))
                .read()) as i32)
        {
            let __p6 = (line);
            (__p6).write(((__p6).read()).wrapping_add(1));
            if ((((line).wrapping_add(8)).read()) as i32) == 1i32 {
                if (((line).read()) as i32) > 7i32 {
                    let __p7 = (line).wrapping_add(4);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    (line).write(4u8);
                }
            } else {
                if (((line).read()) as i32) > 3i32 {
                    let __p8 = (line).wrapping_add(4);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                    (line).write(0u8);
                }
            }
        }
        return 1u16;
    }
}
pub(crate) unsafe extern "C" fn Task_Groudon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sGroudon_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Groudon_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (tilemap).cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
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
        LZ77UnCompVram(
            ((&raw const sGroudon_Tileset)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tileset).cast::<u8>(),
        );
        LZ77UnCompVram(
            ((&raw const sGroudon_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tilemap).cast::<u8>(),
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Groudon_PaletteFlash(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if crate::c::rem_i32(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            3i32,
        ) == 0i32
        {
            let mut offset: u16 = ((crate::c::div_i32(
                crate::c::rem_i32(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
                    30i32,
                ),
                3i32,
            )) as u16);
            LoadPalette(
                ((((&raw const sGroudon1_Palette)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset((((offset) as i32).wrapping_mul(16i32)) as isize))
                .cast::<u8>(),
                240u16,
                32u16,
            );
        }
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 58i32
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Groudon_PaletteBrighten(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if crate::c::rem_i32(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            5i32,
        ) == 0i32
        {
            let mut offset: i16 = ((crate::c::div_i32(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
                5i32,
            )) as i16);
            LoadPalette(
                ((((&raw const sGroudon2_Palette)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset((((offset) as i32).wrapping_mul(16i32)) as isize))
                .cast::<u8>(),
                240u16,
                32u16,
            );
        }
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 68i32
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(30i16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_Rayquaza(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sRayquaza_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Rayquaza_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        let mut i: u16 = 0u16;
        InitTransitionData();
        ScanlineEffect_Clear();
        SetGpuReg(8u8, 39432u16);
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (tilemap).cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
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
        'l5: loop {
            'l6: {
                'l7: loop {
                    'l8: {
                        CpuSet(
                            (((&raw const sRayquaza_Tileset)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .cast::<u8>(),
                            (tileset).cast::<u8>(),
                            ((0i32
                                | (crate::c::div_i32(8192i32, crate::c::div_i32(16i32, 8i32))
                                    & 2097151i32)) as u32),
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
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<i16>())
        .write(0i16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        LoadPalette(
            ((((&raw const sRayquaza_Palette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(80))
            .cast::<u8>(),
            240u16,
            32u16,
        );
        {
            i = 0u16;
            'l9: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l9;
                }
                'l10: {
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(256u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        SetVBlankCallback(Some(VBlankCB_Rayquaza));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Rayquaza_SetGfx(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (((&raw const sRayquaza_Tilemap)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .cast::<u8>(),
                            (tilemap).cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    4096u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
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
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Rayquaza_PaletteFlash(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if crate::c::rem_i32(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            4i32,
        ) == 0i32
        {
            let mut value: u16 = ((crate::c::div_i32(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
                4i32,
            )) as u16);
            let mut palPtr: *mut u16 = (((&raw const sRayquaza_Palette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((((value) as i32).wrapping_add(5i32)).wrapping_mul(16i32)) as isize);
            LoadPalette((palPtr).cast::<u8>(), 240u16, 32u16);
        }
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 40i32
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Rayquaza_FadeToBlack(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 20i32
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            BeginNormalPaletteFade(4294934528u32, 2i8, 0u8, 16u8, 0u16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Rayquaza_WaitFade(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<i16>())
            .write(1i16);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Rayquaza_SetBlack(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        BlendPalettes(32767u32, 8u8, 0u16);
        BlendPalettes(4294934528u32, 0u8, 0u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Rayquaza_TriRing(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if crate::c::rem_i32(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            3i32,
        ) == 0i32
        {
            let mut value: u16 = ((crate::c::div_i32(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
                3i32,
            )) as u16);
            let mut palPtr: *mut u16 = (((&raw const sRayquaza_Palette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((((value) as i32).wrapping_add(0i32)).wrapping_mul(16i32)) as isize);
            LoadPalette((palPtr).cast::<u8>(), 240u16, 32u16);
        }
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 40i32
        {
            let mut i: u16 = 0u16;
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .write(63u16);
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<u16>())
            .write(240u16);
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .write(160u16);
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 160i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(0u16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            SetVBlankCallback(Some(VBlankCB_CircularMask));
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(256i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            ClearGpuRegBits(0u8, 256u16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_Rayquaza() {
    unsafe {
        let mut dmaSrc: *mut u8 = core::ptr::null_mut();
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        VBlankCB_BattleTransition();
        if ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            dmaSrc =
                (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).cast::<u8>();
        } else {
            if ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<i16>())
            .read()) as i32)
                == 1i32
            {
                dmaSrc = ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                    .wrapping_offset(1920))
                .cast::<u16>())
                .cast::<u8>();
            } else {
                dmaSrc = (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                    .cast::<u8>();
            }
        }
        'l1: loop {
            'l2: {
                {
                    let mut dmaRegs: *mut u32 = ((67109040i32) as usize as *mut u32);
                    crate::c::volatile_write(dmaRegs, ((dmaSrc) as usize as u32));
                    crate::c::volatile_write(
                        (dmaRegs).wrapping_offset(1),
                        (((67108882i32) as usize as *mut u16) as usize as u32),
                    );
                    crate::c::volatile_write((dmaRegs).wrapping_offset(2), 2722103297u32);
                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WhiteBarsFade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sWhiteBarsFade_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn WhiteBarsFade_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u16 = 0u16;
        InitTransitionData();
        ScanlineEffect_Clear();
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14)
            .cast::<u16>())
        .write(191u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(18)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .write(30u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .write(63u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .write(160u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset((((i) as i32).wrapping_add(160i32)) as isize))
                    .write(240u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        EnableInterrupts(2u16);
        SetHBlankCallback(Some(HBlankCB_WhiteBarsFade));
        SetVBlankCallback(Some(VBlankCB_WhiteBarsFade));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WhiteBarsFade_StartBars(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: i16 = 0i16;
        let mut posY: i16 = 0i16;
        let mut delays = crate::ffi::Align4([0u8; 16]);
        let mut sprite: *mut u8 = core::ptr::null_mut();
        crate::c::memcpy(
            ((&raw mut delays).cast::<i16>()).cast::<u8>(),
            (((&raw const sWhiteBarsFade_StartDelays)
                .cast::<u8>()
                .cast_mut()
                .cast::<i16>())
            .cast::<i16>())
            .cast::<u8>(),
            16u32,
        );
        {
            i = 0i16;
            posY = 0i16;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    sprite = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((CreateInvisibleSprite(Some(SpriteCB_WhiteBarFade))) as i32) as isize * 68,
                    );
                    ((sprite).wrapping_add(32).cast::<i16>()).write(240i16);
                    ((sprite).wrapping_add(34).cast::<i16>()).write(posY);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                        (((&raw mut delays).cast::<i16>()).wrapping_offset(((i) as i32) as isize))
                            .read(),
                    );
                }
                i = (i).wrapping_add(1);
                posY = ((((posY) as i32).wrapping_add(crate::c::div_i32(160i32, 8i32))) as i16);
            }
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = ((task).wrapping_add(8)).cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WhiteBarsFade_WaitBars(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        if ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<i16>())
        .read()) as i32)
            >= 8i32
        {
            BlendPalettes(4294967295u32, 16u8, 32767u16);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WhiteBarsFade_BlendToBlack(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        SetVBlankCallback(None);
        SetHBlankCallback(None);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6)
            .cast::<u16>())
        .write(240u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(18)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14)
            .cast::<u16>())
        .write(255u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .write(63u16);
        SetVBlankCallback(Some(VBlankCB_WhiteBarsFade_Blend));
        let __p3 = ((task).wrapping_add(8)).cast::<i16>();
        (__p3).write(((__p3).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WhiteBarsFade_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (({
            let __p1 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<u16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 16i32
        {
            FadeScreenBlack();
            DestroyTask(FindTaskIdByFunc(Some(Task_WhiteBarsFade)));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_WhiteBarsFade() {
    unsafe {
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        VBlankCB_BattleTransition();
        crate::c::volatile_write(
            ((67108944i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108936i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108938i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108932i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read(),
        );
        if ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).read_volatile())
            != 0
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(
                                    dmaRegs,
                                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .cast::<u16>())
                                        as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .wrapping_offset(1920))
                                    .cast::<u16>()) as usize
                                        as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2147483648i32)
                                        | crate::c::div_i32(640i32, crate::c::div_i32(16i32, 8i32)))
                                        as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
        'l5: loop {
            'l6: {
                {
                    let mut dmaRegs: *mut u32 = ((67109040i32) as usize as *mut u32);
                    crate::c::volatile_write(
                        dmaRegs,
                        ((((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>())
                        .wrapping_offset(160)) as usize as u32),
                    );
                    crate::c::volatile_write(
                        (dmaRegs).wrapping_offset(1),
                        (((67108928i32) as usize as *mut u16) as usize as u32),
                    );
                    crate::c::volatile_write((dmaRegs).wrapping_offset(2), 2722103297u32);
                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_WhiteBarsFade_Blend() {
    unsafe {
        VBlankCB_BattleTransition();
        crate::c::volatile_write(
            ((67108948i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108944i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108936i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108938i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108928i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108932i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn HBlankCB_WhiteBarsFade() {
    unsafe {
        crate::c::volatile_write(
            ((67108948i32) as usize as *mut u16),
            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                .cast::<u16>())
            .wrapping_offset(
                ((((67108870i32) as usize as *mut u16).read_volatile()) as i32) as isize,
            ))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_WhiteBarFade(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) != 0 {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) != 0 {
                crate::c::volatile_write(
                    (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
                    1u8,
                );
            }
        } else {
            let mut i: u16 = 0u16;
            let mut ptr1: *mut u16 = (((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                .cast::<u16>())
            .wrapping_offset(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) as isize);
            let mut ptr2: *mut u16 = (((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                .cast::<u16>())
            .wrapping_offset(
                (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(160i32))
                    as isize,
            );
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < crate::c::div_i32(160i32, 8i32)) {
                        break 'l1;
                    }
                    'l2: {
                        ((ptr1).wrapping_offset(((i) as i32) as isize)).write(
                            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 8)
                                as u16),
                        );
                        ((ptr2).wrapping_offset(((i) as i32) as isize)).write(
                            (((((sprite).wrapping_add(32).cast::<i16>()).read()) as u8) as u16),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) == 0i32)
                && ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 4096i32)
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(1i16);
            }
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_sub(16i32)) as i16));
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(crate::c::div_i32(4096i32, 32i32))) as i16),
            );
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < 0i32 {
                ((sprite).wrapping_add(32).cast::<i16>()).write(0i16);
            }
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 4096i32 {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(4096i16);
            }
            if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) != 0 {
                crate::c::volatile_write(
                    (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
                    1u8,
                );
            }
            if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0 {
                if (!((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    != 0))
                    || ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32)
                        .cast::<i16>())
                    .read()) as i32)
                        >= 7i32)
                        && ((({
                            let __p4 =
                                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                            let __t5 = (__p4).read();
                            (__p4).write(((__p4).read()).wrapping_add(1));
                            __t5
                        }) as i32)
                            > 7i32))
                {
                    let __p6 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32)
                        .cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    DestroySprite(sprite);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_GridSquares(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sGridSquares_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GridSquares_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        'l1: loop {
            'l2: {
                CpuSet(
                    (((&raw const sShrinkingBoxTileset)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    (tileset).cast::<u8>(),
                    16u32,
                );
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l3: loop {
            'l4: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(61440u16);
                    'l5: loop {
                        'l6: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (tilemap).cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
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
        LoadPalette(
            (((&raw const sFieldEffectPal_Pokeball)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GridSquares_Main(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tileset: *mut u16 = core::ptr::null_mut();
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32 {
            GetBg0TilemapDst(&raw mut tileset);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(3i16);
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            'l1: loop {
                'l2: {
                    CpuSet(((((&raw const sShrinkingBoxTileset).cast::<u8>().cast_mut().cast::<u32>()).cast::<u32>()).wrapping_offset((((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32))).wrapping_mul(8i32)) as isize)).cast::<u8>(), (tileset).cast::<u8>(), 16u32);
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 13i32
            {
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(16i16);
            }
        }
        let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p3).write(((__p3).read()).wrapping_sub(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GridSquares_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            FadeScreenBlack();
            DestroyTask(FindTaskIdByFunc(Some(Task_GridSquares)));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_AngledWipes(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sAngledWipes_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AngledWipes_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u16 = 0u16;
        InitTransitionData();
        ScanlineEffect_Clear();
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<u16>())
        .write(63u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8)
            .cast::<u16>())
        .write(160u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(240u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        'l3: loop {
            'l4: {
                CpuSet(
                    (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .cast::<u8>(),
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .cast::<u8>(),
                    160u32,
                );
            }
            if !((0i32) != 0) {
                break 'l3;
            }
        }
        SetVBlankCallback(Some(VBlankCB_AngledWipes));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn AngledWipes_SetWipeData(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        InitBlackWipe(
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(36))
                .cast::<i16>(),
            (((((&raw const sAngledWipes_MoveData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                        as isize
                        * 10,
                ))
            .cast::<i16>())
            .read(),
            ((((((&raw const sAngledWipes_MoveData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                        as isize
                        * 10,
                ))
            .cast::<i16>())
            .wrapping_offset(1))
            .read(),
            ((((((&raw const sAngledWipes_MoveData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                        as isize
                        * 10,
                ))
            .cast::<i16>())
            .wrapping_offset(2))
            .read(),
            ((((((&raw const sAngledWipes_MoveData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                        as isize
                        * 10,
                ))
            .cast::<i16>())
            .wrapping_offset(3))
            .read(),
            1i16,
            1i16,
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
            ((((((&raw const sAngledWipes_MoveData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                        as isize
                        * 10,
                ))
            .cast::<i16>())
            .wrapping_offset(4))
            .read(),
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn AngledWipes_DoWipe(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: i16 = 0i16;
        let mut finished: u8 = 0u8;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        {
            i = 0i16;
            finished = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    let mut r3: i16 = ((((((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                        .cast::<u16>())
                    .wrapping_offset(
                        ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(36))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        >> 8) as i16);
                    let mut r4: i16 = ((((((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                        .cast::<u16>())
                    .wrapping_offset(
                        ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(36))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        & 255i32) as i16);
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        == 0i32
                    {
                        if ((r3) as i32)
                            < ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(36))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .read()) as i32)
                        {
                            r3 =
                                ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(36))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read();
                        }
                        if ((r3) as i32) > ((r4) as i32) {
                            r3 = r4;
                        }
                    } else {
                        if ((r4) as i32)
                            > ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(36))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .read()) as i32)
                        {
                            r4 =
                                ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(36))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read();
                        }
                        if ((r4) as i32) <= ((r3) as i32) {
                            r4 = r3;
                        }
                    }
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(
                            ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(36))
                            .cast::<i16>())
                            .wrapping_offset(3))
                            .read()) as i32) as isize,
                        ))
                    .write(((((r4) as i32) | (((r3) as i32) << 8)) as u16));
                    if (finished) != 0 {
                        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p1).write(((__p1).read()).wrapping_add(1));
                        break 'l1;
                    }
                    finished = UpdateBlackWipe(
                        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(36))
                        .cast::<i16>(),
                        1u8,
                        1u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p2 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p2, ((__p2).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn AngledWipes_TryEnd(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            < 7i32
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
                ((((&raw const sAngledWipes_EndDelays)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<i16>())
                .cast::<i16>())
                .wrapping_offset(
                    (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        .wrapping_sub(1i32)) as isize,
                ))
                .read(),
            );
            return 1u8;
        } else {
            {
                let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
                let __p4 = (dmaRegs).wrapping_offset(5);
                crate::c::volatile_write(
                    __p4,
                    (((((__p4).read_volatile()) as i32) & (-14849i32)) as u16),
                );
                let __p5 = (dmaRegs).wrapping_offset(5);
                crate::c::volatile_write(
                    __p5,
                    (((((__p5).read_volatile()) as i32) & (-32769i32)) as u16),
                );
                let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
            }
            FadeScreenBlack();
            DestroyTask(FindTaskIdByFunc(Some(Task_AngledWipes)));
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn AngledWipes_StartNext(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_AngledWipes() {
    unsafe {
        {
            let mut dmaRegs: *mut u16 = ((67109040i32) as usize as *mut u16);
            let __p1 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-14849i32)) as u16),
            );
            let __p2 = (dmaRegs).wrapping_offset(5);
            crate::c::volatile_write(
                __p2,
                (((((__p2).read_volatile()) as i32) & (-32769i32)) as u16),
            );
            let _ = ((dmaRegs).wrapping_offset(5)).read_volatile();
        }
        VBlankCB_BattleTransition();
        if ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).read_volatile())
            != 0
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(
                                    dmaRegs,
                                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .cast::<u16>())
                                        as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .wrapping_offset(1920))
                                    .cast::<u16>()) as usize
                                        as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2147483648i32)
                                        | crate::c::div_i32(320i32, crate::c::div_i32(16i32, 8i32)))
                                        as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
        crate::c::volatile_write(
            ((67108936i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108938i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108932i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108928i32) as usize as *mut u16),
            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                .cast::<u16>())
            .read(),
        );
        'l5: loop {
            'l6: {
                {
                    let mut dmaRegs: *mut u32 = ((67109040i32) as usize as *mut u32);
                    crate::c::volatile_write(
                        dmaRegs,
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>()) as usize as u32),
                    );
                    crate::c::volatile_write(
                        (dmaRegs).wrapping_offset(1),
                        (((67108928i32) as usize as *mut u16) as usize as u32),
                    );
                    crate::c::volatile_write((dmaRegs).wrapping_offset(2), 2722103297u32);
                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateIntroTask(
    fadeToGrayDelay: i16,
    fadeFromGrayDelay: i16,
    numFades: i16,
    fadeToGrayIncrement: i16,
    fadeFromGrayIncrement: i16,
) {
    unsafe {
        let mut fadeToGrayDelay = fadeToGrayDelay;
        let mut fadeFromGrayDelay = fadeFromGrayDelay;
        let mut numFades = numFades;
        let mut fadeToGrayIncrement = fadeToGrayIncrement;
        let mut fadeFromGrayIncrement = fadeFromGrayIncrement;
        let mut taskId: u8 = CreateTask(Some(Task_BattleTransition_Intro), 3u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(fadeToGrayDelay);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(fadeFromGrayDelay);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(numFades);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(fadeToGrayIncrement);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(fadeFromGrayIncrement);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(fadeToGrayDelay);
    }
}
pub(crate) unsafe extern "C" fn IsIntroTaskDone() -> u8 {
    unsafe {
        if ((FindTaskIdByFunc(Some(Task_BattleTransition_Intro))) as i32) == 255i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_BattleTransition_Intro(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sTransitionIntroFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TransitionIntro_FadeToGray(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32) == 0i32)
            || ((({
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32)
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read());
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
                )) as i16),
            );
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                > 16i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(16i16);
            }
            BlendPalettes(
                4294967295u32,
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as u8),
                11627u16,
            );
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32) >= 16i32
        {
            let __p4 = ((task).wrapping_add(8)).cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read());
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn TransitionIntro_FadeFromGray(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32) == 0i32)
            || ((({
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32)
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read());
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_sub(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
                )) as i16),
            );
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                < 0i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            }
            BlendPalettes(
                4294967295u32,
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as u8),
                11627u16,
            );
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32) == 0i32 {
            if (({
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                let __t5 = ((__p4).read()).wrapping_sub(1);
                (__p4).write(__t5);
                __t5
            }) as i32)
                == 0i32
            {
                DestroyTask(FindTaskIdByFunc(Some(Task_BattleTransition_Intro)));
            } else {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read());
                (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn InitTransitionData() {
    unsafe {
        crate::c::memset(
            ((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read(),
            0i32,
            60u32,
        );
        GetCameraOffsetWithPan(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20)
                .cast::<i16>(),
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(22)
                .cast::<i16>(),
        );
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_BattleTransition() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn GetBg0TilemapDst(tileset: *mut *mut u16) {
    unsafe {
        let mut tileset = tileset;
        let mut charBase: u16 =
            ((((((67108872i32) as usize as *mut u16).read_volatile()) as i32) >> 2) as u16);
        charBase = ((((charBase) as i32) << 14) as u16);
        (tileset).write((((100663296i32).wrapping_add(((charBase) as i32))) as usize as *mut u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBg0TilesDst(tilemap: *mut *mut u16, tileset: *mut *mut u16) {
    unsafe {
        let mut tilemap = tilemap;
        let mut tileset = tileset;
        let mut screenBase: u16 =
            ((((((67108872i32) as usize as *mut u16).read_volatile()) as i32) >> 8) as u16);
        let mut charBase: u16 =
            ((((((67108872i32) as usize as *mut u16).read_volatile()) as i32) >> 2) as u16);
        screenBase = ((((screenBase) as i32) << 11) as u16);
        charBase = ((((charBase) as i32) << 14) as u16);
        (tilemap)
            .write((((100663296i32).wrapping_add(((screenBase) as i32))) as usize as *mut u16));
        (tileset).write((((100663296i32).wrapping_add(((charBase) as i32))) as usize as *mut u16));
    }
}
pub(crate) unsafe extern "C" fn FadeScreenBlack() {
    unsafe {
        BlendPalettes(4294967295u32, 16u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn SetSinWave(
    array: *mut i16,
    sinAdd: i16,
    index: i16,
    indexIncrementer: i16,
    amplitude: i16,
    arrSize: i16,
) {
    unsafe {
        let mut array = array;
        let mut sinAdd = sinAdd;
        let mut index = index;
        let mut indexIncrementer = indexIncrementer;
        let mut amplitude = amplitude;
        let mut arrSize = arrSize;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((arrSize) as i32) > 0i32) {
                    break 'l1;
                }
                'l2: {
                    ((array).wrapping_offset(((i) as i32) as isize)).write(
                        ((((sinAdd) as i32).wrapping_add(
                            ((Sin(((((index) as i32) & 255i32) as i16), amplitude)) as i32),
                        )) as i16),
                    );
                }
                arrSize = (arrSize).wrapping_sub(1);
                i = (i).wrapping_add(1);
                index = ((((index) as i32).wrapping_add(((indexIncrementer) as i32))) as i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetCircularMask(
    buffer: *mut u16,
    centerX: i16,
    centerY: i16,
    radius: i16,
) {
    unsafe {
        let mut buffer = buffer;
        let mut centerX = centerX;
        let mut centerY = centerY;
        let mut radius = radius;
        let mut i: i16 = 0i16;
        crate::c::memset((buffer).cast::<u8>(), 10i32, 320u32);
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) < 64i32) {
                    break 'l1;
                }
                'l2: {
                    let mut sinResult: i16 = 0i16;
                    let mut cosResult: i16 = 0i16;
                    let mut drawXLeft: i16 = 0i16;
                    let mut drawYBottNext: i16 = 0i16;
                    let mut drawYTopNext: i16 = 0i16;
                    let mut drawX: i16 = 0i16;
                    let mut drawYTop: i16 = 0i16;
                    let mut drawYBott: i16 = 0i16;
                    sinResult = Sin(i, radius);
                    cosResult = Cos(i, radius);
                    drawXLeft = ((((centerX) as i32).wrapping_sub(((sinResult) as i32))) as i16);
                    drawX = ((((centerX) as i32).wrapping_add(((sinResult) as i32))) as i16);
                    drawYTop = ((((centerY) as i32).wrapping_sub(((cosResult) as i32))) as i16);
                    drawYBott = ((((centerY) as i32).wrapping_add(((cosResult) as i32))) as i16);
                    if ((drawXLeft) as i32) < 0i32 {
                        drawXLeft = 0i16;
                    }
                    if ((drawX) as i32) > 240i32 {
                        drawX = 240i16;
                    }
                    if ((drawYTop) as i32) < 0i32 {
                        drawYTop = 0i16;
                    }
                    if ((drawYBott) as i32) > 159i32 {
                        drawYBott = 159i16;
                    }
                    drawX = ((((drawX) as i32) | (((drawXLeft) as i32) << 8)) as i16);
                    ((buffer).wrapping_offset(((drawYTop) as i32) as isize))
                        .write(((drawX) as u16));
                    ((buffer).wrapping_offset(((drawYBott) as i32) as isize))
                        .write(((drawX) as u16));
                    cosResult = Cos(((((i) as i32).wrapping_add(1i32)) as i16), radius);
                    drawYTopNext = ((((centerY) as i32).wrapping_sub(((cosResult) as i32))) as i16);
                    drawYBottNext =
                        ((((centerY) as i32).wrapping_add(((cosResult) as i32))) as i16);
                    if ((drawYTopNext) as i32) < 0i32 {
                        drawYTopNext = 0i16;
                    }
                    if ((drawYBottNext) as i32) > 159i32 {
                        drawYBottNext = 159i16;
                    }
                    'l3: loop {
                        if !(((drawYTop) as i32) > ((drawYTopNext) as i32)) {
                            break 'l3;
                        }
                        ((buffer).wrapping_offset(
                            (({
                                let __t1 = (drawYTop).wrapping_sub(1);
                                drawYTop = __t1;
                                __t1
                            }) as i32) as isize,
                        ))
                        .write(((drawX) as u16));
                    }
                    'l4: loop {
                        if !(((drawYTop) as i32) < ((drawYTopNext) as i32)) {
                            break 'l4;
                        }
                        ((buffer).wrapping_offset(
                            (({
                                let __t2 = (drawYTop).wrapping_add(1);
                                drawYTop = __t2;
                                __t2
                            }) as i32) as isize,
                        ))
                        .write(((drawX) as u16));
                    }
                    'l5: loop {
                        if !(((drawYBott) as i32) > ((drawYBottNext) as i32)) {
                            break 'l5;
                        }
                        ((buffer).wrapping_offset(
                            (({
                                let __t3 = (drawYBott).wrapping_sub(1);
                                drawYBott = __t3;
                                __t3
                            }) as i32) as isize,
                        ))
                        .write(((drawX) as u16));
                    }
                    'l6: loop {
                        if !(((drawYBott) as i32) < ((drawYBottNext) as i32)) {
                            break 'l6;
                        }
                        ((buffer).wrapping_offset(
                            (({
                                let __t4 = (drawYBott).wrapping_add(1);
                                drawYBott = __t4;
                                __t4
                            }) as i32) as isize,
                        ))
                        .write(((drawX) as u16));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitBlackWipe(
    data: *mut i16,
    startX: i16,
    startY: i16,
    endX: i16,
    endY: i16,
    xMove: i16,
    yMove: i16,
) {
    unsafe {
        let mut data = data;
        let mut startX = startX;
        let mut startY = startY;
        let mut endX = endX;
        let mut endY = endY;
        let mut xMove = xMove;
        let mut yMove = yMove;
        (data).write(startX);
        ((data).wrapping_offset(1)).write(startY);
        ((data).wrapping_offset(2)).write(startX);
        ((data).wrapping_offset(3)).write(startY);
        ((data).wrapping_offset(4)).write(endX);
        ((data).wrapping_offset(5)).write(endY);
        ((data).wrapping_offset(6)).write(xMove);
        ((data).wrapping_offset(7)).write(yMove);
        ((data).wrapping_offset(8))
            .write(((((endX) as i32).wrapping_sub(((startX) as i32))) as i16));
        if ((((data).wrapping_offset(8)).read()) as i32) < 0i32 {
            ((data).wrapping_offset(8))
                .write(((((((data).wrapping_offset(8)).read()) as i32).wrapping_neg()) as i16));
            ((data).wrapping_offset(6)).write(((((xMove) as i32).wrapping_neg()) as i16));
        }
        ((data).wrapping_offset(9))
            .write(((((endY) as i32).wrapping_sub(((startY) as i32))) as i16));
        if ((((data).wrapping_offset(9)).read()) as i32) < 0i32 {
            ((data).wrapping_offset(9))
                .write(((((((data).wrapping_offset(9)).read()) as i32).wrapping_neg()) as i16));
            ((data).wrapping_offset(7)).write(((((yMove) as i32).wrapping_neg()) as i16));
        }
        ((data).wrapping_offset(10)).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn UpdateBlackWipe(data: *mut i16, xExact: u8, yExact: u8) -> u8 {
    unsafe {
        let mut data = data;
        let mut xExact = xExact;
        let mut yExact = yExact;
        let mut numFinished: u8 = 0u8;
        if ((((data).wrapping_offset(8)).read()) as i32)
            > ((((data).wrapping_offset(9)).read()) as i32)
        {
            let __p1 = (data).wrapping_offset(2);
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_add(((((data).wrapping_offset(6)).read()) as i32)))
                    as i16),
            );
            let __p2 = (data).wrapping_offset(10);
            (__p2).write(
                (((((__p2).read()) as i32)
                    .wrapping_add(((((data).wrapping_offset(9)).read()) as i32)))
                    as i16),
            );
            if ((((data).wrapping_offset(10)).read()) as i32)
                > ((((data).wrapping_offset(8)).read()) as i32)
            {
                let __p3 = (data).wrapping_offset(3);
                (__p3).write(
                    (((((__p3).read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(7)).read()) as i32)))
                        as i16),
                );
                let __p4 = (data).wrapping_offset(10);
                (__p4).write(
                    (((((__p4).read()) as i32)
                        .wrapping_sub(((((data).wrapping_offset(8)).read()) as i32)))
                        as i16),
                );
            }
        } else {
            let __p5 = (data).wrapping_offset(3);
            (__p5).write(
                (((((__p5).read()) as i32)
                    .wrapping_add(((((data).wrapping_offset(7)).read()) as i32)))
                    as i16),
            );
            let __p6 = (data).wrapping_offset(10);
            (__p6).write(
                (((((__p6).read()) as i32)
                    .wrapping_add(((((data).wrapping_offset(8)).read()) as i32)))
                    as i16),
            );
            if ((((data).wrapping_offset(10)).read()) as i32)
                > ((((data).wrapping_offset(9)).read()) as i32)
            {
                let __p7 = (data).wrapping_offset(2);
                (__p7).write(
                    (((((__p7).read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(6)).read()) as i32)))
                        as i16),
                );
                let __p8 = (data).wrapping_offset(10);
                (__p8).write(
                    (((((__p8).read()) as i32)
                        .wrapping_sub(((((data).wrapping_offset(9)).read()) as i32)))
                        as i16),
                );
            }
        }
        numFinished = 0u8;
        if ((((((data).wrapping_offset(6)).read()) as i32) > 0i32)
            && (((((data).wrapping_offset(2)).read()) as i32)
                >= ((((data).wrapping_offset(4)).read()) as i32)))
            || ((((((data).wrapping_offset(6)).read()) as i32) < 0i32)
                && (((((data).wrapping_offset(2)).read()) as i32)
                    <= ((((data).wrapping_offset(4)).read()) as i32)))
        {
            numFinished = (numFinished).wrapping_add(1);
            if (xExact) != 0 {
                ((data).wrapping_offset(2)).write(((data).wrapping_offset(4)).read());
            }
        }
        if ((((((data).wrapping_offset(7)).read()) as i32) > 0i32)
            && (((((data).wrapping_offset(3)).read()) as i32)
                >= ((((data).wrapping_offset(5)).read()) as i32)))
            || ((((((data).wrapping_offset(7)).read()) as i32) < 0i32)
                && (((((data).wrapping_offset(3)).read()) as i32)
                    <= ((((data).wrapping_offset(5)).read()) as i32)))
        {
            numFinished = (numFinished).wrapping_add(1);
            if (yExact) != 0 {
                ((data).wrapping_offset(3)).write(((data).wrapping_offset(5)).read());
            }
        }
        if ((numFinished) as i32) == 2i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn FrontierLogoWiggle_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        InitPatternWeaveTransition(task);
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (tilemap).cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
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
        LZ77UnCompVram(
            ((&raw const sFrontierLogo_Tileset)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tileset).cast::<u8>(),
        );
        LoadPalette(
            (((&raw const sFrontierLogo_Palette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FrontierLogoWiggle_SetGfx(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        LZ77UnCompVram(
            ((&raw const sFrontierLogo_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tilemap).cast::<u8>(),
        );
        SetSinWave(
            (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>()).cast::<i16>(),
            0i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
            132i16,
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read(),
            160i16,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Task_FrontierLogoWiggle(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sFrontierLogoWiggle_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_FrontierLogoWave(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sFrontierLogoWave_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FrontierLogoWave_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        InitTransitionData();
        ScanlineEffect_Clear();
        ClearGpuRegBits(0u8, 24576u16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(8192i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(32767i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(16i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(2560i16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14)
            .cast::<u16>())
        .write(16193u16);
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(16)
            .cast::<u16>())
        .write(
            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                << 8)
                | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32))
                as u16),
        );
        crate::c::volatile_write(
            ((67108944i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108946i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<u16>())
            .read(),
        );
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (tilemap).cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
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
        LZ77UnCompVram(
            ((&raw const sFrontierLogo_Tileset)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tileset).cast::<u8>(),
        );
        LoadPalette(
            (((&raw const sFrontierLogo_Palette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(22)
            .cast::<i16>())
        .write(0i16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FrontierLogoWave_SetGfx(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        LZ77UnCompVram(
            ((&raw const sFrontierLogo_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tilemap).cast::<u8>(),
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn FrontierLogoWave_InitScanline(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(22)
                            .cast::<i16>())
                        .read()) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        SetVBlankCallback(Some(VBlankCB_FrontierLogoWave));
        SetHBlankCallback(Some(HBlankCB_FrontierLogoWave));
        EnableInterrupts(2u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn FrontierLogoWave_Main(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u8 = 0u8;
        let mut sinVal: u16 = 0u16;
        let mut amplitude: u16 = 0u16;
        let mut sinSpread: u16 = 0u16;
        crate::c::volatile_write(
            (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()),
            0u8,
        );
        amplitude = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            >> 8) as u16);
        sinVal = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16);
        sinSpread = 384u16;
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_sub(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            )) as i16),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) >= 70i32
        {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                .wrapping_sub(384i32)
                >= 0i32
            {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(384i32)) as i16));
            } else {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            }
        }
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) >= 0i32)
            && (crate::c::rem_i32(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
                3i32,
            ) == 0i32)
        {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                < 16i32
            {
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                (__p3).write(((__p3).read()).wrapping_add(1));
            } else {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    > 0i32
                {
                    let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                    (__p4).write(((__p4).read()).wrapping_sub(1));
                }
            }
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<u16>())
            .write(
                (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    << 8)
                    | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)) as u16),
            );
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    let mut index: i16 = ((crate::c::div_i32(((sinVal) as i32), 256i32)) as i16);
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(22)
                            .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(
                                ((Sin(((((index) as i32) & 255i32) as i16), ((amplitude) as i16)))
                                    as i32),
                            )) as u16),
                    );
                }
                i = (i).wrapping_add(1);
                sinVal = ((((sinVal) as i32).wrapping_add(((sinSpread) as i32))) as u16);
            }
        }
        if (({
            let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
            let __t6 = ((__p5).read()).wrapping_add(1);
            (__p5).write(__t6);
            __t6
        }) as i32)
            == 101i32
        {
            let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            (__p7).write(((__p7).read()).wrapping_add(1));
            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) != 0)
            && (!((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0))
        {
            DestroyTask(FindTaskIdByFunc(Some(Task_FrontierLogoWave)));
        }
        let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
        (__p8).write((((((__p8).read()) as i32).wrapping_sub(17i32)) as i16));
        let __p9 = (((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read());
        crate::c::volatile_write(__p9, ((__p9).read_volatile()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_FrontierLogoWave() {
    unsafe {
        VBlankCB_BattleTransition();
        crate::c::volatile_write(
            ((67108944i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<u16>())
            .read(),
        );
        crate::c::volatile_write(
            ((67108946i32) as usize as *mut u16),
            ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16)
                .cast::<u16>())
            .read(),
        );
        if ((((&raw mut sTransitionData).cast::<u8>().cast::<*mut u8>()).read()).read_volatile())
            != 0
        {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(
                                    dmaRegs,
                                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .cast::<u16>())
                                        as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .wrapping_offset(1920))
                                    .cast::<u16>()) as usize
                                        as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2147483648i32)
                                        | crate::c::div_i32(320i32, crate::c::div_i32(16i32, 8i32)))
                                        as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
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
}
pub(crate) unsafe extern "C" fn HBlankCB_FrontierLogoWave() {
    unsafe {
        let mut var: u16 = (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
            .wrapping_offset(1920))
        .cast::<u16>())
        .wrapping_offset(((((67108870i32) as usize as *mut u16).read_volatile()) as i32) as isize))
        .read();
        crate::c::volatile_write(((67108882i32) as usize as *mut u16), var);
    }
}
pub(crate) unsafe extern "C" fn Task_FrontierSquares(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sFrontierSquares_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_FrontierSquaresSpiral(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sFrontierSquaresSpiral_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_FrontierSquaresScroll(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sFrontierSquaresScroll_Funcs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FrontierSquares_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        LZ77UnCompVram(
            ((&raw const sFrontierSquares_FilledBg_Tileset)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tileset).cast::<u8>(),
        );
        FillBgTilemapBufferRect_Palette0(0u8, 0u16, 0u8, 0u8, 32u8, 32u8);
        FillBgTilemapBufferRect(0u8, 1u16, 0u8, 0u8, 1u8, 32u8, 15u8);
        FillBgTilemapBufferRect(0u8, 1u16, 29u8, 0u8, 1u8, 32u8, 15u8);
        CopyBgTilemapBufferToVram(0u8);
        LoadPalette(
            (((&raw const sFrontierSquares_Palette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(1i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(10i16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FrontierSquares_Draw(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        CopyRectToBgTilemapBufferRect(
            0u8,
            (((&raw const sFrontierSquares_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            0u8,
            0u8,
            4u8,
            4u8,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
            4u8,
            4u8,
            15u8,
            0i16,
            0i16,
        );
        CopyBgTilemapBufferToVram(0u8);
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        if (({
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            let __t3 = ((__p2).read()).wrapping_add(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            == crate::c::div_i32(224i32, 32i32)
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(1i16);
            let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
            (__p4).write((((((__p4).read()) as i32).wrapping_add(4i32)) as i16));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                >= (crate::c::div_i32(160i32, 32i32)).wrapping_mul(4i32)
            {
                let __p5 = ((task).wrapping_add(8)).cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FrontierSquares_Shrink(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut i: u8 = 0u8;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
        {
            'l1: {
                let __sw3 =
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32);
                let __matched = __sw3 == 0i32 || __sw3 == 1i32 || __sw3 == 2i32 || __sw3 == 3i32;
                if __sw3 == 0i32 {
                    {
                        i = 250u8;
                        'l2: loop {
                            if !(((i) as i32) < 255i32) {
                                break 'l2;
                            }
                            'l3: {
                                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .write(0u16);
                                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .write(0u16);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
                if __sw3 == 1i32 {
                    BlendPalettes(4294934527u32, 16u8, 0u16);
                    LZ77UnCompVram(
                        ((&raw const sFrontierSquares_EmptyBg_Tileset)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        (tileset).cast::<u8>(),
                    );
                    break 'l1;
                }
                if __sw3 == 2i32 {
                    LZ77UnCompVram(
                        ((&raw const sFrontierSquares_Shrink1_Tileset)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        (tileset).cast::<u8>(),
                    );
                    break 'l1;
                }
                if __sw3 == 3i32 {
                    LZ77UnCompVram(
                        ((&raw const sFrontierSquares_Shrink2_Tileset)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        (tileset).cast::<u8>(),
                    );
                    break 'l1;
                }
                if !__matched {
                    FillBgTilemapBufferRect_Palette0(0u8, 1u16, 0u8, 0u8, 32u8, 32u8);
                    CopyBgTilemapBufferToVram(0u8);
                    let __p4 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    return 0u8;
                }
            }
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
            (__p5).write(((__p5).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FrontierSquaresSpiral_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        LZ77UnCompVram(
            ((&raw const sFrontierSquares_FilledBg_Tileset)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tileset).cast::<u8>(),
        );
        FillBgTilemapBufferRect_Palette0(0u8, 0u16, 0u8, 0u8, 32u8, 32u8);
        FillBgTilemapBufferRect(0u8, 1u16, 0u8, 0u8, 1u8, 32u8, 15u8);
        FillBgTilemapBufferRect(0u8, 1u16, 29u8, 0u8, 1u8, 32u8, 15u8);
        CopyBgTilemapBufferToVram(0u8);
        LoadPalette(
            (((&raw const sFrontierSquares_Palette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            224u16,
            32u16,
        );
        LoadPalette(
            (((&raw const sFrontierSquares_Palette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        BlendPalette(224u16, 16u16, 8u8, 0u16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
            ((((crate::c::div_i32(224i32, 32i32)).wrapping_mul(crate::c::div_i32(160i32, 32i32)))
                .wrapping_sub(1i32)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FrontierSquaresSpiral_Outward(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut pos: u8 = ((((&raw const sFrontierSquaresSpiral_Positions)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                as isize,
        ))
        .read();
        let mut x: u8 =
            ((crate::c::rem_i32(((pos) as i32), crate::c::div_i32(224i32, 32i32))) as u8);
        let mut y: u8 =
            ((crate::c::div_i32(((pos) as i32), crate::c::div_i32(224i32, 32i32))) as u8);
        CopyRectToBgTilemapBufferRect(
            0u8,
            (((&raw const sFrontierSquares_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            0u8,
            0u8,
            4u8,
            4u8,
            ((((4i32).wrapping_mul(((x) as i32))).wrapping_add(1i32)) as u8),
            (((4i32).wrapping_mul(((y) as i32))) as u8),
            4u8,
            4u8,
            15u8,
            0i16,
            0i16,
        );
        CopyBgTilemapBufferToVram(0u8);
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            < 0i32
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FrontierSquaresSpiral_SetBlack(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        BlendPalette(224u16, 16u16, 3u8, 0u16);
        BlendPalettes(4294918143u32, 16u8, 0u16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FrontierSquaresSpiral_Inward(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
            let __v2 = (((((__p1).read()) as i32) ^ 1i32) as i16);
            (__p1).write(__v2);
            __v2
        }) != 0
        {
            CopyRectToBgTilemapBufferRect(
                0u8,
                (((&raw const sFrontierSquares_Tilemap)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u32>())
                .cast::<u32>())
                .cast::<u8>(),
                0u8,
                0u8,
                4u8,
                4u8,
                ((((4i32).wrapping_mul(crate::c::rem_i32(
                    ((((((&raw const sFrontierSquaresSpiral_Positions)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32),
                    crate::c::div_i32(224i32, 32i32),
                )))
                .wrapping_add(1i32)) as u8),
                (((4i32).wrapping_mul(crate::c::div_i32(
                    ((((((&raw const sFrontierSquaresSpiral_Positions)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32),
                    crate::c::div_i32(224i32, 32i32),
                ))) as u8),
                4u8,
                4u8,
                14u8,
                0i16,
                0i16,
            );
        } else {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 0i32
            {
                FillBgTilemapBufferRect(
                    0u8,
                    1u16,
                    ((((4i32).wrapping_mul(
                        crate::c::rem_i32(
                            ((((((&raw const sFrontierSquaresSpiral_Positions)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32)
                                    .wrapping_sub(1i32)) as isize,
                            ))
                            .read()) as i32),
                            crate::c::div_i32(224i32, 32i32),
                        ),
                    ))
                    .wrapping_add(1i32)) as u8),
                    (((4i32).wrapping_mul(
                        crate::c::div_i32(
                            ((((((&raw const sFrontierSquaresSpiral_Positions)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32)
                                    .wrapping_sub(1i32)) as isize,
                            ))
                            .read()) as i32),
                            crate::c::div_i32(224i32, 32i32),
                        ),
                    )) as u8),
                    4u8,
                    4u8,
                    15u8,
                );
            }
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            >= (crate::c::div_i32(224i32, 32i32)).wrapping_mul(crate::c::div_i32(160i32, 32i32))
        {
            let __p4 = ((task).wrapping_add(8)).cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
        }
        CopyBgTilemapBufferToVram(0u8);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FrontierSquares_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        FillBgTilemapBufferRect_Palette0(0u8, 1u16, 0u8, 0u8, 32u8, 32u8);
        CopyBgTilemapBufferToVram(0u8);
        BlendPalettes(4294967295u32, 16u8, 0u16);
        DestroyTask(FindTaskIdByFunc(
            ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).read(),
        ));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_ScrollBg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !(({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            let __v2 = (((((__p1).read()) as i32) ^ 1i32) as i16);
            (__p1).write(__v2);
            __v2
        }) != 0)
        {
            SetGpuReg(18u8, ((&raw mut gBattle_BG0_X).cast::<u16>()).read());
            SetGpuReg(16u8, ((&raw mut gBattle_BG0_Y).cast::<u16>()).read());
            let __p3 = (&raw mut gBattle_BG0_X).cast::<u16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32),
                )) as u16),
            );
            let __p4 = (&raw mut gBattle_BG0_Y).cast::<u16>();
            (__p4).write(
                (((((__p4).read()) as i32).wrapping_add(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32),
                )) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn FrontierSquaresScroll_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut taskId: u8 = 0u8;
        let mut tilemap: *mut u16 = core::ptr::null_mut();
        let mut tileset: *mut u16 = core::ptr::null_mut();
        GetBg0TilesDst(&raw mut tilemap, &raw mut tileset);
        LZ77UnCompVram(
            ((&raw const sFrontierSquares_FilledBg_Tileset)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            (tileset).cast::<u8>(),
        );
        FillBgTilemapBufferRect_Palette0(0u8, 0u16, 0u8, 0u8, 32u8, 32u8);
        CopyBgTilemapBufferToVram(0u8);
        LoadPalette(
            (((&raw const sFrontierSquares_Palette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        SetGpuReg(18u8, ((&raw mut gBattle_BG0_X).cast::<u16>()).read());
        SetGpuReg(16u8, ((&raw mut gBattle_BG0_Y).cast::<u16>()).read());
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        taskId = CreateTask(Some(Task_ScrollBg), 1u8);
        'l1: {
            let __sw1 = crate::c::rem_i32(((Random()) as i32), 4i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(1i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write((-1i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write((-1i16));
                break 'l1;
            }
            if __sw1 == 2i32 {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(1i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write((-1i16));
                break 'l1;
            }
            if !__matched {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write((-1i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(1i16);
                break 'l1;
            }
        }
        let __p2 = ((task).wrapping_add(8)).cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FrontierSquaresScroll_Draw(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut pos: u8 = ((((&raw const sFrontierSquaresScroll_Positions)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                as isize,
        ))
        .read();
        let mut x: u8 = ((crate::c::div_i32(
            ((pos) as i32),
            (crate::c::div_i32(224i32, 32i32)).wrapping_add(1i32),
        )) as u8);
        let mut y: u8 = ((crate::c::rem_i32(
            ((pos) as i32),
            (crate::c::div_i32(224i32, 32i32)).wrapping_add(1i32),
        )) as u8);
        CopyRectToBgTilemapBufferRect(
            0u8,
            (((&raw const sFrontierSquares_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            0u8,
            0u8,
            4u8,
            4u8,
            ((((4i32).wrapping_mul(((x) as i32))).wrapping_add(1i32)) as u8),
            (((4i32).wrapping_mul(((y) as i32))) as u8),
            4u8,
            4u8,
            15u8,
            0i16,
            0i16,
        );
        CopyBgTilemapBufferToVram(0u8);
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= ((crate::c::div_u32(64u32, 1u32)) as i32)
        {
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FrontierSquaresScroll_SetBlack(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        BlendPalettes(4294934527u32, 16u8, 0u16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FrontierSquaresScroll_Erase(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut pos: u8 = ((((&raw const sFrontierSquaresScroll_Positions)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                as isize,
        ))
        .read();
        let mut x: u8 = ((crate::c::div_i32(
            ((pos) as i32),
            (crate::c::div_i32(224i32, 32i32)).wrapping_add(1i32),
        )) as u8);
        let mut y: u8 = ((crate::c::rem_i32(
            ((pos) as i32),
            (crate::c::div_i32(224i32, 32i32)).wrapping_add(1i32),
        )) as u8);
        FillBgTilemapBufferRect(
            0u8,
            1u16,
            ((((4i32).wrapping_mul(((x) as i32))).wrapping_add(1i32)) as u8),
            (((4i32).wrapping_mul(((y) as i32))) as u8),
            4u8,
            4u8,
            15u8,
        );
        CopyBgTilemapBufferToVram(0u8);
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= ((crate::c::div_u32(64u32, 1u32)) as i32)
        {
            DestroyTask(FindTaskIdByFunc(Some(Task_ScrollBg)));
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FrontierSquaresScroll_End(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        ((&raw mut gBattle_BG0_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG0_Y).cast::<u16>()).write(0u16);
        SetGpuReg(18u8, 0u16);
        SetGpuReg(16u8, ((&raw mut gBattle_BG0_Y).cast::<u16>()).read());
        FillBgTilemapBufferRect_Palette0(0u8, 1u16, 0u8, 0u8, 32u8, 32u8);
        CopyBgTilemapBufferToVram(0u8);
        BlendPalettes(4294967295u32, 16u8, 0u16);
        DestroyTask(FindTaskIdByFunc(
            ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).read(),
        ));
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
