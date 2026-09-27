//! Translated from `src/battle_anim_sound_tasks.c` by tools/rustport/c2rs.py, then reviewed.
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

unsafe extern "C" {
    static mut gAnimBattlerSpecies: u8;
    static mut gAnimCustomPanning: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gContestResources: u8;
    static mut gEnemyParty: u8;
    static mut gPlayerParty: u8;
    static mut gTasks: u8;
    fn BattleAnimAdjustPanning(a0: i8) -> i8;
    fn CalculatePanIncrement(a0: i16, a1: i16, a2: i16) -> i16;
    fn DestroyAnimSoundTask(a0: u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn IsCryPlaying() -> u8;
    fn KeepPanInRange(a0: i16, a1: i32) -> i16;
    fn PlayCry_ByMode(a0: u16, a1: i8, a2: u8);
    fn PlayCry_DuckNoRestore(a0: u16, a1: i8, a2: u8);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn PlaySE1WithPanning(a0: u16, a1: i8);
    fn PlaySE2WithPanning(a0: u16, a1: i8);
    fn StopCryAndClearCrySongs();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundTask_FireBlast(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut pan1: i8 = 0i8;
        let mut pan2: i8 = 0i8;
        let mut panIncrement: i8 = 0i8;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        pan1 = BattleAnimAdjustPanning((-64i8));
        pan2 = BattleAnimAdjustPanning(63i8);
        panIncrement = ((CalculatePanIncrement(((pan1) as i16), ((pan2) as i16), 2i16)) as i8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((pan1) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((pan2) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((panIncrement) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(10i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(SoundTask_FireBlast_Step1));
    }
}
pub(crate) unsafe extern "C" fn SoundTask_FireBlast_Step1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut pan: i16 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read();
        let mut panIncrement: i8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as i8);
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 111i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(5i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(SoundTask_FireBlast_Step2));
        } else {
            if (({
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10);
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                == 11i32
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(0i16);
                PlaySE12WithPanning(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as u16),
                    ((pan) as i8),
                );
            }
            pan = ((((pan) as i32).wrapping_add(((panIncrement) as i32))) as i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(KeepPanInRange(pan, ((panIncrement) as i32)));
        }
    }
}
pub(crate) unsafe extern "C" fn SoundTask_FireBlast_Step2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 6i32
        {
            let mut pan: i8 = 0i8;
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(0i16);
            pan = BattleAnimAdjustPanning(63i8);
            PlaySE12WithPanning(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u16),
                pan,
            );
            if (({
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11);
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                == 2i32
            {
                DestroyAnimSoundTask(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundTask_LoopSEAdjustPanning(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut songId: u16 =
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u16);
        let mut targetPan: i8 = ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
            .wrapping_offset(2))
        .read()) as i8);
        let mut panIncrement: i8 = ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
            .wrapping_offset(3))
        .read()) as i8);
        let mut r10: u8 = ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
            .wrapping_offset(4))
        .read()) as u8);
        let mut r7: u8 = ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
            .wrapping_offset(5))
        .read()) as u8);
        let mut r9: u8 = ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
            .wrapping_offset(6))
        .read()) as u8);
        let mut sourcePan: i8 = BattleAnimAdjustPanning(
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as i8),
        );
        targetPan = BattleAnimAdjustPanning(targetPan);
        panIncrement = ((CalculatePanIncrement(
            ((sourcePan) as i16),
            ((targetPan) as i16),
            ((panIncrement) as i16),
        )) as i8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((songId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((sourcePan) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((targetPan) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((panIncrement) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((r10) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(((r7) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((r9) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11))
        .write(((sourcePan) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(12))
        .write(((r9) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(SoundTask_LoopSEAdjustPanning_Step));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read())
        .unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn SoundTask_LoopSEAdjustPanning_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            == ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as i32)
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12))
            .write(0i16);
            PlaySE12WithPanning(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u16),
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .read()) as i8),
            );
            if (({
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4);
                let __t4 = ((__p3).read()).wrapping_sub(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                == 0i32
            {
                DestroyAnimSoundTask(taskId);
                return;
            }
        }
        if (({
            let __p5 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10);
            let __t6 = (__p5).read();
            (__p5).write(((__p5).read()).wrapping_add(1));
            __t6
        }) as i32)
            == ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as i32)
        {
            let mut dPan: u16 = 0u16;
            let mut oldPan: u16 = 0u16;
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(0i16);
            dPan = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as u16);
            oldPan = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .read()) as u16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(((((dPan) as i32).wrapping_add(((oldPan) as i32))) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(KeepPanInRange(
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .read(),
                ((oldPan) as i32),
            ));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundTask_PlayCryHighPitch(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut species: u16 = 0u16;
        let mut pan: i8 = BattleAnimAdjustPanning((-64i8));
        if (IsContest()) != 0 {
            if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32
            {
                species = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read())
                .cast::<u16>())
                .read();
            } else {
                DestroyAnimVisualTask(taskId);
            }
        } else {
            let mut battler: u8 = 0u8;
            if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32
            {
                battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
            } else {
                if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    == 1i32
                {
                    battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
                } else {
                    if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                        == 2i32
                    {
                        battler = ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                            ^ 2i32) as u8);
                    } else {
                        battler = ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
                            ^ 2i32) as u8);
                    }
                }
            }
            if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                == 1i32)
                || ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    == 3i32))
                && (!((IsBattlerSpriteVisible(battler)) != 0))
            {
                DestroyAnimVisualTask(taskId);
                return;
            }
            if ((GetBattlerSide(battler)) as i32) != 0i32 {
                species = ((GetMonData2(
                    ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    11i32,
                )) as u16);
            } else {
                species = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    11i32,
                )) as u16);
            }
        }
        if ((species) as i32) != 0i32 {
            PlayCry_ByMode(species, pan, 3u8);
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundTask_PlayDoubleCry(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut species: u16 = 0u16;
        let mut pan: i8 = BattleAnimAdjustPanning((-64i8));
        if (IsContest()) != 0 {
            if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32
            {
                species = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<*mut u8>())
                .read())
                .cast::<u16>())
                .read();
            } else {
                DestroyAnimVisualTask(taskId);
            }
        } else {
            let mut battler: u8 = 0u8;
            if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32
            {
                battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
            } else {
                if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    == 1i32
                {
                    battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
                } else {
                    if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                        == 2i32
                    {
                        battler = ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                            ^ 2i32) as u8);
                    } else {
                        battler = ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
                            ^ 2i32) as u8);
                    }
                }
            }
            if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                == 1i32)
                || ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    == 3i32))
                && (!((IsBattlerSpriteVisible(battler)) != 0))
            {
                DestroyAnimVisualTask(taskId);
                return;
            }
            if ((GetBattlerSide(battler)) as i32) != 0i32 {
                species = ((GetMonData2(
                    ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    11i32,
                )) as u16);
            } else {
                species = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((battler) as i32) as isize))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    11i32,
                )) as u16);
            }
        }
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((species) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((pan) as i16));
        if ((species) as i32) != 0i32 {
            if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as i32)
                == 255i32
            {
                PlayCry_ByMode(species, pan, 9u8);
            } else {
                PlayCry_ByMode(species, pan, 7u8);
            }
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(SoundTask_PlayDoubleCry_Step));
        } else {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn SoundTask_PlayDoubleCry_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut species: u16 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u16);
        let mut pan: i8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i8);
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .read()) as i32)
            < 2i32
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(9);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 255i32
            {
                if !((IsCryPlaying()) != 0) {
                    PlayCry_ByMode(species, pan, 10u8);
                    DestroyAnimVisualTask(taskId);
                }
            } else {
                if !((IsCryPlaying()) != 0) {
                    PlayCry_ByMode(species, pan, 8u8);
                    DestroyAnimVisualTask(taskId);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundTask_WaitForCry(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .read()) as i32)
            < 2i32
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(9);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if !((IsCryPlaying()) != 0) {
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundTask_PlayCryWithEcho(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut species: u16 = 0u16;
        let mut pan: i8 = 0i8;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        pan = BattleAnimAdjustPanning((-64i8));
        if (IsContest()) != 0 {
            species = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<*mut u8>())
            .read())
            .cast::<u16>())
            .read();
        } else {
            species = ((((&raw mut gAnimBattlerSpecies).cast::<u16>()).cast::<u16>())
                .wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
            .read();
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((species) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((pan) as i16));
        if ((species) as i32) != 0i32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(SoundTask_PlayCryWithEcho_Step));
        } else {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn SoundTask_PlayCryWithEcho_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut species: u16 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u16);
        let mut pan: i8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i8);
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(9))
            .read()) as i32);
            let __matched = __sw1 == 2i32
                || __sw1 == 1i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 0i32;
            let mut __fall = false;
            if __sw1 == 2i32 {
                __fall = true;
                PlayCry_DuckNoRestore(species, pan, 4u8);
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 3i32 || __sw1 == 4i32 {
                __fall = true;
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                if (IsCryPlaying()) != 0 {
                    break 'l1;
                }
            }
            if __fall || __sw1 == 0i32 {
                __fall = true;
                StopCryAndClearCrySongs();
                let __p4 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                __fall = true;
                if !((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read())
                    != 0)
                {
                    PlayCry_DuckNoRestore(species, pan, 6u8);
                } else {
                    PlayCry_ByMode(species, pan, 6u8);
                }
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundTask_PlaySE1WithPanning(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut songId: u16 =
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u16);
        let mut pan: i8 = BattleAnimAdjustPanning(
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as i8),
        );
        PlaySE1WithPanning(songId, pan);
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundTask_PlaySE2WithPanning(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut songId: u16 =
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u16);
        let mut pan: i8 = BattleAnimAdjustPanning(
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as i8),
        );
        PlaySE2WithPanning(songId, pan);
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoundTask_AdjustPanningVar(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut targetPan: i8 = ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
            .wrapping_offset(1))
        .read()) as i8);
        let mut panIncrement: i8 = ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
            .wrapping_offset(2))
        .read()) as i8);
        let mut r9: u16 = ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
            .wrapping_offset(3))
        .read()) as u16);
        let mut sourcePan: i8 = BattleAnimAdjustPanning(
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i8),
        );
        targetPan = BattleAnimAdjustPanning(targetPan);
        panIncrement = ((CalculatePanIncrement(
            ((sourcePan) as i16),
            ((targetPan) as i16),
            ((panIncrement) as i16),
        )) as i8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((sourcePan) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((targetPan) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((panIncrement) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(((r9) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11))
        .write(((sourcePan) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(SoundTask_AdjustPanningVar_Step));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read())
        .unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn SoundTask_AdjustPanningVar_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut panIncrement: u16 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as u16);
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            == ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as i32)
        {
            let mut oldPan: u16 = 0u16;
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(0i16);
            oldPan = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .read()) as u16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(((((panIncrement) as i32).wrapping_add(((oldPan) as i32))) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(KeepPanInRange(
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .read(),
                ((oldPan) as i32),
            ));
        }
        ((&raw mut gAnimCustomPanning).cast::<u8>()).write(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .read()) as u8),
        );
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11))
        .read()) as i32)
            == ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
