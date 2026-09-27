//! Translated from `src/rayquaza_scene.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sTasksForAnimations sOam_64x64 sOam_32x32 sOam_64x32 sOam_32x16 sOam_16x8 sOam_16x32 sOam_16x16 sOam_32x8 sAnim_DuoFightPre_Groudon_Head sAnim_DuoFightPre_Groudon_Body sAnims_DuoFightPre_Groudon sSpriteTemplate_DuoFightPre_Groudon sAnim_DuoFightPre_GroudonShoulderKyogreDorsalFin sAnims_DuoFightPre_GroudonShoulderKyogreDorsalFin sSpriteTemplate_DuoFightPre_GroudonShoulder sAnim_DuoFightPre_GroudonClaw sAnims_DuoFightPre_GroudonClaw sSpriteTemplate_DuoFightPre_GroudonClaw sAnim_DuoFightPre_Kyogre_TopLeft sAnim_DuoFightPre_Kyogre_TopRight sAnim_DuoFightPre_Kyogre_FaceLeft sAnim_DuoFightPre_Kyogre_FaceRight sAnim_DuoFightPre_Kyogre_ChinLeft sAnim_DuoFightPre_Kyogre_ChinRight sAnim_DuoFightPre_Kyogre_LeftPectoralFin sAnim_DuoFightPre_Kyogre_LeftShoulder sAnim_DuoFightPre_Kyogre_RightShoulder sAnims_DuoFightPre_Kyogre sSpriteTemplate_DuoFightPre_Kyogre sAnim_DuoFightPre_KyogrePectoralFin sAnims_DuoFightPre_KyogrePectoralFin sSpriteTemplate_DuoFightPre_KyogrePectoralFin sSpriteTemplate_DuoFightPre_KyogreDorsalFin sScanlineParams_DuoFight_Clouds sBgTemplates_DuoFight sAnim_DuoFight_Groudon_Head sAnim_DuoFight_Groudon_Body sAnims_DuoFight_Groudon sSpriteSheet_DuoFight_Groudon sSpritePal_DuoFight_Groudon sSpriteTemplate_DuoFight_Groudon sAnim_DuoFight_GroudonShoulderKyogreDorsalFin sAnims_DuoFight_GroudonShoulderKyogreDorsalFin sSpriteSheet_DuoFight_GroudonShoulder sSpriteTemplate_DuoFight_GroudonShoulder sAnim_DuoFight_GroudonClaw sAnims_DuoFight_GroudonClaw sSpriteSheet_DuoFight_GroudonClaw sSpriteTemplate_DuoFight_GroudonClaw sAnim_DuoFight_Kyogre_TopLeft sAnim_DuoFight_Kyogre_TopRight sAnim_DuoFight_Kyogre_FaceLeft sAnim_DuoFight_Kyogre_FaceRight sAnim_DuoFight_Kyogre_ChinLeft sAnim_DuoFight_Kyogre_ChinRight sAnim_DuoFight_Kyogre_LeftPectoralFin sAnim_DuoFight_Kyogre_LeftShoulder sAnim_DuoFight_Kyogre_RightShoulder sAnims_DuoFight_Kyogre sSpriteSheet_DuoFight_Kyogre sSpritePal_DuoFight_Kyogre sSpriteTemplate_DuoFight_Kyogre sAnim_DuoFight_KyogrePectoralFin sAnims_DuoFight_KyogrePectoralFin sSpriteSheet_DuoFight_KyogrePectoralFin sSpriteTemplate_DuoFight_KyogrePectoralFin sSpriteSheet_DuoFight_KyogreDorsalFin sSpriteTemplate_DuoFight_KyogreDorsalFin sBgTemplates_TakesFlight sAnim_TakesFlight_Smoke sAnims_TakesFlight_Smoke sAffineAnim_TakesFlight_Smoke sAffineAnims_TakesFlight_Smoke sSpriteSheet_TakesFlight_Smoke sSpritePal_TakesFlight_Smoke sSpriteTemplate_TakesFlight_Smoke sTakesFlight_SmokeCoords sBgTemplates_Descends sAnim_Descends_Rayquaza sAnims_Descends_Rayquaza sAnim_Descends_RayquazaTail sAnims_Descends_RayquazaTail sSpriteSheet_Descends_Rayquaza sSpriteSheet_Descends_RayquazaTail sSpritePal_Descends_Rayquaza sSpriteTemplate_Descends_Rayquaza sSpriteTemplate_Descends_RayquazaTail sBgTemplates_Charges sAnim_ChasesAway_Groudon_Still sAnim_ChasesAway_Groudon_Moving sAnims_ChasesAway_Groudon sAnim_ChasesAway_GroudonTail sAnims_ChasesAway_GroudonTail sAnim_ChasesAway_Kyogre_Front sAnim_ChasesAway_Kyogre_Back sAnim_ChasesAway_Kyogre_Tail sAnims_ChasesAway_Kyogre sAnim_ChasesAway_Rayquaza_FlyingDown sAnim_ChasesAway_Rayquaza_Arriving sAnim_ChasesAway_Rayquaza_Floating sAnim_ChasesAway_Rayquaza_Shouting sAnims_ChasesAway_Rayquaza sAnim_ChasesAway_RayquazaTail_FlyingDown sAnim_ChasesAway_RayquazaTail_Arriving sAnim_ChasesAway_RayquazaTail_Floating sAnim_ChasesAway_RayquazaTail_Shouting sAnims_ChasesAway_RayquazaTail sAnim_ChasesAway_KyogreSplash sAnims_ChasesAway_KyogreSplash sSpriteSheet_ChasesAway_Groudon sSpriteSheet_ChasesAway_GroudonTail sSpriteSheet_ChasesAway_Kyogre sSpriteSheet_ChasesAway_Rayquaza sSpriteSheet_ChasesAway_RayquazaTail sSpriteSheet_ChasesAway_KyogreSplash sSpritePal_ChasesAway_Groudon sSpritePal_ChasesAway_Kyogre sSpritePal_ChasesAway_Rayquaza sSpritePal_ChasesAway_KyogreSplash sSpriteTemplate_ChasesAway_Groudon sSpriteTemplate_ChasesAway_GroudonTail sSpriteTemplate_ChasesAway_Kyogre sSpriteTemplate_ChasesAway_Rayquaza sSpriteTemplate_ChasesAway_RayquazaTail sSpriteTemplate_ChasesAway_KyogreSplash sBgTemplates_ChasesAway
#[allow(unused_imports)]
use crate::data::rayquaza_scene::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRayScene: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gPaletteFade: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gRaySceneCharges_Bg_Gfx: u8;
    static mut gRaySceneCharges_Bg_Pal: u8;
    static mut gRaySceneCharges_Bg_Tilemap: u8;
    static mut gRaySceneCharges_Orbs_Tilemap: u8;
    static mut gRaySceneCharges_Rayquaza_Gfx: u8;
    static mut gRaySceneCharges_Rayquaza_Tilemap: u8;
    static mut gRaySceneCharges_Streaks_Gfx: u8;
    static mut gRaySceneCharges_Streaks_Tilemap: u8;
    static mut gRaySceneChasesAway_Bg_Pal: u8;
    static mut gRaySceneChasesAway_Bg_Tilemap: u8;
    static mut gRaySceneChasesAway_Light_Gfx: u8;
    static mut gRaySceneChasesAway_Light_Tilemap: u8;
    static mut gRaySceneChasesAway_Ring_Gfx: u8;
    static mut gRaySceneChasesAway_Ring_Tilemap: u8;
    static mut gRaySceneDescends_Bg_Gfx: u8;
    static mut gRaySceneDescends_Bg_Pal: u8;
    static mut gRaySceneDescends_Bg_Tilemap: u8;
    static mut gRaySceneDescends_Light_Gfx: u8;
    static mut gRaySceneDescends_Light_Tilemap: u8;
    static mut gRaySceneDuoFight_Clouds1_Tilemap: u8;
    static mut gRaySceneDuoFight_Clouds2_Tilemap: u8;
    static mut gRaySceneDuoFight_Clouds3_Tilemap: u8;
    static mut gRaySceneDuoFight_Clouds_Gfx: u8;
    static mut gRaySceneDuoFight_Clouds_Pal: u8;
    static mut gRaySceneTakesFlight_Bg_Gfx: u8;
    static mut gRaySceneTakesFlight_Bg_Tilemap: u8;
    static mut gRaySceneTakesFlight_Rayquaza_Gfx: u8;
    static mut gRaySceneTakesFlight_Rayquaza_Pal: u8;
    static mut gRaySceneTakesFlight_Rayquaza_Tilemap: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BlendPalettesGradually(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16, a5: u8, a6: u8);
    fn BuildOamBuffer();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearScheduledBgCopiesToVram();
    fn CpuFastSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroyTask(a0: u8);
    fn DoScheduledBgTilemapCopiesToVram();
    fn EnableInterrupts(a0: u16);
    fn FillPalette(a0: u16, a1: u16, a2: u16);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetBgY(a0: u8) -> i32;
    fn GetGpuReg(a0: u8) -> u16;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitSpriteAffineAnim(a0: *mut u8);
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlayNewMapMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn Random() -> u16;
    fn ResetAllBgsCoordinates();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn ResetVramOamAndBgCntRegs();
    fn RunTasks();
    fn ScanlineEffect_Clear();
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn ScanlineEffect_SetParams(a0: crate::c::Rec4<12>);
    fn ScanlineEffect_Stop();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetBgAffine(a0: u8, a1: i32, a2: i32, a3: i16, a4: i16, a5: i16, a6: i16, a7: u16);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StopMapMusic();
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoRayquazaScene(
    animId: u8,
    endEarly: u8,
    exitCallback: Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let mut animId = animId;
        let mut endEarly = endEarly;
        let mut exitCallback = exitCallback;
        ((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(8216u32));
        ((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8198))
            .write(animId);
        ((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(exitCallback);
        ((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8199))
            .write(endEarly);
        SetMainCallback2(Some(CB2_InitRayquazaScene));
    }
}
pub(crate) unsafe extern "C" fn CB2_InitRayquazaScene() {
    unsafe {
        SetVBlankHBlankCallbacksToNull();
        ClearScheduledBgCopiesToVram();
        ScanlineEffect_Stop();
        FreeAllSpritePalettes();
        ResetPaletteFade();
        ResetSpriteData();
        ResetTasks();
        FillPalette(0u16, 240u16, 32u16);
        CreateTask(
            ((((&raw const sTasksForAnimations)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .cast::<Option<unsafe extern "C" fn(u8)>>())
            .wrapping_offset(
                ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8198))
                .read()) as i32) as isize,
            ))
            .read(),
            0u8,
        );
        SetMainCallback2(Some(CB2_RayquazaScene));
    }
}
pub(crate) unsafe extern "C" fn CB2_RayquazaScene() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_RayquazaScene() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn Task_EndAfterFadeScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            ResetSpriteData();
            FreeAllSpritePalettes();
            SetMainCallback2(
                ((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
            Free(((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read());
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SetNextAnim(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            if ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8199))
            .read()) as i32)
                == 1i32
            {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_EndAfterFadeScreen));
            } else {
                let __p1 = (((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8198);
                (__p1).write(((__p1).read()).wrapping_add(1));
                ((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8196)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(
                    ((((&raw const sTasksForAnimations)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .wrapping_offset(
                        ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8198))
                        .read()) as i32) as isize,
                    ))
                    .read(),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetWindowsHideVertBorders() {
    unsafe {
        SetGpuReg(72u8, 63u16);
        SetGpuReg(74u8, 0u16);
        SetGpuReg(64u8, 240u16);
        SetGpuReg(68u8, 6280u16);
        (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).write(0u16);
        (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).write(0u16);
    }
}
pub(crate) unsafe extern "C" fn ResetWindowDimensions() {
    unsafe {
        SetGpuReg(72u8, 63u16);
        SetGpuReg(74u8, 63u16);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleDuoFightPre(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        DuoFight_AnimateRain();
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            let mut frame: i16 = (data).read();
            if ((frame) as i32) == 64i32 {
                DuoFight_Lightning1();
            } else {
                if ((frame) as i32) == 144i32 {
                    DuoFight_Lightning2();
                } else {
                    'l1: {
                        let __sw1 = ((frame) as i32);
                        if __sw1 == 328i32 {
                            DuoFightEnd(taskId, 0i8);
                            return;
                        }
                        if __sw1 == 148i32 {
                            DuoFight_LightningLong();
                            break 'l1;
                        }
                    }
                }
            }
            (data).write(((data).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn DuoFightPre_CreateGroudonSprites() -> u8 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        let mut data: *mut i16 = core::ptr::null_mut();
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_DuoFightPre_Groudon)
                .cast::<u8>()
                .cast_mut(),
            88i16,
            72i16,
            3u8,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_DuoFightPre_Groudon));
        data = ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>();
        (data).write(
            ((CreateSprite(
                (&raw const sSpriteTemplate_DuoFightPre_Groudon)
                    .cast::<u8>()
                    .cast_mut(),
                56i16,
                104i16,
                3u8,
            )) as i16),
        );
        ((data).wrapping_offset(1)).write(
            ((CreateSprite(
                (&raw const sSpriteTemplate_DuoFightPre_GroudonShoulder)
                    .cast::<u8>()
                    .cast_mut(),
                75i16,
                101i16,
                0u8,
            )) as i16),
        );
        ((data).wrapping_offset(2)).write(
            ((CreateSprite(
                (&raw const sSpriteTemplate_DuoFightPre_GroudonClaw)
                    .cast::<u8>()
                    .cast_mut(),
                109i16,
                114i16,
                1u8,
            )) as i16),
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((data).read()) as i32) as isize * 68),
            1u8,
        );
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DuoFightPre_Groudon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut data: *mut i16 = ((sprite).wrapping_add(46)).cast::<i16>();
        let __p1 = (data).wrapping_offset(5);
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = (data).wrapping_offset(5);
        (__p2).write((((((__p2).read()) as i32) & 31i32) as i16));
        if (((((data).wrapping_offset(5)).read()) as i32) == 0i32)
            && (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) != 72i32)
        {
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_sub(1));
            let __p4 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_sub(1));
            let __p5 = (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>();
            (__p5).write(((__p5).read()).wrapping_sub(1));
            let __p6 = (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>();
            (__p6).write(((__p6).read()).wrapping_sub(1));
        }
        'l1: {
            let __sw7 = ((((sprite).wrapping_add(43)).read()) as i32);
            if __sw7 == 0i32 {
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                break 'l1;
            }
            if __sw7 == 1i32 || __sw7 == 3i32 {
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write((-1i16));
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write((-1i16));
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                break 'l1;
            }
            if __sw7 == 2i32 {
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write((-1i16));
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write((-2i16));
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DuoFightPre_CreateKyogreSprites() -> u8 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        let mut data: *mut i16 = core::ptr::null_mut();
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_DuoFightPre_Kyogre)
                .cast::<u8>()
                .cast_mut(),
            136i16,
            96i16,
            1u8,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_DuoFightPre_Kyogre));
        data = ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>();
        (data).write(
            ((((CreateSprite(
                (&raw const sSpriteTemplate_DuoFightPre_Kyogre)
                    .cast::<u8>()
                    .cast_mut(),
                168i16,
                96i16,
                1u8,
            )) as i32)
                << 8) as i16),
        );
        (data).write(
            (((((data).read()) as i32)
                | ((CreateSprite(
                    (&raw const sSpriteTemplate_DuoFightPre_Kyogre)
                        .cast::<u8>()
                        .cast_mut(),
                    136i16,
                    112i16,
                    1u8,
                )) as i32)) as i16),
        );
        ((data).wrapping_offset(1)).write(
            ((((CreateSprite(
                (&raw const sSpriteTemplate_DuoFightPre_Kyogre)
                    .cast::<u8>()
                    .cast_mut(),
                168i16,
                112i16,
                1u8,
            )) as i32)
                << 8) as i16),
        );
        let __p1 = (data).wrapping_offset(1);
        (__p1).write(
            (((((__p1).read()) as i32)
                | ((CreateSprite(
                    (&raw const sSpriteTemplate_DuoFightPre_Kyogre)
                        .cast::<u8>()
                        .cast_mut(),
                    136i16,
                    128i16,
                    1u8,
                )) as i32)) as i16),
        );
        ((data).wrapping_offset(2)).write(
            ((((CreateSprite(
                (&raw const sSpriteTemplate_DuoFightPre_Kyogre)
                    .cast::<u8>()
                    .cast_mut(),
                168i16,
                128i16,
                1u8,
            )) as i32)
                << 8) as i16),
        );
        let __p2 = (data).wrapping_offset(2);
        (__p2).write(
            (((((__p2).read()) as i32)
                | ((CreateSprite(
                    (&raw const sSpriteTemplate_DuoFightPre_Kyogre)
                        .cast::<u8>()
                        .cast_mut(),
                    104i16,
                    128i16,
                    2u8,
                )) as i32)) as i16),
        );
        ((data).wrapping_offset(3)).write(
            ((((CreateSprite(
                (&raw const sSpriteTemplate_DuoFightPre_Kyogre)
                    .cast::<u8>()
                    .cast_mut(),
                136i16,
                128i16,
                2u8,
            )) as i32)
                << 8) as i16),
        );
        let __p3 = (data).wrapping_offset(3);
        (__p3).write(
            (((((__p3).read()) as i32)
                | ((CreateSprite(
                    (&raw const sSpriteTemplate_DuoFightPre_Kyogre)
                        .cast::<u8>()
                        .cast_mut(),
                    184i16,
                    128i16,
                    0u8,
                )) as i32)) as i16),
        );
        ((data).wrapping_offset(4)).write(
            ((((CreateSprite(
                (&raw const sSpriteTemplate_DuoFightPre_KyogrePectoralFin)
                    .cast::<u8>()
                    .cast_mut(),
                208i16,
                132i16,
                0u8,
            )) as i32)
                << 8) as i16),
        );
        let __p4 = (data).wrapping_offset(4);
        (__p4).write(
            (((((__p4).read()) as i32)
                | ((CreateSprite(
                    (&raw const sSpriteTemplate_DuoFightPre_KyogreDorsalFin)
                        .cast::<u8>()
                        .cast_mut(),
                    200i16,
                    120i16,
                    1u8,
                )) as i32)) as i16),
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).read()) as i32) >> 8) as isize * 68),
            1u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).read()) as i32) & 255i32) as isize * 68),
            2u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(1)).read()) as i32) >> 8) as isize * 68,
            ),
            3u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(1)).read()) as i32) & 255i32) as isize * 68,
            ),
            4u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(2)).read()) as i32) >> 8) as isize * 68,
            ),
            5u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(2)).read()) as i32) & 255i32) as isize * 68,
            ),
            6u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(3)).read()) as i32) >> 8) as isize * 68,
            ),
            7u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(3)).read()) as i32) & 255i32) as isize * 68,
            ),
            8u8,
        );
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DuoFightPre_Kyogre(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut data: *mut i16 = ((sprite).wrapping_add(46)).cast::<i16>();
        let __p1 = (data).wrapping_offset(5);
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = (data).wrapping_offset(5);
        (__p2).write((((((__p2).read()) as i32) & 31i32) as i16));
        if (((((data).wrapping_offset(5)).read()) as i32) == 0i32)
            && (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) != 152i32)
        {
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            let __p4 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 8) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
            let __p5 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) & 255i32) as isize
                    * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p5).write(((__p5).read()).wrapping_add(1));
            let __p6 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(1)).read()) as i32) >> 8) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p6).write(((__p6).read()).wrapping_add(1));
            let __p7 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(1)).read()) as i32) & 255i32) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p7).write(((__p7).read()).wrapping_add(1));
            let __p8 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(2)).read()) as i32) >> 8) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p8).write(((__p8).read()).wrapping_add(1));
            let __p9 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(2)).read()) as i32) & 255i32) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p9).write(((__p9).read()).wrapping_add(1));
            let __p10 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(3)).read()) as i32) >> 8) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p10).write(((__p10).read()).wrapping_add(1));
            let __p11 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(3)).read()) as i32) & 255i32) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p11).write(((__p11).read()).wrapping_add(1));
            let __p12 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(4)).read()) as i32) >> 8) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p12).write(((__p12).read()).wrapping_add(1));
            let __p13 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(4)).read()) as i32) & 255i32) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p13).write(((__p13).read()).wrapping_add(1));
        }
        'l1: {
            let __sw14 = ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(2)).read()) as i32) & 255i32) as isize * 68,
            ))
            .wrapping_add(43))
            .read()) as i32);
            if __sw14 == 0i32 {
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).read()) as i32) >> 8) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).read()) as i32) & 255i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(1)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(1)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(2)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(2)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(3)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(3)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(4)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(4)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                break 'l1;
            }
            if __sw14 == 1i32 || __sw14 == 3i32 {
                ((sprite).wrapping_add(38).cast::<i16>()).write(1i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).read()) as i32) >> 8) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).read()) as i32) & 255i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(1)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(1)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(2)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(2)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(3)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(3)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(4)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(4)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                break 'l1;
            }
            if __sw14 == 2i32 {
                ((sprite).wrapping_add(38).cast::<i16>()).write(2i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).read()) as i32) >> 8) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(2i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).read()) as i32) & 255i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(2i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(1)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(2i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(1)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(2i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(2)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(2i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(4)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(2i16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_DuoFight() {
    unsafe {
        VBlankCB_RayquazaScene();
        ScanlineEffect_InitHBlankDmaTransfer();
    }
}
pub(crate) unsafe extern "C" fn InitDuoFightSceneBgs() {
    unsafe {
        ResetVramOamAndBgCntRegs();
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates_DuoFight).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(12u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            0u8,
            (((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            1u8,
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(2048))
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            2u8,
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(4096))
            .cast::<u8>(),
        );
        ResetAllBgsCoordinates();
        ScheduleBgCopyTilemapToVram(0u8);
        ScheduleBgCopyTilemapToVram(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        SetGpuReg(0u8, 4160u16);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        SetGpuReg(80u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn LoadDuoFightSceneGfx() {
    unsafe {
        ResetTempTileDataBuffers();
        DecompressAndCopyTileDataToVram(
            0u8,
            (((&raw mut gRaySceneDuoFight_Clouds_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        'l1: loop {
            if !((FreeTempTileDataBuffersIfPossible()) != 0) {
                break 'l1;
            }
        }
        LZDecompressWram(
            ((&raw mut gRaySceneDuoFight_Clouds2_Tilemap).cast::<u32>()).cast::<u32>(),
            (((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .cast::<u8>(),
        );
        LZDecompressWram(
            ((&raw mut gRaySceneDuoFight_Clouds1_Tilemap).cast::<u32>()).cast::<u32>(),
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(2048))
            .cast::<u8>(),
        );
        LZDecompressWram(
            ((&raw mut gRaySceneDuoFight_Clouds3_Tilemap).cast::<u32>()).cast::<u32>(),
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(4096))
            .cast::<u8>(),
        );
        LoadCompressedPalette(
            ((&raw mut gRaySceneDuoFight_Clouds_Pal).cast::<u32>()).cast::<u32>(),
            0u16,
            64u16,
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_DuoFight_Groudon)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_DuoFight_GroudonShoulder)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_DuoFight_GroudonClaw)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_DuoFight_Kyogre)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_DuoFight_KyogrePectoralFin)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_DuoFight_KyogreDorsalFin)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpritePalette(
            (&raw const sSpritePal_DuoFight_Groudon)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpritePalette(
            (&raw const sSpritePal_DuoFight_Kyogre)
                .cast::<u8>()
                .cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_DuoFightAnim(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ScanlineEffect_Clear();
        InitDuoFightSceneBgs();
        LoadDuoFightSceneGfx();
        {
            let mut tmp: u32 = 0u32;
            (&raw mut tmp).write_volatile(0u32);
            'l1: loop {
                'l2: {
                    CpuFastSet(
                        (&raw mut tmp).cast::<u8>(),
                        (&raw mut gScanlineEffectRegBuffers).cast::<u8>(),
                        (16777216u32
                            | (crate::c::div_u32(
                                3840u32,
                                ((crate::c::div_i32(32i32, 8i32)) as u32),
                            ) & 2097151u32)),
                    );
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
        ScanlineEffect_SetParams(
            (&raw const sScanlineParams_DuoFight_Clouds)
                .cast::<u8>()
                .cast_mut()
                .cast::<crate::c::Rec4<12>>()
                .read_unaligned(),
        );
        (data).write(0i16);
        ((data).wrapping_offset(1))
            .write(((CreateTask(Some(Task_DuoFight_AnimateClouds), 0u8)) as i16));
        if ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8198))
            .read()) as i32)
            == 0i32
        {
            ((data).wrapping_offset(2)).write(((DuoFightPre_CreateGroudonSprites()) as i16));
            ((data).wrapping_offset(3)).write(((DuoFightPre_CreateKyogreSprites()) as i16));
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleDuoFightPre));
        } else {
            ((data).wrapping_offset(2)).write(((DuoFight_CreateGroudonSprites()) as i16));
            ((data).wrapping_offset(3)).write(((DuoFight_CreateKyogreSprites()) as i16));
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleDuoFight));
            StopMapMusic();
        }
        BlendPalettes(4294967295u32, 16u8, 0u16);
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
        SetVBlankCallback(Some(VBlankCB_DuoFight));
        PlaySE(83u16);
    }
}
pub(crate) unsafe extern "C" fn Task_DuoFight_AnimateClouds(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i16 = 0i16;
        let mut data: *mut u16 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u16>();
        {
            i = 24i16;
            'l1: loop {
                if !(((i) as i32) < 92i32) {
                    break 'l1;
                }
                'l2: {
                    if ((i) as i32) <= 47i32 {
                        ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write((((((data).read()) as i32) >> 8) as u16));
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write((((((data).read()) as i32) >> 8) as u16));
                    } else {
                        if ((i) as i32) <= 63i32 {
                            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(((((((data).wrapping_offset(1)).read()) as i32) >> 8) as u16));
                            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                .wrapping_offset(1920))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(((((((data).wrapping_offset(1)).read()) as i32) >> 8) as u16));
                        } else {
                            if ((i) as i32) <= 75i32 {
                                ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                    .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(
                                    ((((((data).wrapping_offset(2)).read()) as i32) >> 8) as u16),
                                );
                                (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                    .wrapping_offset(1920))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(
                                    ((((((data).wrapping_offset(2)).read()) as i32) >> 8) as u16),
                                );
                            } else {
                                if ((i) as i32) <= 83i32 {
                                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .write(
                                        ((((((data).wrapping_offset(3)).read()) as i32) >> 8)
                                            as u16),
                                    );
                                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                        .wrapping_offset(1920))
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .write(
                                        ((((((data).wrapping_offset(3)).read()) as i32) >> 8)
                                            as u16),
                                    );
                                } else {
                                    if ((i) as i32) <= 87i32 {
                                        ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                            .cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(
                                            ((((((data).wrapping_offset(4)).read()) as i32) >> 8)
                                                as u16),
                                        );
                                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                            .wrapping_offset(1920))
                                        .cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(
                                            ((((((data).wrapping_offset(4)).read()) as i32) >> 8)
                                                as u16),
                                        );
                                    } else {
                                        ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                            .cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(
                                            ((((((data).wrapping_offset(5)).read()) as i32) >> 8)
                                                as u16),
                                        );
                                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                            .wrapping_offset(1920))
                                        .cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(
                                            ((((((data).wrapping_offset(5)).read()) as i32) >> 8)
                                                as u16),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8198))
            .read()) as i32)
            == 0i32
        {
            (data).write((((((data).read()) as i32).wrapping_add(448i32)) as u16));
            let __p1 = (data).wrapping_offset(1);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(384i32)) as u16));
            let __p2 = (data).wrapping_offset(2);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(320i32)) as u16));
            let __p3 = (data).wrapping_offset(3);
            (__p3).write((((((__p3).read()) as i32).wrapping_add(256i32)) as u16));
            let __p4 = (data).wrapping_offset(4);
            (__p4).write((((((__p4).read()) as i32).wrapping_add(192i32)) as u16));
            let __p5 = (data).wrapping_offset(5);
            (__p5).write((((((__p5).read()) as i32).wrapping_add(128i32)) as u16));
        } else {
            (data).write((((((data).read()) as i32).wrapping_add(768i32)) as u16));
            let __p6 = (data).wrapping_offset(1);
            (__p6).write((((((__p6).read()) as i32).wrapping_add(640i32)) as u16));
            let __p7 = (data).wrapping_offset(2);
            (__p7).write((((((__p7).read()) as i32).wrapping_add(512i32)) as u16));
            let __p8 = (data).wrapping_offset(3);
            (__p8).write((((((__p8).read()) as i32).wrapping_add(384i32)) as u16));
            let __p9 = (data).wrapping_offset(4);
            (__p9).write((((((__p9).read()) as i32).wrapping_add(256i32)) as u16));
            let __p10 = (data).wrapping_offset(5);
            (__p10).write((((((__p10).read()) as i32).wrapping_add(128i32)) as u16));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleDuoFight(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        DuoFight_AnimateRain();
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            let mut frame: i16 = (data).read();
            if (((frame) as i32) == 32i32) || (((frame) as i32) == 112i32) {
                DuoFight_Lightning1();
            } else {
                if ((frame) as i32) == 216i32 {
                    DuoFight_Lightning2();
                } else {
                    if ((frame) as i32) == 220i32 {
                        DuoFight_LightningLong();
                    } else {
                        'l1: {
                            let __sw1 = ((frame) as i32);
                            if __sw1 == 412i32 {
                                DuoFightEnd(taskId, 2i8);
                                return;
                            }
                            if __sw1 == 380i32 {
                                SetGpuReg(80u8, 580u16);
                                ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                                    ((((data).wrapping_offset(1)).read()) as i32) as isize * 40,
                                ))
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                                .write(Some(DuoFight_PanOffScene));
                                (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                                    ((((data).wrapping_offset(1)).read()) as i32) as isize * 40,
                                ))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .write(0i16);
                                ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                                    ((((data).wrapping_offset(1)).read()) as i32) as isize * 40,
                                ))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .write(((data).wrapping_offset(2)).read());
                                ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                                    ((((data).wrapping_offset(1)).read()) as i32) as isize * 40,
                                ))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(3))
                                .write(((data).wrapping_offset(3)).read());
                                ScanlineEffect_Stop();
                                break 'l1;
                            }
                        }
                    }
                }
            }
            (data).write(((data).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn DuoFight_Lightning1() {
    unsafe {
        PlaySE(87u16);
        BlendPalettesGradually(32767u32, 0i8, 16u8, 0u8, 65535u16, 0u8, 0u8);
        BlendPalettesGradually(4294901760u32, 0i8, 16u8, 0u8, 0u16, 0u8, 1u8);
    }
}
pub(crate) unsafe extern "C" fn DuoFight_Lightning2() {
    unsafe {
        PlaySE(87u16);
        BlendPalettesGradually(32767u32, 0i8, 16u8, 16u8, 65535u16, 0u8, 0u8);
        BlendPalettesGradually(4294901760u32, 0i8, 16u8, 16u8, 0u16, 0u8, 1u8);
    }
}
pub(crate) unsafe extern "C" fn DuoFight_LightningLong() {
    unsafe {
        BlendPalettesGradually(32767u32, 4i8, 16u8, 0u8, 65535u16, 0u8, 0u8);
        BlendPalettesGradually(4294901760u32, 4i8, 16u8, 0u8, 0u16, 0u8, 1u8);
    }
}
pub(crate) unsafe extern "C" fn DuoFight_AnimateRain() {
    unsafe {
        ChangeBgX(2u8, 1024i32, 1u8);
        ChangeBgY(2u8, 2048i32, 2u8);
    }
}
pub(crate) unsafe extern "C" fn DuoFight_PanOffScene(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut bgY: u16 = 0u16;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        DuoFight_SlideGroudonDown(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68),
        );
        DuoFight_SlideKyogreDown(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(3)).read()) as i32) as isize * 68),
        );
        bgY = ((GetBgY(1u8)) as u16);
        if (GetBgY(1u8) == 0i32) || (((bgY) as i32) > 32768i32) {
            ChangeBgY(1u8, 1024i32, 2u8);
        }
        if (((data).read()) as i32) != 16i32 {
            (data).write(((data).read()).wrapping_add(1));
            SetGpuReg(
                82u8,
                ((((((data).read()) as i32) << 8) | (16i32).wrapping_sub((((data).read()) as i32)))
                    as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DuoFightEnd(taskId: u8, palDelay: i8) {
    unsafe {
        let mut taskId = taskId;
        let mut palDelay = palDelay;
        PlaySE(84u16);
        BeginNormalPaletteFade(4294967295u32, palDelay, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_DuoFightEnd));
    }
}
pub(crate) unsafe extern "C" fn Task_DuoFightEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        DuoFight_AnimateRain();
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DestroyTask(((((data).wrapping_offset(1)).read()) as u8));
            ChangeBgY(1u8, 0i32, 0u8);
            SetVBlankCallback(None);
            ScanlineEffect_Stop();
            ResetSpriteData();
            FreeAllSpritePalettes();
            (data).write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SetNextAnim));
        }
    }
}
pub(crate) unsafe extern "C" fn DuoFight_CreateGroudonSprites() -> u8 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        let mut data: *mut i16 = core::ptr::null_mut();
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_DuoFight_Groudon)
                .cast::<u8>()
                .cast_mut(),
            98i16,
            72i16,
            3u8,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_DuoFight_Groudon));
        data = ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>();
        (data).write(
            ((CreateSprite(
                (&raw const sSpriteTemplate_DuoFight_Groudon)
                    .cast::<u8>()
                    .cast_mut(),
                66i16,
                104i16,
                3u8,
            )) as i16),
        );
        ((data).wrapping_offset(1)).write(
            ((CreateSprite(
                (&raw const sSpriteTemplate_DuoFight_GroudonShoulder)
                    .cast::<u8>()
                    .cast_mut(),
                85i16,
                101i16,
                0u8,
            )) as i16),
        );
        ((data).wrapping_offset(2)).write(
            ((CreateSprite(
                (&raw const sSpriteTemplate_DuoFight_GroudonClaw)
                    .cast::<u8>()
                    .cast_mut(),
                119i16,
                114i16,
                1u8,
            )) as i16),
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((data).read()) as i32) as isize * 68),
            1u8,
        );
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DuoFight_Groudon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut data: *mut i16 = ((sprite).wrapping_add(46)).cast::<i16>();
        let __p1 = (data).wrapping_offset(5);
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = (data).wrapping_offset(5);
        (__p2).write((((((__p2).read()) as i32) & 15i32) as i16));
        if (!((((((data).wrapping_offset(5)).read()) as i32) & 7i32) != 0))
            && (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) != 72i32)
        {
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_sub(1));
            let __p4 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_sub(1));
            let __p5 = (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>();
            (__p5).write(((__p5).read()).wrapping_sub(1));
            let __p6 = (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>();
            (__p6).write(((__p6).read()).wrapping_sub(1));
        }
        'l1: {
            let __sw7 = ((((sprite).wrapping_add(43)).read()) as i32);
            if __sw7 == 0i32 {
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                break 'l1;
            }
            if __sw7 == 1i32 || __sw7 == 3i32 {
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write((-1i16));
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write((-1i16));
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                break 'l1;
            }
            if __sw7 == 2i32 {
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write((-1i16));
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write((-2i16));
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DuoFight_SlideGroudonDown(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut data: *mut i16 = ((sprite).wrapping_add(46)).cast::<i16>();
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) <= 160i32 {
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
            let __p2 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
            let __p3 = (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_add(8i32)) as i16));
            let __p4 = (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_add(8i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn DuoFight_CreateKyogreSprites() -> u8 {
    unsafe {
        let mut spriteId: u8 = 0u8;
        let mut data: *mut i16 = core::ptr::null_mut();
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_DuoFight_Kyogre)
                .cast::<u8>()
                .cast_mut(),
            126i16,
            96i16,
            1u8,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_DuoFight_Kyogre));
        data = ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>();
        (data).write(
            ((((CreateSprite(
                (&raw const sSpriteTemplate_DuoFight_Kyogre)
                    .cast::<u8>()
                    .cast_mut(),
                158i16,
                96i16,
                1u8,
            )) as i32)
                << 8) as i16),
        );
        (data).write(
            (((((data).read()) as i32)
                | ((CreateSprite(
                    (&raw const sSpriteTemplate_DuoFight_Kyogre)
                        .cast::<u8>()
                        .cast_mut(),
                    126i16,
                    112i16,
                    1u8,
                )) as i32)) as i16),
        );
        ((data).wrapping_offset(1)).write(
            ((((CreateSprite(
                (&raw const sSpriteTemplate_DuoFight_Kyogre)
                    .cast::<u8>()
                    .cast_mut(),
                158i16,
                112i16,
                1u8,
            )) as i32)
                << 8) as i16),
        );
        let __p1 = (data).wrapping_offset(1);
        (__p1).write(
            (((((__p1).read()) as i32)
                | ((CreateSprite(
                    (&raw const sSpriteTemplate_DuoFight_Kyogre)
                        .cast::<u8>()
                        .cast_mut(),
                    126i16,
                    128i16,
                    1u8,
                )) as i32)) as i16),
        );
        ((data).wrapping_offset(2)).write(
            ((((CreateSprite(
                (&raw const sSpriteTemplate_DuoFight_Kyogre)
                    .cast::<u8>()
                    .cast_mut(),
                158i16,
                128i16,
                1u8,
            )) as i32)
                << 8) as i16),
        );
        let __p2 = (data).wrapping_offset(2);
        (__p2).write(
            (((((__p2).read()) as i32)
                | ((CreateSprite(
                    (&raw const sSpriteTemplate_DuoFight_Kyogre)
                        .cast::<u8>()
                        .cast_mut(),
                    94i16,
                    128i16,
                    2u8,
                )) as i32)) as i16),
        );
        ((data).wrapping_offset(3)).write(
            ((((CreateSprite(
                (&raw const sSpriteTemplate_DuoFight_Kyogre)
                    .cast::<u8>()
                    .cast_mut(),
                126i16,
                128i16,
                2u8,
            )) as i32)
                << 8) as i16),
        );
        let __p3 = (data).wrapping_offset(3);
        (__p3).write(
            (((((__p3).read()) as i32)
                | ((CreateSprite(
                    (&raw const sSpriteTemplate_DuoFight_Kyogre)
                        .cast::<u8>()
                        .cast_mut(),
                    174i16,
                    128i16,
                    0u8,
                )) as i32)) as i16),
        );
        ((data).wrapping_offset(4)).write(
            ((((CreateSprite(
                (&raw const sSpriteTemplate_DuoFight_KyogrePectoralFin)
                    .cast::<u8>()
                    .cast_mut(),
                198i16,
                132i16,
                0u8,
            )) as i32)
                << 8) as i16),
        );
        let __p4 = (data).wrapping_offset(4);
        (__p4).write(
            (((((__p4).read()) as i32)
                | ((CreateSprite(
                    (&raw const sSpriteTemplate_DuoFight_KyogreDorsalFin)
                        .cast::<u8>()
                        .cast_mut(),
                    190i16,
                    120i16,
                    1u8,
                )) as i32)) as i16),
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).read()) as i32) >> 8) as isize * 68),
            1u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((data).read()) as i32) & 255i32) as isize * 68),
            2u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(1)).read()) as i32) >> 8) as isize * 68,
            ),
            3u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(1)).read()) as i32) & 255i32) as isize * 68,
            ),
            4u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(2)).read()) as i32) >> 8) as isize * 68,
            ),
            5u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(2)).read()) as i32) & 255i32) as isize * 68,
            ),
            6u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(3)).read()) as i32) >> 8) as isize * 68,
            ),
            7u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(3)).read()) as i32) & 255i32) as isize * 68,
            ),
            8u8,
        );
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DuoFight_Kyogre(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut data: *mut i16 = ((sprite).wrapping_add(46)).cast::<i16>();
        let __p1 = (data).wrapping_offset(5);
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = (data).wrapping_offset(5);
        (__p2).write((((((__p2).read()) as i32) & 15i32) as i16));
        if (!((((((data).wrapping_offset(5)).read()) as i32) & 7i32) != 0))
            && (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) != 152i32)
        {
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            let __p4 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 8) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
            let __p5 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) & 255i32) as isize
                    * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p5).write(((__p5).read()).wrapping_add(1));
            let __p6 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(1)).read()) as i32) >> 8) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p6).write(((__p6).read()).wrapping_add(1));
            let __p7 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(1)).read()) as i32) & 255i32) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p7).write(((__p7).read()).wrapping_add(1));
            let __p8 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(2)).read()) as i32) >> 8) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p8).write(((__p8).read()).wrapping_add(1));
            let __p9 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(2)).read()) as i32) & 255i32) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p9).write(((__p9).read()).wrapping_add(1));
            let __p10 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(3)).read()) as i32) >> 8) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p10).write(((__p10).read()).wrapping_add(1));
            let __p11 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(3)).read()) as i32) & 255i32) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p11).write(((__p11).read()).wrapping_add(1));
            let __p12 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(4)).read()) as i32) >> 8) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p12).write(((__p12).read()).wrapping_add(1));
            let __p13 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(4)).read()) as i32) & 255i32) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>();
            (__p13).write(((__p13).read()).wrapping_add(1));
        }
        'l1: {
            let __sw14 = ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(2)).read()) as i32) & 255i32) as isize * 68,
            ))
            .wrapping_add(43))
            .read()) as i32);
            if __sw14 == 0i32 {
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).read()) as i32) >> 8) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).read()) as i32) & 255i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(1)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(1)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(2)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(2)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(3)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(3)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(4)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(4)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                break 'l1;
            }
            if __sw14 == 1i32 || __sw14 == 3i32 {
                ((sprite).wrapping_add(38).cast::<i16>()).write(1i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).read()) as i32) >> 8) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).read()) as i32) & 255i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(1)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(1)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(2)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(2)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(3)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(3)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(4)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(4)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(1i16);
                break 'l1;
            }
            if __sw14 == 2i32 {
                ((sprite).wrapping_add(38).cast::<i16>()).write(2i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).read()) as i32) >> 8) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(2i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((data).read()) as i32) & 255i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(2i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(1)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(2i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(1)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(2i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(2)).read()) as i32) >> 8) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(2i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((data).wrapping_offset(4)).read()) as i32) & 255i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(2i16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DuoFight_SlideKyogreDown(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut data: *mut i16 = ((sprite).wrapping_add(46)).cast::<i16>();
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) <= 160i32 {
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
            let __p2 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 8) as isize * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
            let __p3 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) & 255i32) as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_add(8i32)) as i16));
            let __p4 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(1)).read()) as i32) >> 8) as isize * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_add(8i32)) as i16));
            let __p5 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(1)).read()) as i32) & 255i32) as isize * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>();
            (__p5).write((((((__p5).read()) as i32).wrapping_add(8i32)) as i16));
            let __p6 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(2)).read()) as i32) >> 8) as isize * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>();
            (__p6).write((((((__p6).read()) as i32).wrapping_add(8i32)) as i16));
            let __p7 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(2)).read()) as i32) & 255i32) as isize * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>();
            (__p7).write((((((__p7).read()) as i32).wrapping_add(8i32)) as i16));
            let __p8 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(3)).read()) as i32) >> 8) as isize * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>();
            (__p8).write((((((__p8).read()) as i32).wrapping_add(8i32)) as i16));
            let __p9 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(3)).read()) as i32) & 255i32) as isize * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>();
            (__p9).write((((((__p9).read()) as i32).wrapping_add(8i32)) as i16));
            let __p10 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(4)).read()) as i32) >> 8) as isize * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>();
            (__p10).write((((((__p10).read()) as i32).wrapping_add(8i32)) as i16));
            let __p11 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((data).wrapping_offset(4)).read()) as i32) & 255i32) as isize * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>();
            (__p11).write((((((__p11).read()) as i32).wrapping_add(8i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn InitTakesFlightSceneBgs() {
    unsafe {
        ResetVramOamAndBgCntRegs();
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            1u8,
            ((&raw const sBgTemplates_TakesFlight)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            ((crate::c::div_u32(12u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            0u8,
            (((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            1u8,
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(2048))
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            2u8,
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(4096))
            .cast::<u8>(),
        );
        ResetAllBgsCoordinates();
        ScheduleBgCopyTilemapToVram(0u8);
        ScheduleBgCopyTilemapToVram(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        SetGpuReg(0u8, 4160u16);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        SetGpuReg(80u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn LoadTakesFlightSceneGfx() {
    unsafe {
        ResetTempTileDataBuffers();
        DecompressAndCopyTileDataToVram(
            0u8,
            (((&raw mut gRaySceneDuoFight_Clouds_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        DecompressAndCopyTileDataToVram(
            1u8,
            (((&raw mut gRaySceneTakesFlight_Bg_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        DecompressAndCopyTileDataToVram(
            2u8,
            (((&raw mut gRaySceneTakesFlight_Rayquaza_Gfx).cast::<u32>()).cast::<u32>())
                .cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        'l1: loop {
            if !((FreeTempTileDataBuffersIfPossible()) != 0) {
                break 'l1;
            }
        }
        LZDecompressWram(
            ((&raw mut gRaySceneDuoFight_Clouds2_Tilemap).cast::<u32>()).cast::<u32>(),
            (((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .cast::<u8>(),
        );
        LZDecompressWram(
            ((&raw mut gRaySceneTakesFlight_Bg_Tilemap).cast::<u32>()).cast::<u32>(),
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(2048))
            .cast::<u8>(),
        );
        LZDecompressWram(
            ((&raw mut gRaySceneTakesFlight_Rayquaza_Tilemap).cast::<u32>()).cast::<u32>(),
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(4096))
            .cast::<u8>(),
        );
        LoadCompressedPalette(
            ((&raw mut gRaySceneTakesFlight_Rayquaza_Pal).cast::<u32>()).cast::<u32>(),
            0u16,
            64u16,
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_TakesFlight_Smoke)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpritePalette(
            (&raw const sSpritePal_TakesFlight_Smoke)
                .cast::<u8>()
                .cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_RayTakesFlightAnim(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        PlayNewMapMusic(464u16);
        InitTakesFlightSceneBgs();
        LoadTakesFlightSceneGfx();
        SetGpuReg(80u8, 592u16);
        SetGpuReg(82u8, 2056u16);
        BlendPalettes(4294967295u32, 16u8, 0u16);
        SetVBlankCallback(Some(VBlankCB_RayquazaScene));
        CreateTask(Some(Task_TakesFlight_CreateSmoke), 0u8);
        (data).write(0i16);
        ((data).wrapping_offset(1)).write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleRayTakesFlight));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleRayTakesFlight(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                if ((((data).wrapping_offset(1)).read()) as i32) == 8i32 {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                    ((data).wrapping_offset(2)).write(0i16);
                    ((data).wrapping_offset(3)).write(30i16);
                    ((data).wrapping_offset(4)).write(0i16);
                    ((data).wrapping_offset(5)).write(7i16);
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                } else {
                    let __p2 = (data).wrapping_offset(1);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = (data).wrapping_offset(2);
                (__p3).write(
                    (((((__p3).read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(3)).read()) as i32)))
                        as i16),
                );
                let __p4 = (data).wrapping_offset(4);
                (__p4).write(
                    (((((__p4).read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(5)).read()) as i32)))
                        as i16),
                );
                if ((((data).wrapping_offset(3)).read()) as i32) > 3i32 {
                    let __p5 = (data).wrapping_offset(3);
                    (__p5).write((((((__p5).read()) as i32).wrapping_sub(3i32)) as i16));
                }
                if ((((data).wrapping_offset(5)).read()) as i32) != 0i32 {
                    let __p6 = (data).wrapping_offset(5);
                    (__p6).write(((__p6).read()).wrapping_sub(1));
                }
                if ((((data).wrapping_offset(2)).read()) as i32) > 255i32 {
                    ((data).wrapping_offset(2)).write(256i16);
                    ((data).wrapping_offset(3)).write(0i16);
                    ((data).wrapping_offset(6)).write(12i16);
                    ((data).wrapping_offset(7)).write((-1i16));
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                SetBgAffine(
                    2u8,
                    30720i32,
                    6144i32,
                    120i16,
                    ((((((data).wrapping_offset(4)).read()) as i32).wrapping_add(32i32)) as i16),
                    ((data).wrapping_offset(2)).read(),
                    ((data).wrapping_offset(2)).read(),
                    0u16,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p7 = (data).wrapping_offset(1);
                (__p7).write(((__p7).read()).wrapping_add(1));
                SetBgAffine(
                    2u8,
                    30720i32,
                    6144i32,
                    120i16,
                    (((((((data).wrapping_offset(4)).read()) as i32).wrapping_add(32i32))
                        .wrapping_add((((((data).wrapping_offset(6)).read()) as i32) >> 2)))
                        as i16),
                    ((data).wrapping_offset(2)).read(),
                    ((data).wrapping_offset(2)).read(),
                    0u16,
                );
                let __p8 = (data).wrapping_offset(6);
                (__p8).write(
                    (((((__p8).read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(7)).read()) as i32)))
                        as i16),
                );
                if (((((data).wrapping_offset(6)).read()) as i32) == 12i32)
                    || (((((data).wrapping_offset(6)).read()) as i32) == (-12i32))
                {
                    let __p9 = (data).wrapping_offset(7);
                    (__p9).write((((((__p9).read()) as i32).wrapping_mul((-1i32))) as i16));
                    if ((((data).wrapping_offset(1)).read()) as i32) > 295i32 {
                        (data).write(((data).read()).wrapping_add(1));
                        BeginNormalPaletteFade(4294967295u32, 6i8, 0u8, 16u8, 0u16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p10 = (data).wrapping_offset(2);
                (__p10).write((((((__p10).read()) as i32).wrapping_add(16i32)) as i16));
                SetBgAffine(
                    2u8,
                    30720i32,
                    6144i32,
                    120i16,
                    ((((((data).wrapping_offset(4)).read()) as i32).wrapping_add(32i32)) as i16),
                    ((data).wrapping_offset(2)).read(),
                    ((data).wrapping_offset(2)).read(),
                    0u16,
                );
                Task_RayTakesFlightEnd(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_RayTakesFlightEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            SetVBlankCallback(None);
            ResetSpriteData();
            FreeAllSpritePalettes();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SetNextAnim));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TakesFlight_CreateSmoke(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((((data).wrapping_offset(1)).read()) as i32) & 3i32) == 0i32 {
            let mut spriteId: u8 = CreateSprite(
                (&raw const sSpriteTemplate_TakesFlight_Smoke)
                    .cast::<u8>()
                    .cast_mut(),
                ((((((((((&raw const sTakesFlight_SmokeCoords)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset((((data).read()) as i32) as isize * 2))
                .cast::<i8>())
                .read()) as i32)
                    .wrapping_mul(4i32))
                .wrapping_add(120i32)) as i16),
                (((((((((((&raw const sTakesFlight_SmokeCoords)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset((((data).read()) as i32) as isize * 2))
                .cast::<i8>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_mul(4i32))
                .wrapping_add(80i32)) as i16),
                0u8,
            );
            (((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .write(((((data).read()) as i8) as i16));
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                2,
                2,
                (1u32) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                0,
                2,
                (3u32) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                (2u16) as i32,
            );
            InitSpriteAffineAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            if (((data).read()) as i32) == 9i32 {
                DestroyTask(taskId);
                return;
            } else {
                (data).write(((data).read()).wrapping_add(1));
            }
        }
        let __p1 = (data).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TakesFlight_Smoke(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 0i32
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        } else {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    (((((((&raw const sTakesFlight_SmokeCoords)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 2,
                    ))
                    .cast::<i8>())
                    .read()) as i32),
                )) as i16),
            );
            let __p2 = (sprite).wrapping_add(38).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((((&raw const sTakesFlight_SmokeCoords)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 2,
                    ))
                    .cast::<i8>())
                    .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
            );
        }
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p3).write(((__p3).read()).wrapping_add(1));
        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p4).write((((((__p4).read()) as i32) & 15i32) as i16));
    }
}
pub(crate) unsafe extern "C" fn InitDescendsSceneBgs() {
    unsafe {
        ResetVramOamAndBgCntRegs();
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates_Descends).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            0u8,
            (((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            1u8,
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(2048))
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            2u8,
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(4096))
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            3u8,
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(6144))
            .cast::<u8>(),
        );
        ResetAllBgsCoordinates();
        ScheduleBgCopyTilemapToVram(0u8);
        ScheduleBgCopyTilemapToVram(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        ScheduleBgCopyTilemapToVram(3u8);
        SetGpuReg(0u8, 4160u16);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        ShowBg(3u8);
        SetGpuReg(80u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn LoadDescendsSceneGfx() {
    unsafe {
        ResetTempTileDataBuffers();
        DecompressAndCopyTileDataToVram(
            0u8,
            (((&raw mut gRaySceneDescends_Light_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        DecompressAndCopyTileDataToVram(
            1u8,
            (((&raw mut gRaySceneDescends_Bg_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        'l1: loop {
            if !((FreeTempTileDataBuffersIfPossible()) != 0) {
                break 'l1;
            }
        }
        LZDecompressWram(
            ((&raw mut gRaySceneDescends_Light_Tilemap).cast::<u32>()).cast::<u32>(),
            (((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .cast::<u8>(),
        );
        LZDecompressWram(
            ((&raw mut gRaySceneDescends_Bg_Tilemap).cast::<u32>()).cast::<u32>(),
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(6144))
            .cast::<u8>(),
        );
        {
            let mut tmp: u32 = 0u32;
            (&raw mut tmp).write_volatile(0u32);
            'l2: loop {
                'l3: {
                    CpuFastSet(
                        (&raw mut tmp).cast::<u8>(),
                        ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset(4096))
                        .cast::<u8>(),
                        ((16777216i32
                            | (crate::c::div_i32(2048i32, crate::c::div_i32(32i32, 8i32))
                                & 2097151i32)) as u32),
                    );
                }
                if !((0i32) != 0) {
                    break 'l2;
                }
            }
        }
        'l4: loop {
            'l5: {
                CpuFastSet(
                    ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .cast::<u8>())
                    .wrapping_offset(6144))
                    .cast::<u8>(),
                    ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .cast::<u8>())
                    .wrapping_offset(2048))
                    .cast::<u8>(),
                    ((crate::c::div_i32(2048i32, crate::c::div_i32(32i32, 8i32)) & 2097151i32)
                        as u32),
                );
            }
            if !((0i32) != 0) {
                break 'l4;
            }
        }
        {
            let mut tmp: u32 = 0u32;
            (&raw mut tmp).write_volatile(0u32);
            'l6: loop {
                'l7: {
                    CpuFastSet(
                        (&raw mut tmp).cast::<u8>(),
                        (((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset(2048))
                        .cast::<u8>())
                        .wrapping_offset(256),
                        ((16777216i32
                            | (crate::c::div_i32(832i32, crate::c::div_i32(32i32, 8i32))
                                & 2097151i32)) as u32),
                    );
                }
                if !((0i32) != 0) {
                    break 'l6;
                }
            }
        }
        LoadCompressedPalette(
            ((&raw mut gRaySceneDescends_Bg_Pal).cast::<u32>()).cast::<u32>(),
            0u16,
            64u16,
        );
        (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).write(32767u16);
        (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).write(32767u16);
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_Descends_Rayquaza)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_Descends_RayquazaTail)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpritePalette(
            (&raw const sSpritePal_Descends_Rayquaza)
                .cast::<u8>()
                .cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn HBlankCB_RayDescends() {
    unsafe {
        let mut vcount: u16 = GetGpuReg(6u8);
        if ((((vcount) as i32) >= 24i32) && (((vcount) as i32) <= 135i32))
            && (((vcount) as i32).wrapping_sub(24i32)
                <= ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8200)
                    .cast::<i16>())
                .read()) as i32))
        {
            crate::c::volatile_write(((67108946i32) as usize as *mut u16), 3336u16);
        } else {
            crate::c::volatile_write(((67108946i32) as usize as *mut u16), 4096u16);
        }
        if ((vcount) as i32) == 0i32 {
            if ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8200)
                .cast::<i16>())
            .read()) as i32)
                <= 8191i32
            {
                if ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8200)
                    .cast::<i16>())
                .read()) as i32)
                    <= 39i32
                {
                    let __p1 = (((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8200)
                        .cast::<i16>();
                    (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
                } else {
                    if ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8200)
                        .cast::<i16>())
                    .read()) as i32)
                        <= 79i32
                    {
                        let __p2 = (((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8200)
                            .cast::<i16>();
                        (__p2).write((((((__p2).read()) as i32).wrapping_add(2i32)) as i16));
                    } else {
                        let __p3 = (((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8200)
                            .cast::<i16>();
                        (__p3).write((((((__p3).read()) as i32).wrapping_add(1i32)) as i16));
                    }
                }
            }
            let __p4 = (((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8202)
                .cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_RayDescendsAnim(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        InitDescendsSceneBgs();
        LoadDescendsSceneGfx();
        SetGpuRegBits(80u8, 7745u16);
        SetGpuReg(82u8, 4096u16);
        BlendPalettes(4294967295u32, 16u8, 0u16);
        SetVBlankCallback(Some(VBlankCB_RayquazaScene));
        ((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8200)
            .cast::<i16>())
        .write(0i16);
        ((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8202)
            .cast::<i16>())
        .write(0i16);
        (data).write(0i16);
        ((data).wrapping_offset(1)).write(0i16);
        ((data).wrapping_offset(2)).write(0i16);
        ((data).wrapping_offset(3)).write(0i16);
        ((data).wrapping_offset(4)).write(4096i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleRayDescends));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleRayDescends(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                if ((((data).wrapping_offset(1)).read()) as i32) == 8i32 {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                } else {
                    let __p2 = (data).wrapping_offset(1);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    if ((((data).wrapping_offset(1)).read()) as i32) == 10i32 {
                        ((data).wrapping_offset(1)).write(0i16);
                        (data).write(((data).read()).wrapping_add(1));
                        SetHBlankCallback(Some(HBlankCB_RayDescends));
                        EnableInterrupts(3u16);
                    } else {
                        let __p3 = (data).wrapping_offset(1);
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((data).wrapping_offset(1)).read()) as i32) == 80i32 {
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                    CreateDescendsRayquazaSprite();
                } else {
                    let __p4 = (data).wrapping_offset(1);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (({
                    let __p5 = (data).wrapping_offset(1);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    == 368i32
                {
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_RayDescendsEnd));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_RayDescendsEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            SetVBlankCallback(None);
            SetHBlankCallback(None);
            ResetSpriteData();
            FreeAllSpritePalettes();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SetNextAnim));
        }
    }
}
pub(crate) unsafe extern "C" fn CreateDescendsRayquazaSprite() -> u8 {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_Descends_Rayquaza)
                .cast::<u8>()
                .cast_mut(),
            160i16,
            0i16,
            0u8,
        );
        let mut data: *mut i16 = ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>();
        (data).write(
            ((CreateSprite(
                (&raw const sSpriteTemplate_Descends_RayquazaTail)
                    .cast::<u8>()
                    .cast_mut(),
                184i16,
                (-48i16),
                0u8,
            )) as i16),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Descends_Rayquaza));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (3u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((data).read()) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (3u16) as i32,
        );
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Descends_Rayquaza(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut data: *mut i16 = ((sprite).wrapping_add(46)).cast::<i16>();
        let mut frame: i16 = ((data).wrapping_offset(2)).read();
        if ((frame) as i32) == 0i32 {
            ((data).wrapping_offset(3)).write(12i16);
            ((data).wrapping_offset(4)).write(8i16);
        } else {
            if ((frame) as i32) == 256i32 {
                ((data).wrapping_offset(3)).write(9i16);
                ((data).wrapping_offset(4)).write(7i16);
            } else {
                if ((frame) as i32) == 268i32 {
                    ((data).wrapping_offset(3)).write(8i16);
                    ((data).wrapping_offset(4)).write(6i16);
                } else {
                    if ((frame) as i32) == 280i32 {
                        ((data).wrapping_offset(3)).write(7i16);
                        ((data).wrapping_offset(4)).write(5i16);
                    } else {
                        if ((frame) as i32) == 292i32 {
                            ((data).wrapping_offset(3)).write(6i16);
                            ((data).wrapping_offset(4)).write(4i16);
                        } else {
                            if ((frame) as i32) == 304i32 {
                                ((data).wrapping_offset(3)).write(5i16);
                                ((data).wrapping_offset(4)).write(3i16);
                            } else {
                                if ((frame) as i32) == 320i32 {
                                    ((data).wrapping_offset(3)).write(4i16);
                                    ((data).wrapping_offset(4)).write(2i16);
                                }
                            }
                        }
                    }
                }
            }
        }
        if crate::c::rem_i32(
            ((((data).wrapping_offset(2)).read()) as i32),
            ((((data).wrapping_offset(3)).read()) as i32),
        ) == 0i32
        {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            let __p2 = (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((data).read()) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_sub(1));
        }
        if crate::c::rem_i32(
            ((((data).wrapping_offset(2)).read()) as i32),
            ((((data).wrapping_offset(4)).read()) as i32),
        ) == 0i32
        {
            let __p3 = (sprite).wrapping_add(38).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            let __p4 = (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((data).read()) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
        }
        let __p5 = (data).wrapping_offset(2);
        (__p5).write(((__p5).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn InitChargesSceneBgs() {
    unsafe {
        ResetVramOamAndBgCntRegs();
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates_Charges).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            0u8,
            (((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            1u8,
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(2048))
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            2u8,
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(4096))
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            3u8,
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(6144))
            .cast::<u8>(),
        );
        ResetAllBgsCoordinates();
        ScheduleBgCopyTilemapToVram(0u8);
        ScheduleBgCopyTilemapToVram(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        ScheduleBgCopyTilemapToVram(3u8);
        SetGpuReg(0u8, 12352u16);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        ShowBg(3u8);
        SetGpuReg(80u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn LoadChargesSceneGfx() {
    unsafe {
        ResetTempTileDataBuffers();
        DecompressAndCopyTileDataToVram(
            1u8,
            (((&raw mut gRaySceneCharges_Rayquaza_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        DecompressAndCopyTileDataToVram(
            2u8,
            (((&raw mut gRaySceneCharges_Streaks_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        DecompressAndCopyTileDataToVram(
            3u8,
            (((&raw mut gRaySceneCharges_Bg_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        'l1: loop {
            if !((FreeTempTileDataBuffersIfPossible()) != 0) {
                break 'l1;
            }
        }
        LZDecompressWram(
            ((&raw mut gRaySceneCharges_Orbs_Tilemap).cast::<u32>()).cast::<u32>(),
            (((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .cast::<u8>(),
        );
        LZDecompressWram(
            ((&raw mut gRaySceneCharges_Rayquaza_Tilemap).cast::<u32>()).cast::<u32>(),
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(2048))
            .cast::<u8>(),
        );
        LZDecompressWram(
            ((&raw mut gRaySceneCharges_Streaks_Tilemap).cast::<u32>()).cast::<u32>(),
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(4096))
            .cast::<u8>(),
        );
        LZDecompressWram(
            ((&raw mut gRaySceneCharges_Bg_Tilemap).cast::<u32>()).cast::<u32>(),
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(6144))
            .cast::<u8>(),
        );
        LoadCompressedPalette(
            ((&raw mut gRaySceneCharges_Bg_Pal).cast::<u32>()).cast::<u32>(),
            0u16,
            128u16,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_RayChargesAnim(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        InitChargesSceneBgs();
        LoadChargesSceneGfx();
        SetWindowsHideVertBorders();
        BlendPalettes(4294967295u32, 16u8, 0u16);
        SetVBlankCallback(Some(VBlankCB_RayquazaScene));
        (data).write(0i16);
        ((data).wrapping_offset(1)).write(0i16);
        ((data).wrapping_offset(2))
            .write(((CreateTask(Some(Task_RayCharges_ShakeRayquaza), 0u8)) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleRayCharges));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleRayCharges(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        RayCharges_AnimateBg();
        if (((((((data).wrapping_offset(3)).read()) as i32) & 7i32) == 0i32)
            && ((((data).read()) as i32) <= 1i32))
            && (((((data).wrapping_offset(1)).read()) as i32) <= 89i32)
        {
            PlaySE(103u16);
        }
        let __p1 = (data).wrapping_offset(3);
        (__p1).write(((__p1).read()).wrapping_add(1));
        'l1: {
            let __sw2 = (((data).read()) as i32);
            if __sw2 == 0i32 {
                if ((((data).wrapping_offset(1)).read()) as i32) == 8i32 {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                } else {
                    let __p3 = (data).wrapping_offset(1);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw2 == 1i32 {
                if ((((data).wrapping_offset(1)).read()) as i32) == 127i32 {
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                    ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_offset(2)).read()) as i32) as isize * 40,
                    ))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_RayCharges_FlyOffscreen));
                } else {
                    let __p4 = (data).wrapping_offset(1);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw2 == 2i32 {
                if ((((data).wrapping_offset(1)).read()) as i32) == 12i32 {
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                } else {
                    let __p5 = (data).wrapping_offset(1);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw2 == 3i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_RayChargesEnd));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_RayCharges_ShakeRayquaza(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((((data).wrapping_offset(15)).read()) as i32) & 3i32) == 0i32 {
            ChangeBgX(
                1u8,
                ((crate::c::rem_i32(((Random()) as i32), 8i32)).wrapping_sub(4i32) << 8),
                0u8,
            );
            ChangeBgY(
                1u8,
                ((crate::c::rem_i32(((Random()) as i32), 8i32)).wrapping_sub(4i32) << 8),
                0u8,
            );
        }
        let __p1 = (data).wrapping_offset(15);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Task_RayCharges_FlyOffscreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((data).read()) as i32) == 0i32 {
            ChangeBgX(1u8, 0i32, 0u8);
            ChangeBgY(1u8, 0i32, 0u8);
            (data).write(((data).read()).wrapping_add(1));
            ((data).wrapping_offset(1)).write(10i16);
            ((data).wrapping_offset(2)).write((-1i16));
        } else {
            if (((data).read()) as i32) == 1i32 {
                ChangeBgX(
                    1u8,
                    (((((data).wrapping_offset(1)).read()) as i32) << 8),
                    2u8,
                );
                ChangeBgY(
                    1u8,
                    (((((data).wrapping_offset(1)).read()) as i32) << 8),
                    1u8,
                );
                let __p1 = (data).wrapping_offset(1);
                (__p1).write(
                    (((((__p1).read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(2)).read()) as i32)))
                        as i16),
                );
                if ((((data).wrapping_offset(1)).read()) as i32) == (-10i32) {
                    let __p2 = (data).wrapping_offset(2);
                    (__p2).write((((((__p2).read()) as i32).wrapping_mul((-1i32))) as i16));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RayCharges_AnimateBg() {
    unsafe {
        ChangeBgX(2u8, 1024i32, 2u8);
        ChangeBgY(2u8, 1024i32, 1u8);
        ChangeBgX(0u8, 2048i32, 2u8);
        ChangeBgY(0u8, 2048i32, 1u8);
    }
}
pub(crate) unsafe extern "C" fn Task_RayChargesEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        RayCharges_AnimateBg();
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            SetVBlankCallback(None);
            ResetWindowDimensions();
            DestroyTask(((((data).wrapping_offset(2)).read()) as u8));
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SetNextAnim));
        }
    }
}
pub(crate) unsafe extern "C" fn InitChasesAwaySceneBgs() {
    unsafe {
        ResetVramOamAndBgCntRegs();
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            1u8,
            ((&raw const sBgTemplates_ChasesAway).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(12u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            0u8,
            (((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            1u8,
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(2048))
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            2u8,
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(4096))
            .cast::<u8>(),
        );
        ResetAllBgsCoordinates();
        ScheduleBgCopyTilemapToVram(0u8);
        ScheduleBgCopyTilemapToVram(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        SetGpuReg(0u8, 12352u16);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        SetGpuReg(80u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn LoadChasesAwaySceneGfx() {
    unsafe {
        ResetTempTileDataBuffers();
        DecompressAndCopyTileDataToVram(
            2u8,
            (((&raw mut gRaySceneChasesAway_Ring_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        DecompressAndCopyTileDataToVram(
            0u8,
            (((&raw mut gRaySceneChasesAway_Light_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        'l1: loop {
            if !((FreeTempTileDataBuffersIfPossible()) != 0) {
                break 'l1;
            }
        }
        LZDecompressWram(
            ((&raw mut gRaySceneChasesAway_Bg_Tilemap).cast::<u32>()).cast::<u32>(),
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(2048))
            .cast::<u8>(),
        );
        LZDecompressWram(
            ((&raw mut gRaySceneChasesAway_Light_Tilemap).cast::<u32>()).cast::<u32>(),
            (((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .cast::<u8>(),
        );
        LZDecompressWram(
            ((&raw mut gRaySceneChasesAway_Ring_Tilemap).cast::<u32>()).cast::<u32>(),
            ((((((&raw mut sRayScene).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>())
            .wrapping_offset(4096))
            .cast::<u8>(),
        );
        LoadCompressedPalette(
            ((&raw mut gRaySceneChasesAway_Bg_Pal).cast::<u32>()).cast::<u32>(),
            0u16,
            96u16,
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_ChasesAway_Groudon)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_ChasesAway_GroudonTail)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_ChasesAway_Kyogre)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_ChasesAway_Rayquaza)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_ChasesAway_RayquazaTail)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_ChasesAway_KyogreSplash)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpritePalette(
            (&raw const sSpritePal_ChasesAway_Groudon)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpritePalette(
            (&raw const sSpritePal_ChasesAway_Kyogre)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpritePalette(
            (&raw const sSpritePal_ChasesAway_Rayquaza)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpritePalette(
            (&raw const sSpritePal_ChasesAway_KyogreSplash)
                .cast::<u8>()
                .cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_RayChasesAwayAnim(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        InitChasesAwaySceneBgs();
        LoadChasesAwaySceneGfx();
        SetWindowsHideVertBorders();
        ClearGpuRegBits(0u8, 1024u16);
        SetGpuReg(80u8, 577u16);
        SetGpuReg(82u8, 3593u16);
        BlendPalettes(4294967295u32, 16u8, 0u16);
        SetVBlankCallback(Some(VBlankCB_RayquazaScene));
        (data).write(0i16);
        ((data).wrapping_offset(1)).write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleRayChasesAway));
        ((data).wrapping_offset(2))
            .write(((CreateTask(Some(Task_ChasesAway_AnimateBg), 0u8)) as i16));
        (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(1i16);
        ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((((data).wrapping_offset(2)).read()) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(1i16);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleRayChasesAway(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                if ((((data).wrapping_offset(1)).read()) as i32) == 8i32 {
                    ChasesAway_CreateTrioSprites(taskId);
                    BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                } else {
                    let __p2 = (data).wrapping_offset(1);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_offset(5)).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .read(),
                ) == (SpriteCB_ChasesAway_RayquazaFloat as *const () as usize)
                {
                    if ((((data).wrapping_offset(1)).read()) as i32) == 64i32 {
                        ChasesAway_KyogreStartLeave(taskId);
                        ChasesAway_GroudonStartLeave(taskId);
                        ((data).wrapping_offset(1)).write(0i16);
                        (data).write(((data).read()).wrapping_add(1));
                    } else {
                        let __p3 = (data).wrapping_offset(1);
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((data).wrapping_offset(1)).read()) as i32) == 448i32 {
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                } else {
                    let __p4 = (data).wrapping_offset(1);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    if crate::c::rem_i32(((((data).wrapping_offset(1)).read()) as i32), 144i32)
                        == 0i32
                    {
                        BlendPalettesGradually(65534u32, 0i8, 16u8, 0u8, 65535u16, 0u8, 0u8);
                        BlendPalettesGradually(4294901760u32, 0i8, 16u8, 0u8, 0u16, 0u8, 1u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                BeginNormalPaletteFade(4294967295u32, 4i8, 0u8, 16u8, 0u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_RayChasesAwayEnd));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ChasesAway_AnimateBg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((((data).read()) as i32) & 15i32) == 0i32 {
            SetGpuReg(
                82u8,
                ((((((((data).wrapping_offset(1)).read()) as i32).wrapping_add(14i32) << 8)
                    & 7936i32)
                    | (((((data).wrapping_offset(2)).read()) as i32).wrapping_add(9i32) & 15i32))
                    as u16),
            );
            let __p1 = (data).wrapping_offset(1);
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_sub(((((data).wrapping_offset(3)).read()) as i32)))
                    as i16),
            );
            let __p2 = (data).wrapping_offset(2);
            (__p2).write(
                (((((__p2).read()) as i32)
                    .wrapping_add(((((data).wrapping_offset(4)).read()) as i32)))
                    as i16),
            );
            if (((((data).wrapping_offset(1)).read()) as i32) == (-3i32))
                || (((((data).wrapping_offset(1)).read()) as i32) == 0i32)
            {
                let __p3 = (data).wrapping_offset(3);
                (__p3).write((((((__p3).read()) as i32).wrapping_mul((-1i32))) as i16));
            }
            if (((((data).wrapping_offset(2)).read()) as i32) == 3i32)
                || (((((data).wrapping_offset(2)).read()) as i32) == 0i32)
            {
                let __p4 = (data).wrapping_offset(4);
                (__p4).write((((((__p4).read()) as i32).wrapping_mul((-1i32))) as i16));
            }
        }
        (data).write(((data).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Task_RayChasesAwayEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            StopMapMusic();
            if ((((data).wrapping_offset(1)).read()) as i32) == 0i32 {
                SetVBlankCallback(None);
                ResetWindowDimensions();
                ResetSpriteData();
                FreeAllSpritePalettes();
                DestroyTask(((((data).wrapping_offset(2)).read()) as u8));
            }
            if ((((data).wrapping_offset(1)).read()) as i32) == 32i32 {
                ((data).wrapping_offset(1)).write(0i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_SetNextAnim));
            } else {
                let __p1 = (data).wrapping_offset(1);
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ChasesAway_CreateTrioSprites(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut taskData: *mut i16 = core::ptr::null_mut();
        let mut spriteData: *mut i16 = core::ptr::null_mut();
        taskData = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((taskData).wrapping_offset(3)).write(
            ((CreateSprite(
                (&raw const sSpriteTemplate_ChasesAway_Groudon)
                    .cast::<u8>()
                    .cast_mut(),
                64i16,
                120i16,
                0u8,
            )) as i16),
        );
        spriteData = ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(3)).read()) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>();
        (spriteData).write(
            ((CreateSprite(
                (&raw const sSpriteTemplate_ChasesAway_GroudonTail)
                    .cast::<u8>()
                    .cast_mut(),
                16i16,
                130i16,
                0u8,
            )) as i16),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((taskData).wrapping_offset(3)).read()) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((spriteData).read()) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        ((taskData).wrapping_offset(4)).write(
            ((CreateSprite(
                (&raw const sSpriteTemplate_ChasesAway_Kyogre)
                    .cast::<u8>()
                    .cast_mut(),
                160i16,
                128i16,
                1u8,
            )) as i16),
        );
        spriteData = ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(4)).read()) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>();
        (spriteData).write(
            ((CreateSprite(
                (&raw const sSpriteTemplate_ChasesAway_Kyogre)
                    .cast::<u8>()
                    .cast_mut(),
                192i16,
                128i16,
                1u8,
            )) as i16),
        );
        ((spriteData).wrapping_offset(1)).write(
            ((CreateSprite(
                (&raw const sSpriteTemplate_ChasesAway_Kyogre)
                    .cast::<u8>()
                    .cast_mut(),
                224i16,
                128i16,
                1u8,
            )) as i16),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((taskData).wrapping_offset(4)).read()) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((spriteData).read()) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((spriteData).wrapping_offset(1)).read()) as i32) as isize * 68,
            ))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((spriteData).read()) as i32) as isize * 68),
            1u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((spriteData).wrapping_offset(1)).read()) as i32) as isize * 68),
            2u8,
        );
        ((taskData).wrapping_offset(5)).write(
            ((CreateSprite(
                (&raw const sSpriteTemplate_ChasesAway_Rayquaza)
                    .cast::<u8>()
                    .cast_mut(),
                120i16,
                (-65i16),
                0u8,
            )) as i16),
        );
        spriteData = ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(5)).read()) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>();
        (spriteData).write(
            ((CreateSprite(
                (&raw const sSpriteTemplate_ChasesAway_RayquazaTail)
                    .cast::<u8>()
                    .cast_mut(),
                120i16,
                (-113i16),
                0u8,
            )) as i16),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((taskData).wrapping_offset(5)).read()) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((spriteData).read()) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn ChasesAway_PushDuoBack(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut taskData: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(3)).read()) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_ChasesAway_DuoRingPush));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(3)).read()) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(3)).read()) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(3)).read()) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(4i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(3)).read()) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(0i16);
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(4)).read()) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_ChasesAway_DuoRingPush));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(4)).read()) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(4)).read()) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(4)).read()) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(4i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(4)).read()) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(1i16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ChasesAway_DuoRingPush(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            & 7i32)
            == 0i32
        {
            if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) != 0) {
                let __p1 = (sprite).wrapping_add(32).cast::<i16>();
                (__p1).write(
                    (((((__p1).read()) as i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                    )) as i16),
                );
                let __p2 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>();
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                    )) as i16),
                );
            } else {
                let __p3 = (sprite).wrapping_add(32).cast::<i16>();
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                    )) as i16),
                );
                let __p4 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>();
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                    )) as i16),
                );
                let __p5 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>();
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                    )) as i16),
                );
            }
            let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p6).write(((__p6).read()).wrapping_add(1));
            let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p7).write(
                (((((__p7).read()) as i32).wrapping_sub(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32),
                )) as i16),
            );
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                == 3i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
                return;
            }
        }
        let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p8).write(((__p8).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn ChasesAway_GroudonStartLeave(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut taskData: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(3)).read()) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_ChasesAway_GroudonLeave));
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((taskData).wrapping_offset(3)).read()) as i32) as isize * 68),
            1u8,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ChasesAway_GroudonLeave(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = ((((sprite).wrapping_add(43)).read()) as i32);
            if __sw1 == 0i32 || __sw1 == 2i32 {
                if crate::c::rem_i32(
                    ((crate::c::bf_read((sprite).wrapping_add(44), 0, 6, false) as u8) as i32),
                    12i32,
                ) == 0i32
                {
                    let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p2).write((((((__p2).read()) as i32).wrapping_sub(2i32)) as i16));
                    let __p3 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(32)
                    .cast::<i16>();
                    (__p3).write((((((__p3).read()) as i32).wrapping_sub(2i32)) as i16));
                }
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 3i32 {
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write((-2i16));
                if (((crate::c::bf_read((sprite).wrapping_add(44), 0, 6, false) as u8) as i32)
                    & 15i32)
                    == 0i32
                {
                    let __p4 = (sprite).wrapping_add(34).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    let __p5 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(34)
                    .cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ChasesAway_KyogreStartLeave(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut taskData: *mut i16 = core::ptr::null_mut();
        let mut spriteData: *mut i16 = core::ptr::null_mut();
        taskData = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        spriteData = ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(4)).read()) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>();
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((taskData).wrapping_offset(4)).read()) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_ChasesAway_KyogreLeave));
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteData).read()) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_ChasesAway_KyogreLeave));
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((spriteData).wrapping_offset(1)).read()) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_ChasesAway_KyogreLeave));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ChasesAway_KyogreLeave(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            & 3i32)
            == 0i32
        {
            if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) == 1i32 {
                ((sprite).wrapping_add(36).cast::<i16>()).write((-1i16));
            } else {
                ((sprite).wrapping_add(36).cast::<i16>()).write(1i16);
            }
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            == 128i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                ((CreateSprite(
                    (&raw const sSpriteTemplate_ChasesAway_KyogreSplash)
                        .cast::<u8>()
                        .cast_mut(),
                    152i16,
                    132i16,
                    0u8,
                )) as i16),
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(5),
                2,
                2,
                (1u16) as i32,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                ((CreateSprite(
                    (&raw const sSpriteTemplate_ChasesAway_KyogreSplash)
                        .cast::<u8>()
                        .cast_mut(),
                    224i16,
                    132i16,
                    0u8,
                )) as i16),
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(5),
                2,
                2,
                (1u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(63),
                0,
                1,
                (1u16) as i32,
            );
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            > 127i32
        {
            if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) != 32i32 {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        >> 4) as i16),
                );
            }
        } else {
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if crate::c::rem_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            64i32,
        ) == 0i32
        {
            PlaySE(165u16);
        }
        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p4).write(((__p4).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ChasesAway_Rayquaza(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut frame: i16 =
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read();
        if ((frame) as i32) <= 64i32 {
            let __p1 = (sprite).wrapping_add(38).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
            let __p2 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(2i32)) as i16));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                == 64i32
            {
                ChasesAway_SetRayquazaAnim(sprite, 1u8, 0i16, (-48i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(5i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write((-1i16));
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(3i16);
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(5i16);
            }
        } else {
            if ((frame) as i32) <= 111i32 {
                SpriteCB_ChasesAway_RayquazaFloat(sprite);
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    == 0i32
                {
                    PlaySE(104u16);
                }
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    == (-3i32)
                {
                    ChasesAway_SetRayquazaAnim(sprite, 2u8, 48i16, 16i16);
                }
            } else {
                if ((frame) as i32) == 112i32 {
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(7i16);
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(3i16);
                    SpriteCB_ChasesAway_RayquazaFloat(sprite);
                } else {
                    if ((frame) as i32) <= 327i32 {
                        SpriteCB_ChasesAway_RayquazaFloat(sprite);
                    } else {
                        if ((frame) as i32) == 328i32 {
                            SpriteCB_ChasesAway_RayquazaFloat(sprite);
                            ChasesAway_SetRayquazaAnim(sprite, 3u8, 48i16, 16i16);
                            ((sprite).wrapping_add(36).cast::<i16>()).write(1i16);
                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
                                    as isize
                                    * 68,
                            ))
                            .wrapping_add(36)
                            .cast::<i16>())
                            .write(1i16);
                            PlayCry_Normal(406u16, 0i8);
                            CreateTask(Some(Task_ChasesAway_AnimateRing), 0u8);
                        } else {
                            'l1: {
                                let __sw3 = ((frame) as i32);
                                if __sw3 == 376i32 {
                                    ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        (((((sprite).wrapping_add(46)).cast::<i16>()).read())
                                            as i32)
                                            as isize
                                            * 68,
                                    ))
                                    .wrapping_add(36)
                                    .cast::<i16>())
                                    .write(0i16);
                                    SpriteCB_ChasesAway_RayquazaFloat(sprite);
                                    ChasesAway_SetRayquazaAnim(sprite, 2u8, 48i16, 16i16);
                                    ((sprite)
                                        .wrapping_add(28)
                                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                    .write(Some(SpriteCB_ChasesAway_RayquazaFloat));
                                    return;
                                }
                                if __sw3 == 352i32 {
                                    ChasesAway_PushDuoBack(FindTaskIdByFunc(Some(
                                        Task_HandleRayChasesAway,
                                    )));
                                    break 'l1;
                                }
                            }
                        }
                    }
                }
            }
        }
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 328i32)
            && ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                as i32)
                & 1i32)
                == 0i32)
        {
            let __p4 = (sprite).wrapping_add(36).cast::<i16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_mul((-1i32))) as i16));
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .write(((sprite).wrapping_add(36).cast::<i16>()).read());
        }
        let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p5).write(((__p5).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ChasesAway_RayquazaFloat(body: *mut u8) {
    unsafe {
        let mut body = body;
        let mut tail: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((body).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
        );
        if !((((((((body).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
            & ((((((tail).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32))
            != 0)
        {
            let __p1 = (body).wrapping_add(38).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((body).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32),
                )) as i16),
            );
            let __p2 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((body).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((body).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32),
                )) as i16),
            );
            let __p3 = (((body).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((body).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32),
                )) as i16),
            );
            if (((((((body).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                >= ((((((tail).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32))
                || (((((((body).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    <= ((((((tail).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        .wrapping_neg())
            {
                if ((((((body).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    > ((((((tail).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                {
                    ((((body).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                        ((((tail).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                    );
                } else {
                    if ((((((body).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        < ((((((tail).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32)
                            .wrapping_neg()
                    {
                        ((((body).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                            ((((((((tail).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .read()) as i32)
                                .wrapping_neg()) as i16),
                        );
                    }
                }
                let __p4 = (((body).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p4).write((((((__p4).read()) as i32).wrapping_mul((-1i32))) as i16));
            }
        }
        let __p5 = (((body).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
        (__p5).write(((__p5).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn ChasesAway_SetRayquazaAnim(
    body: *mut u8,
    animNum: u8,
    x: i16,
    y: i16,
) {
    unsafe {
        let mut body = body;
        let mut animNum = animNum;
        let mut x = x;
        let mut y = y;
        let mut tail: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((body).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
        );
        ((tail).wrapping_add(32).cast::<i16>()).write(
            ((((((body).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_add(((x) as i32)))
                as i16),
        );
        ((tail).wrapping_add(34).cast::<i16>()).write(
            ((((((body).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(((y) as i32)))
                as i16),
        );
        ((tail).wrapping_add(36).cast::<i16>())
            .write(((body).wrapping_add(36).cast::<i16>()).read());
        ((tail).wrapping_add(38).cast::<i16>())
            .write(((body).wrapping_add(38).cast::<i16>()).read());
        StartSpriteAnim(body, animNum);
        StartSpriteAnim(tail, animNum);
    }
}
pub(crate) unsafe extern "C" fn Task_ChasesAway_AnimateRing(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                SetBgAffine(2u8, 16384i32, 16384i32, 120i16, 64i16, 256i16, 256i16, 0u16);
                SetGpuRegBits(0u8, 1024u16);
                ((data).wrapping_offset(4)).write(16i16);
                (data).write(((data).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((data).wrapping_offset(5)).read()) as i32) == 8i32 {
                    PlaySE(18u16);
                }
                if ((((data).wrapping_offset(2)).read()) as i32) == 2i32 {
                    (data).write(((data).read()).wrapping_add(1));
                } else {
                    let __p2 = (data).wrapping_offset(1);
                    (__p2).write(
                        (((((__p2).read()) as i32)
                            .wrapping_add(((((data).wrapping_offset(4)).read()) as i32)))
                            as i16),
                    );
                    let __p3 = (data).wrapping_offset(5);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    if (crate::c::rem_i32(((((data).wrapping_offset(3)).read()) as i32), 3i32)
                        == 0i32)
                        && (((((data).wrapping_offset(4)).read()) as i32) != 4i32)
                    {
                        let __p4 = (data).wrapping_offset(4);
                        (__p4).write((((((__p4).read()) as i32).wrapping_sub(2i32)) as i16));
                    }
                    let __p5 = (data).wrapping_offset(3);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    SetBgAffine(
                        2u8,
                        16384i32,
                        16384i32,
                        120i16,
                        64i16,
                        (((256i32).wrapping_sub(((((data).wrapping_offset(1)).read()) as i32)))
                            as i16),
                        (((256i32).wrapping_sub(((((data).wrapping_offset(1)).read()) as i32)))
                            as i16),
                        0u16,
                    );
                    if ((((data).wrapping_offset(1)).read()) as i32) > 255i32 {
                        ((data).wrapping_offset(1)).write(0i16);
                        ((data).wrapping_offset(3)).write(0i16);
                        ((data).wrapping_offset(5)).write(0i16);
                        ((data).wrapping_offset(4)).write(16i16);
                        let __p6 = (data).wrapping_offset(2);
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ClearGpuRegBits(0u8, 1024u16);
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
